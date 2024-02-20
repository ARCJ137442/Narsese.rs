//! 实现/解析器
//!
//! ! 📝使用trait构造`to_result`快速生成`Ok((input, [XXXX]))`的办法不实用
//! * 此举仅能应对形如`IResult<&str, ParseResult>`的结果
//! * 无法定义方法`fn to_result_self(self, input: &str) -> IResult<&str, Self>;`
//!   * ❗`` the size for values of type `Self` cannot be known at compilation time required because it appears within the type `(&str, Self)` ``
//!   * 📌[`IResult<I,O>`]参与了元组类型`(I,O)`，而作为`O`置入的`Self`大小未知
//! * 🚩结论：分别在模块下实现私用方法`to_result`返回`IResult<&str, Self>`
//!
//! ! 📝【2024-02-20 17:25:09】暂缓使用[`nom`]构造解析器
//! * 📌原因：难以实现「动态语法元素插入」
//!   * 💭似乎[`nom`]仅能用于一个【一切语法元素均已固定】的语法解析器生成
//!     * 如固定的`JSON`、`TOML`之类
//!   * 💥冲突：[`Parser`]需要根据[`NarseseFormat`]存储的「语法信息」进行解析
//!
//! ! 📝【2024-02-20 17:38:05】弃用[`pest`]库：亦有同样的「动态性缺失」现象
//! * 目前从[包文档](https://pest.rs/book)中得到的信息：似乎只支持固定的「PEG语法文件」（`*.pest`）
//!   * 此举使用`#[grammar="【文件名】"]`的宏定义——除非「根据格式生成文件」，否则无法自动「由格式生成规则」
//!     * 与Julia的[PikaParser.jl](https://github.com/LCSB-BioCore/PikaParser.jl)不同：「规则」并不作为一个「可被构造的对象」而存在，故无法进行「格式插值」
//! * 🚩目前采用与[JuNarsese](https://github.com/ARCJ137442/JuNarsese.jl)类似的「手写」方式
//!
//! * 🚩采用「字符数组+消耗后索引」的设计架构
//!   * 📌具体解析在一个（作为整体的）文本字符数组中进行
//!     * ✨被称作「解析环境」
//!   * 📌解析函数总是从某个「起始位置」开始，通过系列解析过程，返回「解析结果」以及
//!     * ✨有相应的「结果索引」类型

use crate::{first, Budget, Punctuation, Sentence, Stamp, Task, Term, Truth};
use std::{error::Error, fmt::Display};

use super::NarseseFormat;

/// 定义一个「CommonNarsese结果」类型
/// * 🎯用于存储「最终被解析出来的CommonNarsese对象」
///   * 词项
///   * 语句
///   * 任务
#[derive(Debug, Clone)]
pub enum NarseseResult {
    /// 解析出来的词项
    Term(Term),
    /// 解析出来的语句
    Sentence(Sentence),
    /// 解析出来的任务
    Task(Task),
}

/// 定义「CommonNarsese组分」的结构
/// * 🎯用于存储「中间解析结果」
///   * 🚩服务的核心过程：文本==解析=>各大组分==组装=>解析结果
/// * 📌使用[`Option`]存储「可能有可能没有的成分」
///   * 允许成分缺省（后续「转换成最终结果」时再报错）
///   * 允许顺序不定
#[derive(Debug, Clone, Default)]
struct MidParseResult {
    /// 词项
    term: Option<Term>,
    /// 真值 @ 语句
    truth: Option<Truth>,
    /// 预算值 @ 任务
    budget: Option<Budget>,
    /// 时间戳 @ 语句
    stamp: Option<Stamp>,
    /// 标点 @ 语句
    punctuation: Option<Punctuation>,
}

/// 实现/构造
///
/// ! 不直接实现`Into<ParseResult>`：报错信息需要「解析状态」
impl MidParseResult {
    /// 构造一个全空的结果
    pub fn new() -> Self {
        Self {
            term: None,
            truth: None,
            budget: None,
            stamp: None,
            punctuation: None,
        }
    }
}

/// 用于表征「解析环境」
/// * 具有所有权
type ParseEnv<T = char> = Vec<T>;
/// 用于表征「解析索引」
type ParseIndex = usize;

/// 用于表征「解析结果」
/// * 用于表示「解析对象」
///
/// ! 📝原先基于「返回『(解析出的对象, 下一起始索引)』」的方法已无需使用
/// * 现在是基于「解析器状态」的「状态机模型」
///   * 📌关键差异：附带可设置的「中间解析结果」与「可变索引」
///   * 🚩子解析函数在解析之后，直接填充「中间解析结果」并修改「可变索引」
type ParseResult = Result<NarseseResult, ParseError>;

/// 用于表征「解析错误」
/// * 📝不要依赖于任何外部引用：后续需要【脱离】解析环境
#[derive(Debug, Clone)]
pub struct ParseError {
    /// 错误消息 | 一般不含冒号
    /// * 🎯用于描述出错原因
    message: String,
    /// 裁剪出的「解析环境」切片
    /// * 🎯用于展示出错范围
    env_slice: ParseEnv,
    /// 出错所在的「解析索引」
    /// * 🎯用于指示出错位置
    index: ParseIndex,
}
impl ParseError {
    /// 工具函数/生成「环境切片」
    fn generate_env_slice(env: ParseEnv, index: ParseIndex) -> ParseEnv {
        // 字符范围下限 | 后续截取包含
        let char_range_left = match index > ERR_CHAR_VIEW_RANGE {
            true => index - ERR_CHAR_VIEW_RANGE,
            false => 0,
        };
        // 字符范围上限 | 后续截取不包含
        let char_range_right = match index + ERR_CHAR_VIEW_RANGE + 1 < env.len() {
            true => index + ERR_CHAR_VIEW_RANGE + 1,
            false => env.len(),
        };
        // 截取字符，生成环境
        env[char_range_left..char_range_right].into()
    }

    /// 构造函数
    pub fn new(message: &str, env: ParseEnv, index: ParseIndex) -> ParseError {
        ParseError {
            message: message.to_string(),
            env_slice: ParseError::generate_env_slice(env, index),
            index,
        }
    }
}
/// 用于在报错时展示周边文本
const ERR_CHAR_VIEW_RANGE: usize = 4;
/// 呈现报错文本
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // 输出
        write!(
            f,
            "Narsese解析错误：{} @ {} in {}",
            self.message,
            self.index,
            String::from_iter(self.env_slice.iter())
        )
    }
}
impl Error for ParseError {}

/// 定义一个「解析器状态」类型
/// * 🎯除了内置「格式」外，还可【缓存】解析状态
/// * 📄学习参考：[tomllib/parser.rs](https://github.com/joelself/tomllib/blob/master/src/internals/parser.rs)
pub struct ParseState<'a, Content> {
    /// 引用的「解析格式」
    format: &'a NarseseFormat<Content>,
    /// 「解析环境」
    env: ParseEnv,
    /// 当前解析的位置 | 亦用作「下一起始索引」
    head: ParseIndex,
    /// 「中间解析结果」
    mid_result: MidParseResult,
}

/// 实现/构造、重置、生成
impl<'a, C> ParseState<'a, C> {
    /// 根据格式构造parser
    /// * 🚩方法：默认状态+重定向
    pub fn new(
        format: &'a NarseseFormat<C>,
        input: &'a str,
        head: ParseIndex,
    ) -> ParseState<'a, C> {
        ParseState {
            // 直接指向格式
            format,
            // 指向环境
            env: ParseState::_build_env(input),
            // 从首个索引开始
            head,
            // 从空结果开始
            mid_result: MidParseResult::new(),
        }
    }

    /// 重置状态到指定情形
    /// * 用于重定向上下文
    pub fn reset_to(&mut self, env: ParseEnv, head: ParseIndex) {
        self.env = env;
        self.head = head;
    }

    /// 重置状态
    /// * 重置状态到默认情形：解析环境不变，头索引指向`0`
    pub fn reset(&mut self) {
        self.head = 0;
    }

    /// 生成「解析成功」结果：直接根据值内联自身解析状态
    /// * 🎯用于最后「生成结果」的情况
    /// * 📝生成的结果不能与自身有任何瓜葛
    pub fn ok(&self, result: NarseseResult) -> ParseResult {
        Ok(result)
    }

    /// 生成「解析错误」结果：直接根据消息内联自身解析状态
    /// * 🎯用于最后「生成结果」的情况
    /// * 📝生成的结果不能与自身有任何瓜葛
    ///   * 📌后续「错误」中引用的「解析环境」可能在「状态销毁」后导致「悬垂引用」问题
    pub fn err(&self, message: &str) -> ParseResult {
        Err(ParseError::new(message, self.env.clone(), self.head))
    }
}

/// ✨实现/解析 @ 静态字串
/// 🚩整体解析流程
/// 1. 构建解析环境
/// 2. 构建「中间解析结果」
/// 3. 根据内容填充「中间解析结果」
/// 4. 转换「中间解析结果」为最终结果
impl<'a> ParseState<'a, &str> {
    // 构造 | 入口 //

    /// 构造解析环境
    #[inline(always)]
    fn _build_env(input: &'a str) -> ParseEnv {
        input.chars().collect()
    }

    /// 解析总入口 | 全部使用自身状态
    pub fn parse(&mut self) -> ParseResult {
        // 消耗文本，构建「中间解析结果」
        self.build_mid_result();
        // 转换解析结果
        self.transform_mid_result()
    }

    // 消耗文本 | 构建「中间解析结果」 //

    /// 构建「中间解析结果」/入口
    /// * 🚩核心逻辑
    ///   * 1 不断从「解析环境」中消耗文本（头部索引`head`右移）并置入「中间解析结果」中
    ///   * 2 直到「头部索引」超过文本长度（越界）
    fn build_mid_result(&mut self) {
        let len_env = self.env.len();
        // 重复直到「头部索引」超过文本长度
        while self.head < len_env {
            // 消耗文本&置入「中间结果」
            self.consume_one();
        }
    }

    /// 检查自己的「解析环境」是否在「头部索引」处以指定字符串开头
    fn starts_with(&self, to_compare: &str) -> bool {
        for (i, c) in to_compare.chars().enumerate() {
            if self.env[self.head + i] != c {
                return false;
            }
        }
        true
    }

    /// 消耗文本&置入「中间结果」
    /// * 头部索引移动
    ///   * 📌无需顾忌「是否越界」
    /// * 产生值并置入「中间解析结果」
    ///
    /// 💡📝可使用`match`简化重复的`if-else`逻辑
    /// ! 📝`match`箭头的左边只能是
    ///
    fn consume_one(&mut self) {
        // * 此处使用`match`纯属为了代码风格
        first! {
            // 空格⇒跳过
            self.starts_with(self.format.space) => {
                self.head += self.format.space.len();
            },
            // 陈述括弧开头⇒解析陈述
            self.starts_with(self.format.statement.brackets.0) => {
                self.head += self.format.space.len();
            },
            // 空格⇒跳过
            self.starts_with(self.format.space) => {
                self.head += self.format.space.len();
            },
            // 兜底⇒解析「原子词项」
            _ => {
                self.head += self.format.space.len();
            }, // TODO: 有待完备
        }
    }

    /// 消耗&置入/预算值
    /// 消耗&置入/词项/原子
    /// 消耗&置入/词项/复合（括弧）
    /// 消耗&置入/词项/复合（外延集）
    /// 消耗&置入/词项/复合（内涵集）
    /// 消耗&置入/词项/陈述
    /// 消耗&置入/标点
    /// 消耗&置入/时间戳
    /// 消耗&置入/真值

    // 组装 //

    /// 组装 | 将「中间结果」转换为词项
    /// * 📝在「中间结果内联入状态」后，不能对其中的[`Option`]对象直接使用`unwrap`方法
    ///   * ❌直接使用[`Option::unwrap`]会获取自身的所有权
    ///   * 📌可以使用`take`实现：
    ///     * 1 移交所有权给调用者
    ///     * 2 将自身设置为`None`
    fn form_term(&mut self) -> Term {
        self.mid_result.term.take().unwrap()
    }

    /// 组装 | 将「中间结果」转换为语句
    /// * 📌其中「词项」「标点」必须具有
    ///   * ⚠️若无⇒`panic`（所以请确保有）
    /// * 📝在「中间结果内联入状态」后，需要「使用[`Option::take`]转交所有权」并对代码进行拆分
    /// ! 📝不能混用「结构体整体」`result: MidParseResult`与其成员：无法「部分移动」所有权
    ///   * 📌【2024-02-20 21:56:21】现在又可复用「转换词项」「转换语句」了
    ///     * 原因：使用[`Option::take`]避开了所有权冲突
    fn form_sentence(&mut self) -> Sentence {
        Sentence::from_punctuation(
            // 必要的「词项」「标点」
            self.form_term(),
            self.mid_result.punctuation.take().unwrap(),
            // ! 默认时间戳为「永恒」
            self.mid_result.stamp.take().unwrap_or(Stamp::Eternal),
            // ! 默认真值为「空真值」
            self.mid_result.truth.take().unwrap_or(Truth::Empty),
        )
    }

    /// 组装 | 将「中间结果」转换为任务
    /// * 📌其中「预算」「词项」「标点」必须具有
    ///   * ⚠️若无⇒`panic`（所以请确保有）
    /// ! 📝无法复用[`form_sentence`]代码：无法复用所有权
    fn form_task(&mut self) -> Task {
        Task::new(
            self.form_sentence(),
            // 必要的「预算值」
            self.mid_result.budget.take().unwrap(),
        )
    }

    /// 组装 | 将「中间结果」转换为最终结果
    ///
    /// ! 📝在「中间结果内联入状态」后，需要对代码进行拆分
    /// * ❌直接嵌套[`Option::take`]（产生自身的可变引用）与[`Self::ok`]（产生自身的不可变引用）：借用冲突
    ///   * 📌必须先【可变借用】产生「元素」，再【不可变借用】产生「结果」
    ///   * 📌【2024-02-20 21:55:25】现在重新
    fn transform_mid_result(&mut self) -> ParseResult {
        // 直接匹配各个属性 | 按照CommonNarsese语序`预算值 词项 标点 时间戳 真值`排列
        match (
            // ! 📝此处必须要用「不可变借用」以避免「部分所有权移动」问题
            &self.mid_result.budget,
            &self.mid_result.term,
            &self.mid_result.punctuation,
            &self.mid_result.stamp,
            &self.mid_result.truth,
        ) {
            // 没词项不行
            (_, None, _, _, _) => self.err("词项缺失"),
            // 有预算&标点&词项⇒任务
            (Some(_), Some(_), Some(_), ..) => {
                // !【2024-02-20 21:58:21】必须先进行可变借用
                let value = self.form_task();
                // 然后再进行不可变借用（以构造最终值）
                self.ok(NarseseResult::Task(value))
            }
            // else有标点&词项⇒语句
            (_, Some(_), Some(_), ..) => {
                // !【2024-02-20 21:58:21】必须先进行可变借用
                let value = self.form_sentence();
                // 然后再进行不可变借用（以构造最终值）
                self.ok(NarseseResult::Sentence(value))
            }
            // else有词项⇒词项
            (_, Some(_), ..) => {
                // !【2024-02-20 21:58:21】必须先进行可变借用
                let value = self.form_term();
                // 然后再进行不可变借用（以构造最终值）
                self.ok(NarseseResult::Term(value))
            }
        }
    }
}

/// 总解析函数
impl NarseseFormat<&str> {
    /// 构造解析状态
    pub fn build_parse_state<'a>(&'a self, input: &'a str) -> ParseState<'a, &str> {
        ParseState::new(self, input, 0)
    }

    /// 主解析函数
    pub fn parse<'a>(&'a self, input: &'a str) -> ParseResult {
        // 构造解析状态
        let mut state: ParseState<&str> = self.build_parse_state(input);
        // 用状态进行解析
        state.parse()
        // ! 随后丢弃状态
    }
}

/// 单元测试
#[cfg(test)]
mod tests_parse {
    use crate::conversion::string::FORMAT_ASCII;

    // 词项
    #[test]
    fn test_parse_term() {
        let format = FORMAT_ASCII;
        let input = "A";
        let result = format.parse(input);
        println!("result: {result:?}");
        assert!(result.is_ok());
        let term = result.unwrap();
        println!("{term:?}");
    }
}
