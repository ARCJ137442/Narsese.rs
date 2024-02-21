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

use crate::{first, util::FloatPrecision, Budget, Punctuation, Sentence, Stamp, Task, Term, Truth};
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
type ParseResult<T = NarseseResult> = Result<T, ParseError>;
/// 用于表征「令牌消耗结果」
/// * 🎯用于在出错时传播错误
type ConsumeResult = ParseResult<()>;

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
    /// 「解析环境」的长度 | 用于缓存常用变量
    len_env: usize,
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
        // 生成解析环境
        let env = ParseState::_build_env(input);
        // 生成环境长度 // ! 直接插入会有「同时引用」的所有权问题
        let len_env = env.len();
        // 构造结构体
        ParseState {
            // 直接指向格式
            format,
            // 置入环境
            env,
            // 置入环境长度
            len_env,
            // 从首个索引开始
            head,
            // 从空结果开始
            mid_result: MidParseResult::new(),
        }
    }

    /// 重置状态到指定情形
    /// * 用于重定向上下文
    /// * 📌自动内联
    #[inline(always)]
    pub fn reset_to(&mut self, env: ParseEnv, head: ParseIndex) {
        self.env = env;
        self.head = head;
    }

    /// 重置状态
    /// * 重置状态到默认情形：解析环境不变，头索引指向`0`
    /// * 📌自动内联
    #[inline(always)]
    pub fn reset(&mut self) {
        self.head = 0;
    }

    /// 生成「解析成功」结果：无需内联自身状态
    /// * 🎯用于最后「生成结果」的情况
    /// * 📝生成的结果不能与自身有任何瓜葛
    /// * 📌自动内联
    #[inline(always)]
    pub fn ok(result: NarseseResult) -> ParseResult {
        Ok(result)
    }

    /// 生成「解析错误」结果：直接根据消息内联自身解析状态
    /// * 🎯用于最后「生成结果」的情况
    /// * 📝生成的结果不能与自身有任何瓜葛
    ///   * 📌后续「错误」中引用的「解析环境」可能在「状态销毁」后导致「悬垂引用」问题
    /// * 📝合并「消耗错误」结果：泛型参数可以自动捕获返回类型
    /// * 📌自动内联
    #[inline(always)]
    pub fn err<T>(&self, message: &str) -> ParseResult<T> {
        Err(ParseError::new(message, self.env.clone(), self.head))
    }

    /// 生成「消耗成功」结果：无需内联自身状态
    /// * 🎯用于中间「消耗字符」的情况
    /// * 📌自动内联
    #[inline(always)]
    pub fn ok_consume() -> ConsumeResult {
        Ok(())
    }
}

/// 匹配并执行第一个匹配到的分支
/// * 🎯用于快速识别开头
/// 📝`self`是一个内容相关的关键字，必须向其中传递`self`作为参数
macro_rules! first_method {
    {
        // * 传入「self.方法名」作为被调用的方法
        $self_:ident.$method_name: ident;
        // * 传入所有的分支
        $( $pattern:expr => $branch:expr ),*,
        // * 传入「else」分支
        _ => $branch_else:expr $(,)?
    } => {
        // 插入`first!`宏中
        first! {
            $( $self_.$method_name($pattern) => $branch ),*,
            _ => $branch_else
        }
    };
}

/// 匹配并执行第一个成功匹配的分支
/// * 🎯用于快速识别开头并执行代码
///   * 📌对匹配失败者：还原头索引，并继续下一匹配
/// * 📌用于消歧义：💢「独立变量」和「预算值」开头撞了
/// 📝`self`是一个内容相关的关键字，必须向其中传递`self`作为参数
macro_rules! first_method_ok {
    {
        // * 传入「self.方法名」作为被调用的方法
        $self_:ident . $method_name:ident;
        // * 传入「self.方法名」作为「移动头索引」的方法
        $self_move:ident . $method_move:ident;
        // * 传入「当前头索引」表达式
        $original_head:expr;
        // * 传入所有的分支
        $( $pattern:expr => $branch:expr ),*,
        // * 传入「else」分支
        _ => $branch_else:expr $(,)?
    } => {
        {
            // 缓存「头索引」
            let original_head = $original_head;
            let mut result;
            // 插入`first!`宏中
            first! {
                $(
                    // 每一个条件分支
                    (
                        // 先决条件：匹配判别方法
                        $self_.$method_name($pattern)
                        // 后续条件：是否执行成功
                        && {
                            // 回到原始头索引
                            $self_move.$method_move(original_head);
                            // 预先计算结果
                            result = $branch;
                            // 尝试匹配模式：只有`Ok`能截断返回
                            matches!(result, Ok(_))
                        }
                    ) => result
                ),*,
                // 以上条件均失效时，匹配的分支
                _ => $branch_else
            }
        }
    };
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
        self.build_mid_result()?;
        // 转换解析结果
        self.transform_mid_result()
    }

    // 消耗文本 | 构建「中间解析结果」 //

    /// 判断「是否可继续消耗」
    /// * 🎯用于抽象「是否可（向右）消耗」的逻辑
    /// * 🚩逻辑：判断「头部索引」是否超出范围`[0, 解析环境长度)`
    /// * 📌自动内联
    #[inline(always)]
    fn can_consume(&self) -> bool {
        self.head < self.len_env
    }

    /// 获取当前字符
    /// * 🎯用于抽象「获取当前字符」的逻辑
    /// * 🚩逻辑：获取当前字符
    /// * 📌自动内联
    /// * ⚠️未检查边界，可能会panic
    #[inline(always)]
    fn head_char(&self) -> char {
        self.env[self.head]
    }

    /// 头索引移动
    /// * 🎯用于抽象「头部索引移动到指定位置」的过程
    ///   * ⚠️基于字符，不是字节
    /// * 🚩逻辑：头部索引赋值
    /// * 📌自动内联
    #[inline(always)]
    fn head_move(&mut self, to: ParseIndex) {
        self.head = to;
    }

    /// 头索引递进
    /// * 🎯用于抽象「头部索引位移」的过程
    ///   * ⚠️跳过的是字符，不是字节
    /// * 🚩逻辑：头部索引增加赋值
    /// * 📌自动内联
    #[inline(always)]
    fn head_step(&mut self, step: usize) {
        self.head += step;
    }

    /// 头索引移位（单个字符）
    /// * 🎯用于抽象「头部索引递进」的过程
    /// * 🚩逻辑：头部索引递进一个字符
    /// * 📌自动内联
    #[inline(always)]
    fn head_step_one(&mut self) {
        self.head_step(1)
    }

    /// 头索引跳过
    /// * 🎯用于抽象「头部索引跳过」的过程
    /// * 🚩逻辑：头部索引根据字符数量递进
    /// * 📌自动内联
    #[inline(always)]
    fn head_skip(&mut self, to_be_skip: &str) {
        // 跳过「字符数」个字符
        self.head_step(to_be_skip.chars().count())
    }

    /// 头索引跳过系列空白
    /// * 🎯用于抽象「头部索引跳过空白序列」的过程
    /// * 🚩逻辑：有多少空白跳过多少空白
    /// * 📌自动内联
    #[inline(always)]
    fn head_skip_spaces(&mut self) {
        while self.starts_with(self.format.space) {
            self.head_skip(self.format.space);
        }
    }

    /// 构建「中间解析结果」/入口
    /// * 🚩核心逻辑
    ///   * 1 不断从「解析环境」中消耗文本（头部索引`head`右移）并置入「中间解析结果」中
    ///   * 2 直到「头部索引」超过文本长度（越界）
    fn build_mid_result(&mut self) -> ConsumeResult {
        // 在「可以继续消耗」时
        while self.can_consume() {
            // 索引跳过系列空白 | 用于处理对象之间的空白
            self.head_skip_spaces();
            // 仍能继续消耗⇒消耗文本
            if self.can_consume() {
                // 消耗文本&置入「中间结果」
                self.consume_one()?;
            }
        }
        // 返回「消耗成功」结果
        Self::ok_consume()
    }

    /// 检查自己的「解析环境」是否在「头部索引」处以指定字符串开头
    fn starts_with(&self, to_compare: &str) -> bool {
        // 长度检验
        if self.len_env - self.head < to_compare.chars().count() {
            // 长度不够⇒肯定不匹配
            return false;
        }
        // 逐个字符比较
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
    /// * 此处使用`first!`代表「截断条件表达式」
    /// * 📌该函数仅承担分支工作
    ///   * 「头部索引位移」在分支中进行
    ///   * 当前一分支失败（返回Err）时，自动尝试匹配下一个分支
    ///     * 🎯用于解决「『预算值』『独立变量』相互冲突」的问题
    /// * ⚠️【2024-02-21 17:17:58】此处引入「词项→标点」的固定顺序
    ///   * 🎯为了解决如 `?查询变量vs问题?` 的冲突
    ///     * 不应「先消耗为问题，然后消耗为词语，最后遇到重复标点」
    fn consume_one(&mut self) -> ConsumeResult {
        first_method_ok! {
            // 匹配开头
            self.starts_with;
            // 当匹配失败时移回原始索引
            self.head_move;
            // 要缓存的索引
            self.head;

            // 空格⇒跳过 //
            self.format.space => Ok(self.head_skip(self.format.space)),
            // 预算值 //
            self.format.task.budget_brackets.0 => self.consume_budget(),
            // 时间戳 //
            self.format.sentence.stamp_brackets.0 => self.consume_stamp(),
            // 真值 //
            self.format.sentence.truth_brackets.0 => self.consume_truth(),
            // 词项→标点（兜底） //
            _ => {
                // 先解析词项
                let result = self.consume_term();
                // 然后软性消耗「标点」
                if self.can_consume() {
                    let _ = self.consume_punctuation();
                }
                // 返回词项的解析结果
                result
            },
        }
    }

    /// 消耗
    fn consume_punctuation(&mut self) -> ConsumeResult {
        first_method! {
            // 匹配开头
            self.starts_with;
            // 标点 // ⚠️因开头不同且无法兜底，故直接内联至此
            // 判断
            self.format.sentence.punctuation_judgement => self.consume_punctuation_judgement(),
            // 目标
            self.format.sentence.punctuation_goal => self.consume_punctuation_goal(),
            // 问题
            self.format.sentence.punctuation_question => self.consume_punctuation_question(),
            // 请求
            self.format.sentence.punctuation_quest => self.consume_punctuation_quest(),
            // 否则⇒错误
            _ => self.err("未知的标点")
        }
    }

    /// 工具函数/尝试置入
    /// * 🚩仅对单个[`Option`]对象
    /// * 返回
    ///   * 无（成功）
    ///   * 格式化后的「错误消息」（失败）
    ///     * 具体细节需要不可变引用进行补充
    /// * 🎯统一格式化错误消息，并减少重复代码量
    ///   * 用于「向『中间解析结果』插入值」
    ///   * 📌缘由：无法引用「结构字段」
    ///     * 💢明明[`Self::err`]、[`Option::insert`]互不干扰，但仍然会报所有权问题
    /// * 📌自动内联
    #[inline(always)]
    fn try_set<T: std::fmt::Debug>(
        option: &mut Option<T>,
        new_value: T,
        name: &str,
    ) -> Option<String> {
        match option {
            // 已有⇒报错
            Some(old_value) => Some(format!(
                "尝试置入{name}「{new_value:?}」遇到已有{name}「{old_value:?}」"
            )),
            // 无⇒置入&结束
            None => {
                // 置入 // ! 无需使用其返回值
                let _ = option.insert(new_value);
                // 结束
                None
            }
        }
    }

    /// 工具函数/尝试置入标点
    /// * 📌自动内联
    #[inline(always)]
    fn _try_set_punctuation(&mut self, punctuation: Punctuation) -> ConsumeResult {
        // 尝试置入
        match Self::try_set(&mut self.mid_result.punctuation, punctuation, "标点") {
            Some(message) => self.err(&message),
            None => Self::ok_consume(),
        }
    }

    /// 消耗&置入/标点/判断
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn consume_punctuation_judgement(&mut self) -> ConsumeResult {
        // 索引跳过
        self.head_skip(self.format.sentence.punctuation_judgement);
        // 尝试置入标点
        self._try_set_punctuation(Punctuation::Judgement)
    }

    /// 消耗&置入/标点/目标
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn consume_punctuation_goal(&mut self) -> ConsumeResult {
        // 索引跳过
        self.head_skip(self.format.sentence.punctuation_goal);
        // 尝试置入标点
        self._try_set_punctuation(Punctuation::Goal)
    }

    /// 消耗&置入/标点/问题
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn consume_punctuation_question(&mut self) -> ConsumeResult {
        // 索引跳过
        self.head_skip(self.format.sentence.punctuation_question);
        // 尝试置入标点
        self._try_set_punctuation(Punctuation::Question)
    }

    /// 消耗&置入/标点/请求
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn consume_punctuation_quest(&mut self) -> ConsumeResult {
        // 索引跳过
        self.head_skip(self.format.sentence.punctuation_quest);
        // 尝试置入标点
        self._try_set_punctuation(Punctuation::Quest)
    }

    /// 解析&置入/固定次数分隔的浮点数
    /// * 使用常量`N`指定解析的数目
    ///   * 多的会报错
    ///   * 少的会忽略（额外返回「解析出的数目」作为标记）
    fn parse_separated_floats<const N: usize>(
        &mut self,
        separator: &str,
        right_bracket: &str,
    ) -> ParseResult<([FloatPrecision; N], usize)> {
        // 直接初始化定长数组
        let mut result: [FloatPrecision; N] = [0.0; N];
        // 构造数值缓冲区
        let mut value_buffer = String::new();
        // 填充数组
        let mut i: usize = 0;
        while self.can_consume() {
            match self.head_char() {
                // 空白⇒跳过
                _ if self.starts_with(self.format.space) => self.head_skip(self.format.space),
                // 小数点
                // 数值|小数点⇒计入缓冲区&跳过
                '.' | '0'..='9' => {
                    value_buffer.push(self.head_char());
                    self.head_step_one();
                }
                // 分隔符⇒解析并存入数值&跳过
                _ if self.starts_with(separator) => {
                    // 解析并存入数值
                    match value_buffer.parse::<FloatPrecision>() {
                        // 有效数值
                        Ok(value) => {
                            // 填充数组
                            result[i] = value;
                            // 清空缓冲区
                            value_buffer.clear();
                            // 跳过分隔符
                            self.head_skip(separator);
                            // 增加计数
                            i += 1;
                        }
                        // 无效数值
                        Err(_) => {
                            // 无效数值
                            return self.err(&format!("{value_buffer:?}不是有效的数值"));
                        }
                    }
                }
                // 尾括弧⇒跳出循环 | 「跳出尾括弧」在循环外操作
                _ if self.starts_with(right_bracket) => {
                    break;
                } // 其它⇒无效字符
                c => return self.err(&format!("在解析浮点序列时出现无效字符{c:?}")),
            }
        }
        // 返回最终结果
        Ok((result, i + 1))
    }

    /// 消耗&置入/时间戳
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn consume_stamp(&mut self) -> ConsumeResult {
        // TODO: 有待完成
        self.err("TODO!")
    }

    /// 消耗&置入/真值
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn consume_truth(&mut self) -> ConsumeResult {
        // 跳过左括弧
        self.head_skip(self.format.sentence.truth_brackets.0);
        let ([f, c], num) = self.parse_separated_floats::<2>(
            self.format.sentence.truth_separator,
            self.format.sentence.truth_brackets.1,
        )?;
        // 构造真值
        let truth = match num {
            // 无⇒空真值
            0 => Truth::Empty,
            // 单⇒单真值
            1 => Truth::Single(f),
            // 双⇒双真值
            _ => Truth::Double(f, c),
        };
        // 跳过右括弧
        self.head_skip(self.format.sentence.truth_brackets.1);
        // 尝试置入真值
        match Self::try_set(&mut self.mid_result.truth, truth, "真值") {
            Some(message) => self.err(&message),
            None => Self::ok_consume(),
        }
    }

    /// 消耗&置入/预算值
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn consume_budget(&mut self) -> ConsumeResult {
        // 跳过左括弧
        self.head_skip(self.format.task.budget_brackets.0);
        let ([p, d, q], num) = self.parse_separated_floats::<3>(
            self.format.task.budget_separator,
            self.format.task.budget_brackets.1,
        )?;
        // 构造预算
        let budget = match num {
            // 无⇒空预算
            0 => Budget::Empty,
            // 单⇒单预算
            1 => Budget::Single(p),
            // 双⇒双预算
            2 => Budget::Double(p, d),
            // 三⇒三预算
            _ => Budget::Triple(p, d, q),
        };
        // 跳过右括弧
        self.head_skip(self.format.task.budget_brackets.1);
        // 尝试置入预算
        match Self::try_set(&mut self.mid_result.budget, budget, "预算值") {
            Some(message) => self.err(&message),
            None => Self::ok_consume(),
        }
    }

    /// 消耗&置入/词项
    /// * 🚩消耗&解析出一个词项，然后置入「中间解析结果」中
    /// * 📌需要递归解析，因此不能直接开始「置入」
    fn consume_term(&mut self) -> ConsumeResult {
        // 先解析词项
        let term = self.parse_term()?;
        // 尝试置入词项
        match Self::try_set(&mut self.mid_result.term, term, "词项") {
            Some(message) => self.err(&message),
            None => Self::ok_consume(),
        }
    }

    /// 消耗&解析/词项
    /// * 🎯仍然只负责分派方法
    ///   * 乃至无需`?`语法糖（错误直接传递，而无需提取值）
    fn parse_term(&mut self) -> ParseResult<Term> {
        first_method! {
            self.starts_with;
            // 词项/外延集
            self.format.compound.brackets_set_extension.0 => self.parse_compound_set_extension(),
            // 词项/内涵集
            self.format.compound.brackets_set_intension.0 => self.parse_compound_set_intension(),
            // 词项/复合词项
            self.format.compound.brackets.0 => self.parse_compound(),
            // 词项/陈述
            self.format.statement.brackets.0 => self.parse_statement(),
            // 词项/原子（兜底）
            _ => self.parse_atom()
        }
    }

    /// 消耗&置入/词项/复合（外延集）
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn parse_compound_set_extension(&mut self) -> ParseResult<Term> {
        // TODO: 有待完成
        self.err("TODO!")
    }

    /// 消耗&置入/词项/复合（内涵集）
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn parse_compound_set_intension(&mut self) -> ParseResult<Term> {
        // TODO: 有待完成
        self.err("TODO!")
    }

    /// 消耗&置入/词项/复合（括弧）
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn parse_compound(&mut self) -> ParseResult<Term> {
        // TODO: 有待完成
        self.err("TODO!")
    }

    /// 消耗&置入/词项/陈述
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn parse_statement(&mut self) -> ParseResult<Term> {
        // TODO: 有待完成
        self.err("TODO!")
    }

    /// 工具函数/判断字符是否能作为「词项名」
    /// * 🎯用于判断「合法词项名」
    #[inline(always)]
    fn is_valid_atom_name(c: char) -> bool {
        match c {
            // 特殊：横杠/下划线
            '-' | '_' => true,
            //  否则：判断是否为「字母/数字」
            _ => c.is_alphabetic() || c.is_numeric(),
        }
    }

    /// 消耗&置入/词项/原子
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn parse_atom(&mut self) -> ParseResult<Term> {
        // 消耗前缀，并以此预置「词项」
        let mut term;
        first_method! {
            self.starts_with;
            // 占位符 | 此举相当于识别以「_」开头的词项
            self.format.atom.prefix_placeholder => {
                // 词项赋值
                term = Term::new_placeholder();
                // 头索引跳过
                self.head_skip(self.format.atom.prefix_placeholder);
            },
            // 独立变量
            self.format.atom.prefix_variable_independent => {
                // 词项赋值
                term = Term::new_variable_independent("");
                // 头索引跳过
                self.head_skip(self.format.atom.prefix_variable_independent);
            },
            // 非独变量
            self.format.atom.prefix_variable_dependent => {
                // 词项赋值
                term = Term::new_variable_dependent("");
                // 头索引跳过
                self.head_skip(self.format.atom.prefix_variable_dependent);
            },
            // 查询变量
            self.format.atom.prefix_variable_query => {
                // 词项赋值
                term = Term::new_variable_query("");
                // 头索引跳过
                self.head_skip(self.format.atom.prefix_variable_query);
            },
            // 间隔
            self.format.atom.prefix_interval => {
                // 词项赋值
                term = Term::new_interval(0);
                // 头索引跳过
                self.head_skip(self.format.atom.prefix_interval);
            },
            // 操作符
            self.format.atom.prefix_operator => {
                // 词项赋值
                term = Term::new_operator("");
                // 头索引跳过
                self.head_skip(self.format.atom.prefix_operator);
            },
            // 词语 | ⚠️必须以此兜底（空字串也算前缀）
            self.format.atom.prefix_word => {
                term = Term::new_word("");
                // 头索引跳过
                self.head_skip(self.format.atom.prefix_word);
            },
            _ => {
                return self.err("未知的原子词项前缀")
            }
        }
        // 新建缓冲区
        let mut name_buffer = String::new();
        let mut head_char;
        // 可消耗时重复，加载进名称缓冲区
        while self.can_consume() {
            // 获取头部字符
            head_char = self.head_char();
            match Self::is_valid_atom_name(head_char) {
                // 合法词项名字符⇒加入缓冲区&递进
                true => {
                    // 加入缓冲区
                    name_buffer.push(head_char);
                    // 跳过当前字符
                    self.head_step_one();
                }
                // 非法字符⇒结束循环 | 此时已自动消耗到「下一起始位置」
                false => break,
            }
        }
        // 对「占位符」进行特殊处理：直接返回（忽略缓冲区）
        if let Term::Placeholder = term {
            return Ok(term);
        }
        // 非「占位符」检验名称非空
        if name_buffer.is_empty() {
            return self.err("词项名不能为空");
        }
        // 尝试将缓冲区转为词项名，返回词项/错误
        match term.set_atom_name(&name_buffer) {
            // 成功⇒返回词项
            Ok(_) => Ok(term),
            // 失败⇒传播错误 | 💭总是要转换错误类型
            Err(_) => self.err(&format!("非法词项名 {name_buffer:?}")),
        }
    }

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
                Self::ok(NarseseResult::Task(value))
            }
            // else有标点&词项⇒语句
            (_, Some(_), Some(_), ..) => {
                // !【2024-02-20 21:58:21】必须先进行可变借用
                let value = self.form_sentence();
                // 然后再进行不可变借用（以构造最终值）
                Self::ok(NarseseResult::Sentence(value))
            }
            // else有词项⇒词项
            (_, Some(_), ..) => {
                // !【2024-02-20 21:58:21】必须先进行可变借用
                let value = self.form_term();
                // 然后再进行不可变借用（以构造最终值）
                Self::ok(NarseseResult::Term(value))
            }
        }
    }
}

/// 总定义
impl NarseseFormat<&str> {
    /// 构造解析状态
    /// * 索引默认从开头开始
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
    use crate::{
        conversion::string::{impl_parser::NarseseResult, NarseseFormat, FORMAT_ASCII},
        fail_tests, show,
    };

    /// 生成「矩阵」
    /// * 结果：`Vec<(format, Vec<result>)>`
    macro_rules! f_matrix {
        [
            $f:ident;
            $($format:expr),+ $(,)?;
            $($input:expr),+ $(,)? $(;)?
        ] => {
            {
                // 新建一个矩阵
                let mut matrix = vec![];
                // 生成行列
                let formats = [$($format),+];
                let inputs = [$($input),+];
                // 给矩阵添加元素
                for format in formats {
                    // 新建一个列
                    let mut col = vec![];
                    // 生成列元素
                    for input in inputs {
                        col.push($f(format, input))
                    }
                    // 添加列
                    matrix.push((format, col));
                }
                // 返回矩阵
                matrix
            }
        };
    }

    /// 通用测试/原子词项
    fn _test_parse_atom(format: &NarseseFormat<&str>, input: &str) {
        // 解析
        let result = format.parse(input);
        show!(&result);
        // 检验
        let term = match result {
            // 词项⇒解析出词项
            Ok(NarseseResult::Term(term)) => term,
            // 错误
            Err(e) => {
                show!(e);
                panic!("词项解析失败");
            }
            // 别的解析结果
            _ => panic!("解析出来的不是词项！{result:?}"),
        };
        // 展示
        show!(term);
    }

    /// 测试/原子词项
    #[test]
    fn test_parse_atom() {
        let format_ascii = FORMAT_ASCII;
        let matrix = f_matrix! [
            // 应用的函数
            _test_parse_atom;
            // 格式×输入
            &format_ascii;
            "word", "_", "$i_var", "#d_var", "?q_var", "+137", "^op";
        ];
        show!(matrix);
    }

    // 测试/原子词项/失败
    fail_tests! {
        test_parse_atom_fail_未知前缀 _test_parse_atom(&FORMAT_ASCII, "@word");
        test_parse_atom_fail_未知前缀2 _test_parse_atom(&FORMAT_ASCII, "`word");
        test_parse_atom_fail_非法字符1 _test_parse_atom(&FORMAT_ASCII, ",");
        test_parse_atom_fail_非法字符2 _test_parse_atom(&FORMAT_ASCII, "wo:rd");
        test_parse_atom_fail_非法字符3 _test_parse_atom(&FORMAT_ASCII, "wo[rd");
        test_parse_atom_fail_非法字符4 _test_parse_atom(&FORMAT_ASCII, "wo啊/d");
    }

    /// 通用测试/语句
    fn _test_parse_sentence(format: &NarseseFormat<&str>, input: &str) {
        // 解析
        let result = format.parse(input);
        show!(&result);
        // 检验
        let term = match result {
            // 语句⇒解析出语句
            Ok(NarseseResult::Sentence(sentence)) => sentence,
            // 错误
            Err(e) => panic!("语句解析失败{e}"),
            // 别的解析结果
            _ => panic!("解析出来的不是语句！{result:?}"),
        };
        // 展示
        show!(term);
    }

    /// 测试/标点（语句）
    #[test]
    fn test_parse_punctuation() {
        let matrix = f_matrix! [
        // 应用的函数
        _test_parse_sentence;
        // 格式×输入
        &FORMAT_ASCII;
        "判断.", "目标!", "问题?", "请求@", "?查询变量vs问题?"
        ];
        show!(matrix);
    }

    /// 通用测试/任务
    fn _test_parse_task(format: &NarseseFormat<&str>, input: &str) {
        // 解析
        let result = format.parse(input);
        show!(&result);
        // 检验
        let term = match result {
            // 任务⇒解析出任务
            Ok(NarseseResult::Task(task)) => task,
            // 错误
            Err(e) => panic!("任务解析失败{e}"),
            // 别的解析结果
            _ => panic!("解析出来的不是任务！{result:?}"),
        };
        // 展示
        show!(term);
    }

    /// 测试/真值（语句）
    #[test]
    fn test_parse_truth() {
        let matrix = f_matrix! [
            // 应用的函数
            _test_parse_sentence;
            // 格式×输入
            &FORMAT_ASCII;
            "判断. %1.0;0.9%", "目标! %.0;.9%", "问题?", "请求@"
        ];
        show!(matrix);
    }

    /// 测试/预算值（任务）
    #[test]
    fn test_parse_budget() {
        let matrix = f_matrix! [
            // 应用的函数
            _test_parse_task;
            // 格式×输入
            &FORMAT_ASCII;
            "$0.5;0.5;0.5$ 判断. %1.0;0.9%",
            "$.7;.75;0.555$目标! %.0;.9%",
            "$1;1;1$ 问题?",
            "$0;0;0$请求@"
        ];
        show!(matrix);
    }

    // 词项
    #[test]
    fn test_parse_term() {}
}
