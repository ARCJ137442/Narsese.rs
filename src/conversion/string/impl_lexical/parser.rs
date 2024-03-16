//! 实现/词法解析器
//! * 🎯字符串→词法Narsese

use util::{first, PrefixMatch};

use super::NarseseFormat;
use crate::{
    lexical::{Narsese, Sentence, Task, Term},
    util::{BufferIterator, IntoChars},
};
use std::{error::Error, fmt::Display};

// * 📌现在不再使用类似「NarseseResult」的「解析结果」类型
//   * 直接使用[`LexicalNarsese`]作为「词项/语句/任务」的枚举

/// 用于表征「解析结果」
/// * 用于表示「解析对象」
///
/// ! 📝原先基于「返回『(解析出的对象, 下一起始索引)』」的方法已无需使用
/// * 现在是基于「解析器状态」的「状态机模型」
///   * 📌关键差异：附带可设置的「中间解析结果」与「可变索引」
///   * 🚩子解析函数在解析之后，直接填充「中间解析结果」并修改「可变索引」
type ParseResult<T = Narsese> = Result<T, ParseError>;
/// 用于表征「令牌消耗结果」
/// * 🎯用于在出错时传播错误
type ConsumeResult = ParseResult<()>;

/// 用于表征「解析错误」
/// * 📝不要依赖于任何外部引用：后续需要【脱离】解析环境
/// * 🚩在使用「缓冲区迭代器」的「词法解析器」中，只**显示缓冲区**而不进行回溯
/// * 📌一般在「解析错误」时，迭代器已经无需使用了
#[derive(Debug, Clone)]
pub struct ParseError {
    /// 错误消息 | 一般不含冒号
    /// * 🎯用于描述出错原因
    message: String,
    /// 裁剪出的「解析环境」切片
    /// * 🎯用于展示出错范围
    context: String,
    /// 出错所在的「解析索引」
    /// * 🎯用于指示出错位置
    index: usize,
}
impl ParseError {
    /// 构造函数
    /// * 🚩不同于先前解析器，此处不再自动计算上下文
    pub fn new(message: &str, context: String, index: usize) -> ParseError {
        ParseError {
            message: message.to_string(),
            context,
            // env_slice: ParseError::generate_env_slice(env, index),
            index,
        }
    }
}
/// 呈现报错文本
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // 输出
        write!(
            f,
            "Narsese解析错误：{} @ {} & {}",
            self.message, self.index, self.context
        )
    }
}
impl Error for ParseError {}

/// 词法Narsese的「中间结果」
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MidParseResult {
    /// 解析出的词项
    pub term: Option<Term>,
    /// 解析出的标点（字符串）
    pub punctuation: Option<String>,
    /// 解析出的时间戳（字符串）
    pub stamp: Option<String>,
    /// 解析出的真值（字符串）
    pub truth: Option<String>,
    /// 解析出的预算值（字符串）
    pub budget: Option<String>,
}

/// 词法Narsese的「解析状态」
/// * 其中的`C`一般为「字符」
/// * 🚩不再设置泛型参数`T`：默认就是字符串[`String`]
pub struct ParseState<'a, C = char> {
    /// 引用的「解析格式」
    format: &'a NarseseFormat<'a>,
    /// 内置的「缓冲迭代器」
    /// * 🚩使用[`Box`]封装原始迭代器
    iter: BufferIterator<C, Box<dyn Iterator<Item = C> + 'a>>,
    /// 解析出来的中间结果
    mid_result: MidParseResult,
}

/// 通用实现
impl<'a, Item> ParseState<'a, Item> {
    /// 构造函数
    /// * 🚩传入迭代器进行构造
    pub fn new(format: &'a NarseseFormat, iter: impl Iterator<Item = Item> + 'a) -> Self {
        Self {
            format,
            iter: BufferIterator::new(Box::new(iter)),
            mid_result: MidParseResult::default(), // 全`None`
        }
    }

    /// 快捷构造解析结果/Ok
    pub fn ok<T>(value: T) -> ParseResult<T> {
        ParseResult::Ok(value)
    }

    // ! ❌【2024-03-15 23:25:27】暂时没法解决「借用内部option的同时借用self」的问题
    // /// 尝试向中间结果插入元素
    // /// * 🎯简化「若有⇒返回错误，若无⇒成功插入」的逻辑
    // pub fn try_insert_result<T>(option: &mut Option<T>, value: T) -> ConsumeResult {
    //     match option {
    //         // 若无，则插入
    //         None => {
    //             *option = Some(value);
    //             ParseState::ok(())
    //         }
    //         // 若有，则返回错误
    //         Some(_) => ParseState::err(&format!("重复插入元素：{:#?}", value)),
    //     }
    // }
}

/// 字符实现
/// * 🚩解析逻辑正式开始
impl<'a> ParseState<'a, char> {
    /// 快速构造解析结果/Err
    pub fn err<T>(&self, message: &str) -> ParseResult<T> {
        Err(ParseError::new(
            // 传入的错误消息
            message,
            // 自身缓冲区内容
            self.iter.buffer_iter().copied().collect(),
            // 自身缓冲区头索引（相对滞后）
            self.iter.buffer_head(),
        ))
    }

    // 实用工具函数 //

    /// 工具/前缀匹配 & 偏移前移
    /// * 🎯在「匹配前缀」的同时不消耗缓冲区
    ///   * 📌没有解析出来时，都不要动缓冲区。。。
    ///   * 💭也正是因此，这个「缓冲区迭代器」实际上用处不大
    /// * 🚩匹配了返回「消耗成功」，否则返回「前缀不匹配」错误
    #[inline(always)]
    fn offset_skip_when_starts_with(&mut self, prefix: &str, offset: &mut usize) -> ConsumeResult {
        if self.iter.starts_with(prefix.chars()) {
            // 统一跳过
            let len = prefix.chars().count();
            *offset += len;
            self.iter.buffer_consume_n(len);
            // 返回「消耗成功」
            Self::ok(())
        } else {
            // 返回「消耗失败」
            self.err(&format!("前缀「{prefix}」不匹配，无法前移"))
        }
    }

    /// 工具/字符匹配 & 偏移跳过连续空白符
    /// * 🎯复合词项/陈述 跳过左右括弧
    fn offset_skip_spaces(&mut self, offset: &mut usize) {
        // 不断在偏移处获取字符
        while let Some(c) = self.iter.buffer_get(*offset) {
            match (self.format.space.is_for_parse)(*c) {
                // 是空白符⇒偏移跳过
                true => *offset += 1,
                // 否则⇒跳出循环
                false => break,
            }
        }
    }

    /// 工具/前缀匹配 & 跳过后续空白符
    /// * 🎯复合词项/陈述 跳过左右括弧
    fn offset_skip_when_starts_with_and_skip_spaces(
        &mut self,
        prefix: &str,
        offset: &mut usize,
    ) -> ConsumeResult {
        // 尝试跳过前缀（若无，则不会继续跳过）
        self.offset_skip_when_starts_with(prefix, offset)?;
        // 然后跳过空白符
        self.offset_skip_spaces(offset);
        // 返回成功
        Self::ok(())
    }

    /// 工具/前缀匹配 & 返回 & 跳过连续空白符
    /// * 🎯词项连接符/陈述系词 多个选一个
    ///
    /// ! 📝生命周期问题：不要和原有的`'a`冲突
    fn offset_skip_when_starts_with_one_and_skip_spaces<'b, T>(
        &'b mut self,
        prefixes: &'b (impl PrefixMatch<T> + std::fmt::Debug),
        offset: &mut usize,
    ) -> ParseResult<&'b T> {
        for term in prefixes.prefixes_terms() {
            if self
                .offset_skip_when_starts_with_and_skip_spaces(
                    prefixes.get_prefix_from_term(term),
                    offset,
                )
                .is_ok()
            {
                return Self::ok(term);
            }
        }
        self.err(&format!("未在「{prefixes:?}」中找到匹配的前缀"))
    }

    // 具体内容解析 //

    /// 🔦入口
    /// * 🚩使用自身（从迭代器中）解析出一个结果
    ///   * 📌无需依赖其它外部数据
    pub fn parse(&mut self) -> ParseResult {
        // 逐个开始解析各条目、跳过空白符等
        while self.consume_one().is_ok() {}
        // 根据解析到的「中间结果」进行转换
        self.fold_mid_result()
    }

    /// 根据解析到的「中间结果」进行转换
    /// ! 不能用值的绑定……因为会导致「部分所有权移动」
    /// * 📌不然就要用`clone`，但会损失性能
    /// * ❌【2024-03-16 00:10:48】即便使用了`字段.take().unwrap()`，要规避「绑定」还是太损失效率了
    /// * 🚩【2024-03-16 00:11:27】最后折中选择「先转交，再消耗」方案
    fn fold_mid_result(&mut self) -> ParseResult {
        // 转交
        let mid_result = MidParseResult {
            term: self.mid_result.term.take(),
            punctuation: self.mid_result.punctuation.take(),
            stamp: self.mid_result.stamp.take(),
            truth: self.mid_result.truth.take(),
            budget: self.mid_result.budget.take(),
        };
        match mid_result {
            // 任务
            MidParseResult {
                budget: Some(budget),
                punctuation: Some(punctuation),
                term: Some(term),
                stamp,
                truth,
            } => Self::ok(Narsese::Task(Task {
                budget,
                sentence: Sentence {
                    term,
                    punctuation,
                    stamp: stamp.unwrap_or("".into()),
                    truth: truth.unwrap_or("".into()),
                },
            })),
            // 语句
            MidParseResult {
                punctuation: Some(punctuation),
                term: Some(term),
                stamp,
                truth,
                ..
            } => Self::ok(Narsese::Sentence(Sentence {
                term,
                punctuation,
                stamp: stamp.unwrap_or("".into()),
                truth: truth.unwrap_or("".into()),
            })),
            // 词项
            MidParseResult {
                term: Some(term), ..
            } => Self::ok(Narsese::Term(term)),
            // 无效情况
            result => self.err(&format!("无法转换「中间结果」：{result:?}")),
        }
    }

    /// （尝试）消耗一个条目
    fn consume_one(&mut self) -> ConsumeResult {
        // 返回第一个消耗成功的
        // * 💭这里还不是前缀匹配的时候
        // ? 到时是「缓冲区匹配前缀集」还是「前缀集匹配缓冲区」？如何处理？
        // ? 后续是「先划界，再解析」还是「边划界边解析」？
        //   ? 「先划界再解析」是需要把「中间结果」都变成字符串。。
        // 🚩只要有一个Ok，自身就Ok
        first! {
            // 通过结果进行匹配
            // * 📌【2024-03-16 18:23:08】若需使用静态方法优化，则所有返回值需要显示引用
            (ConsumeResult::is_ok) => (Self::ok); // ←此处使用`Self::ok`封装，但因此返回`err`的要加`return`
            // 空白符
            &self.consume_spaces() => (),
            // 预算
            &self.consume_budget() => (),
            // 词项
            &self.consume_term() => (),
            // 标点
            &self.consume_punctuation() => (),
            // 时间戳
            &self.consume_stamp() => (),
            // 真值
            &self.consume_truth() => (),
            // 其它 | ↓若无`return`，这里会被传入`Self::ok`
            _ => return self.err("没有可解析的条目"),
        }
    }

    // 空白符 //
    fn consume_spaces(&mut self) -> ConsumeResult {
        // 记录「是否有消耗掉空白符」
        let mut has_consumed = false;
        // 不断贪婪匹配缓冲区头部的字符串（任意数量空白符）
        while let Some(&current_char) = self.iter.buffer_head_item() {
            if (self.format.space.is_for_parse)(current_char) {
                // 消耗掉这个空白符 | 缓冲区递进
                has_consumed = self.iter.buffer_next().is_some();
            }
        }
        // 结束消耗
        match has_consumed {
            true => Self::ok(()),
            false => self.err("没有可消耗的空白符"),
        }
    }

    // 通用 @ 真值|预算 //

    /// 消耗左右括弧，及其内匹配的字串
    /// * 🚩从缓冲区头开始
    /// * ⚠️只在【无嵌套】时正常工作
    fn _consume_braces(&mut self, left: &str, right: &str) -> ParseResult<String> {
        // 匹配左括弧
        if self.iter.starts_with(left.chars()) {
            // 寻找右边括弧 | 缓冲区迭代
            // 🎯寻找「从前往后『第一个前缀匹配』的子串」的末尾位置
            let i_right = self.iter.find_next_prefix(right.chars());
            return match i_right {
                // 找到右括号⇒消耗，返回成功
                Some(i) => {
                    // 计算要消耗的字符个数（实际上就是「相对索引」+1）
                    let len_budget = i + right.chars().count() + 1;
                    let mut string = String::new();
                    // 消耗缓冲区字串 | 直接迭代添加
                    for _ in 0..len_budget {
                        string.push(self.iter.buffer_next().unwrap());
                    }
                    // !❌使用`buffer_next_n`会在闭包处导致借用问题
                    // self.iter
                    //     .buffer_next_n(len_budget, |c| budget.push(c.unwrap()));
                    return Self::ok(string);
                }
                // 未找到⇒上报错误
                None => self.err("缺少右括弧！"),
            };
        }
        self.err("找不到左括弧！")
    }

    // 真值 //

    /// （尝试）消耗真值
    /// * 🚩检测匹配之后，立即开始消耗，并【递归】启动下一个解析
    ///   * 📌原则：成功方消耗
    ///     * 返回`Ok`一定**消耗了字符**
    ///     * 返回`Err`反之（不会消耗字符，此时自动回溯）
    fn consume_truth(&mut self) -> ConsumeResult {
        let s = self._consume_braces(
            self.format.sentence.truth_brackets.0,
            self.format.sentence.truth_brackets.1,
        )?;

        // 尝试塞入并返回
        // ! 因为要同时使用`self.err`和`self.mid_result`，所以没法统一成一个方法
        match &self.mid_result.truth {
            // 已有⇒报错
            Some(v) => self.err(&format!("已有真值「{v}」！")),
            None => {
                self.mid_result.truth = Some(s);
                Self::ok(())
            }
        }
    }

    // 预算 //

    /// （尝试）消耗预算值
    /// * 🚩检测匹配之后，立即开始消耗，并【递归】启动下一个解析
    ///   * 📌原则：成功方消耗
    ///     * 返回`Ok`一定**消耗了字符**
    ///     * 返回`Err`反之（不会消耗字符，此时自动回溯）
    fn consume_budget(&mut self) -> ConsumeResult {
        let s = self._consume_braces(
            self.format.task.budget_brackets.0,
            self.format.task.budget_brackets.1,
        )?;

        // 尝试塞入并返回
        // ! 因为要同时使用`self.err`和`self.mid_result`，所以没法统一成一个方法
        match &self.mid_result.budget {
            // 已有⇒报错
            Some(v) => self.err(&format!("已有预算「{v}」！")),
            None => {
                self.mid_result.budget = Some(s);
                Self::ok(())
            }
        }
    }

    // 标点 //

    /// （尝试）消耗标点
    /// * 🚩检测匹配之后，立即开始消耗，并【递归】启动下一个解析
    ///   * 📌原则：成功方消耗
    ///     * 返回`Ok`一定**消耗了字符**
    ///     * 返回`Err`反之（不会消耗字符，此时自动回溯）
    ///
    fn consume_punctuation(&mut self) -> ConsumeResult {
        // 扫描前缀匹配字典的所有前缀（此中确保不会有「短的截断长的」的情况）
        let mut punctuation = None;
        for prefix in self.format.sentence.punctuations.prefixes_terms() {
            if self.iter.starts_with(prefix.chars()) {
                // 直接复制前缀
                punctuation = Some(prefix.clone());
                // 缓冲区递进（消耗）
                self.iter.buffer_consume_n(prefix.chars().count());
                break;
            }
        }
        // 分析结果并返回
        match (punctuation, &self.mid_result.stamp) {
            // 匹配都没匹配到⇒报错
            (None, _) => self.err("未匹配到标点！"),
            // 匹配到了但已有⇒报错
            (Some(_), Some(v)) => self.err(&format!("已有标点「{v}」！")),
            // 匹配到了还没有⇒插入 & Ok
            (Some(s), None) => {
                self.mid_result.stamp = Some(s);
                Self::ok(())
            }
        }
    }

    // 时间戳 //

    /// （尝试）消耗时间戳
    /// * 🚩检测匹配之后，立即开始消耗，并【递归】启动下一个解析
    ///   * 📌原则：成功方消耗
    ///     * 返回`Ok`一定**消耗了字符**
    ///     * 返回`Err`反之（不会消耗字符，此时自动回溯）
    fn consume_stamp(&mut self) -> ConsumeResult {
        let s = self._consume_braces(
            self.format.sentence.stamp_brackets.0,
            self.format.sentence.stamp_brackets.1,
        )?;

        // 尝试塞入并返回
        // ! 因为要同时使用`self.err`和`self.mid_result`，所以没法统一成一个方法
        match &self.mid_result.stamp {
            // 已有⇒报错
            Some(v) => self.err(&format!("已有时间戳「{v}」！")),
            None => {
                self.mid_result.stamp = Some(s);
                Self::ok(())
            }
        }
    }

    // 词项 //

    /// （尝试）消耗词项
    /// * 🚩检测匹配之后，立即开始消耗，并【递归】启动下一个解析
    ///   * 📌原则：成功方消耗
    ///     * 返回`Ok`一定**消耗了字符**
    ///     * 返回`Err`反之（不会消耗字符，此时自动回溯）
    fn consume_term(&mut self) -> ConsumeResult {
        first! {
            // 第一个Ok的⇒Ok
            (ConsumeResult::is_ok) => (Self::ok);
            // 复合词项
            &self.consume_compound() => (),
            // 陈述词项
            &self.consume_statement() => (),
            // 原子词项
            &self.consume_atom() => (),
            // 未匹配到
            _ => return self.err("未匹配到词项！"),
        }
    }

    /// （尝试）消耗复合词项（递归）
    fn consume_compound(&mut self) -> ConsumeResult {
        // TODO: 左括弧⇒连接符⇒词项⇒右括弧
        // * ⚠️无法直接消耗左括弧：消耗了后续就没法回溯
        // * ⚠️不能预先确定右边界：输入「字符迭代器」无右界 & 有可能有嵌套括弧
        let mut char_len_offset = 0_usize;

        // （尝试）匹配左括弧并前移offset（不更改缓冲区），顺带跳过空白符
        self.offset_skip_when_starts_with_and_skip_spaces(
            self.format.compound.brackets.0,
            &mut char_len_offset,
        )?;

        // 匹配连接符
        let connecter = self.offset_skip_when_starts_with_one_and_skip_spaces(
            &self.format.compound.connecters,
            &mut char_len_offset,
        )?;

        // 构造结果词项
        let mut compound = Term::Compound {
            connecter: connecter.to_owned(),
            terms: vec![],
        };

        // 开始匹配并填充组分
        // ? 💫【2024-03-16 20:39:39】这里应该装填词项
        //   ? 所以应该先匹配并返回一个词项
        //   ? ❗但返回词项需要独立出一个`parse_term`函数
        //   ? 并且迭代器状态（缓冲区等）还没法共享（返回的时候还不能改变缓冲区）
        //   ? 于是又要把目前这个「偏移量」传递过去
        //   ? 💥这又回到了先前「枚举Narsese」所用的「字符数组+头索引指针」方案
        //   ? 💢还不如按原来的方案——缓冲区迭代器的优势彻底丧尽
        // * 💭弃用「缓冲区迭代器」，改用其它方案

        // （尝试）匹配右括弧并前移offset（不更改缓冲区），顺带跳过空白符
        self.offset_skip_when_starts_with_and_skip_spaces(
            self.format.compound.brackets.1,
            &mut char_len_offset,
        )?;

        // 最终成功⇒消耗字符串
        self.iter.buffer_consume_n(char_len_offset);
        Self::ok(())
    }

    /// （尝试）消耗陈述词项（递归）
    fn consume_statement(&mut self) -> ConsumeResult {
        // TODO: 左括弧⇒左词项⇒系词⇒右词项
        // * 💡使用「预划分」思路：预先找好系词的位置，然后界定并解析左词项
        //   * ✅这样就避免了「把系词当作词项的一部分」的情况
        todo!("开发中")
    }

    /// （尝试）消耗原子词项（递归）
    fn consume_atom(&mut self) -> ConsumeResult {
        // TODO: 前缀⇒贪婪匹配「标识符文本」（支持短横线？）
        todo!("开发中")
    }
}

/// 总定义
impl<'a> NarseseFormat<'a> {
    /// 构造解析状态
    /// * 索引默认从开头开始
    pub fn build_parse_state(
        &'a self,
        input: impl IntoIterator<Item = char> + 'a,
    ) -> ParseState<'a, char> {
        ParseState::new(self, input.into_iter())
    }

    /// 主解析函数@字符串
    pub fn parse(&self, input: &str) -> ParseResult {
        // 转发到（有所有权的）迭代器
        self.parse_from_iter(input.into_chars())
    }

    /// 主解析函数@迭代器
    /// * 🚩从一个字符迭代器开始解析
    /// * 📝放弃使用类似`trait CanLexicalParse`的「方法重载」架构
    ///   * ❌无法解决的冲突：trait无法同时对「所有实现了某特征的类型」和「特别指定的类型」实现
    ///     * 📄case：字符串🆚字符迭代器
    ///     * 📌原因：有可能「某特征」会在其它地方对「特别指定的类型」进行实现，这时候分派方法就会出歧义（走「通用」还是「专用」？）
    ///     * 💭Julia的多分派借「层级类型系统」选择了「偏袒特定类型」的方案，但Rust不同
    pub fn parse_from_iter(&self, input: impl Iterator<Item = char>) -> ParseResult {
        // 构造解析状态
        let iter_char: Box<dyn Iterator<Item = char>> = Box::new(input);
        let mut state = self.build_parse_state(iter_char);
        // 用状态进行解析
        state.parse()
        // ! 随后丢弃状态
    }
}

/// 单元测试
#[cfg(test)]
mod test {
    use super::*;

    /// 通通用测试/尝试解析并返回错误
    fn __test_parse(format: &NarseseFormat, input: &str) -> Narsese {
        // 解析
        let result = format.parse(input);
        // 检验
        match result {
            // 词项⇒解析出词项
            Ok(result) => result,
            // 错误
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
