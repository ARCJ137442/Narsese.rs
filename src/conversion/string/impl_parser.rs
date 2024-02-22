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

use crate::{
    first,
    util::{FloatPrecision, IntPrecision, ZeroOneFloat},
    Budget, Punctuation, Sentence, Stamp, Task, Term, Truth,
};
use std::{error::Error, fmt::Display, io::ErrorKind};

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

// 实现`(try_)From/To`转换方法
impl TryFrom<NarseseResult> for Term {
    type Error = std::io::Error;
    fn try_from(value: NarseseResult) -> Result<Self, Self::Error> {
        match value {
            NarseseResult::Term(term) => Ok(term),
            _ => Err(Self::Error::new(
                ErrorKind::InvalidData,
                format!("类型不匹配，无法转换为词项：{value:?}"),
            )),
        }
    }
}
impl TryFrom<NarseseResult> for Sentence {
    type Error = std::io::Error;
    fn try_from(value: NarseseResult) -> Result<Self, Self::Error> {
        match value {
            NarseseResult::Sentence(sentence) => Ok(sentence),
            _ => Err(Self::Error::new(
                ErrorKind::InvalidData,
                format!("类型不匹配，无法转换为语句：{value:?}"),
            )),
        }
    }
}
impl TryFrom<NarseseResult> for Task {
    type Error = std::io::Error;
    fn try_from(value: NarseseResult) -> Result<Self, Self::Error> {
        match value {
            NarseseResult::Task(task) => Ok(task),
            _ => Err(Self::Error::new(
                ErrorKind::InvalidData,
                format!("类型不匹配，无法转换为任务：{value:?}"),
            )),
        }
    }
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
            "Narsese解析错误：{} @ {} in {:?}",
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
    pub fn reset_to(&mut self, input: &str, head: ParseIndex) {
        self.env = ParseState::_build_env(input);
        self.len_env = self.env.len();
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
    pub fn ok<T>(result: T) -> ParseResult<T> {
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

/// 匹配首个前缀匹配的分支，自动跳过前缀并执行代码
/// * 🚩先跳过前缀，再执行代码
/// * 🎯用于快速识别并跳过指定前缀
/// * 🎯用于避免遗漏「跳过前缀」的操作
/// 📝`self`是一个内容相关的关键字，必须向其中传递`self`作为参数
macro_rules! first_prefix_and_skip_first {
    {
        // * 传入「self.方法名」作为被调用的方法
        $self_:ident;
        // * 传入所有的分支
        $( $prefix:expr => $branch:expr ),*,
        // * 传入「else」分支
        _ => $branch_else:expr $(,)?
    } => {
        // 插入`first!`宏中
        first! {
            $( $self_.starts_with($prefix) => {
                // ! 先跳过前缀
                $self_.head_skip($prefix);
                // * 再执行（并返回）代码
                $branch
            } ),*,
            _ => $branch_else
        }
    };
}

/// 匹配并执行第一个成功匹配的分支
/// * 🎯用于简化执行代码
///   * 📌对匹配失败者：还原头索引，并继续下一匹配
/// * 📌用于消歧义：💢「独立变量」和「预算值」开头撞了
/// * 📌用于消歧义：💢「查询变量」和「问题」标点撞了
/// 📝`self`是一个内容相关的关键字，必须向其中传递`self`作为参数
macro_rules! first_method_ok {
    // 不带「错误收集」的版本
    {
        // * 传入「self.方法名」作为「移动头索引」的方法
        $self_move:ident . $method_move:ident;
        // * 传入「当前头索引」表达式
        $original_head:expr;
        // * 传入所有的分支
        $( $condition:expr => $branch:expr ),*,
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
                        $condition
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
    // 用于在匹配时收集错误
    // * 🎯用于在解析如`( --  , 我是被否定的, 我是多余的)`的词项时，
    // *   不会只有「无条目错误」而可显示「出错之前积累的错误」
    {
        // * 传入「self.方法名」作为「移动头索引」的方法
        $self_move:ident . $method_move:ident;
        // * 传入「当前头索引」表达式
        $original_head:expr;
        // * 传入「待收集错误向量」标识符
        $to_collect:ident;
        // * 传入所有的分支
        $( $condition:expr => $branch:expr ),*,
        // * 传入「else」分支
        _ => $branch_else:expr $(,)?
    } => {
        {
            // 缓存「头索引」
            let original_head = $original_head;
            let mut result: ConsumeResult;
            // 插入`first!`宏中
            first! {
                $(
                    // 每一个条件分支
                    (
                        // 先决条件：匹配判别方法
                        $condition
                        // 后续条件：是否执行成功
                        && {
                            // 回到原始头索引
                            $self_move.$method_move(original_head);
                            // 预先计算结果
                            result = $branch;
                            // 尝试只读地匹配模式
                            match &result {
                                // 只有`Ok`能截断返回
                                Ok(_) => true,
                                // 为`Err`时，收集错误并继续匹配
                                Err(err) => {
                                    // 收集错误：追加至末尾
                                    $to_collect.push(err.to_string());
                                    // 尝试继续匹配
                                    false
                                }
                            }
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
    /*
    /// 头索引尝试跳过
    /// * 🎯用于抽象「头部索引先判断是否开头，然后跳过」的过程
    /// * 🚩逻辑：头部索引根据「是否开头」决定跳过
    ///   * 并返回一个[`ConsumeResult`]决定是否「跳过成功」
    ///   * 💭一般而言，跳过失败是需要报错的
    /// * 📌自动内联
    #[inline(always)]
    fn head_try_skip(&mut self, to_be_skip: &str, err_message: &str) -> ConsumeResult {
        // 匹配开头
        match self.starts_with(to_be_skip) {
            true => {
                self.head_skip(to_be_skip);
                Self::ok_consume()
            }
            false => self.err(err_message),
        }
    } */

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

    /// 头索引跳过某字串，连同系列空白
    /// * 🎯用于抽象「头部索引跳过字符串及之后的空白序列」的过程
    /// * 🚩逻辑：合并上述代码
    /// * 📌自动内联
    #[inline(always)]
    fn head_skip_and_spaces(&mut self, to_be_skip: &str) {
        // 跳过字符串
        self.head_skip(to_be_skip);
        // 跳过空白
        self.head_skip_spaces();
    }

    /// 头索引跳过系列空白，连同某字串
    /// * 🎯用于抽象「头部索引跳过空白序列及之后的字符串」的过程
    /// * 🚩逻辑：合并上述代码
    /// * 📌自动内联
    #[inline(always)]
    fn head_skip_after_spaces(&mut self, to_be_skip: &str) {
        // 跳过空白
        self.head_skip_spaces();
        // 跳过字符串
        self.head_skip(to_be_skip);
    }

    /// 构建「中间解析结果」/入口
    /// * 🚩核心逻辑
    ///   * 1 不断从「解析环境」中消耗文本（头部索引`head`右移）并置入「中间解析结果」中
    ///   * 2 直到「头部索引」超过文本长度（越界）
    fn build_mid_result(&mut self) -> ConsumeResult {
        // 初始化可收集的错误
        let mut errs: Vec<String> = vec![];
        // 在「可以继续消耗」时
        while self.can_consume() {
            // 索引跳过系列空白 | 用于处理对象之间的空白
            self.head_skip_spaces();
            // 仍能继续消耗⇒消耗文本
            if self.can_consume() {
                // 消耗文本&置入「中间结果」
                self.consume_one(&mut errs)?;
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
    /// * 🚩【2024-02-21 23:33:25】现在使用「匹配到就跳过」的手段
    ///   * 📌若已有词项，则一定不会再次消耗词项
    /// * 🚩现在使用「自动录入错误集」来追溯错误来源
    ///   * 📌若`errs`直接存储错误对象，会导致所有权问题（部分借用返回值）
    fn consume_one(&mut self, errs: &mut Vec<String>) -> ConsumeResult {
        first_method_ok! {
            // 当匹配失败时移回原始索引
            self.head_move;
            // 要缓存的索引
            self.head;
            // ! s要缓存进的错误集
            errs;

            // 空格⇒跳过 //
            self.starts_with(self.format.space) => Ok(self.head_skip(self.format.space)),
            // 1 预算值 //
            (
                self.starts_with(self.format.task.budget_brackets.0) &&
                self.mid_result.budget.is_none()
            ) => self.consume_budget(),
            // 2 词项 //
            (
                // ! 此处没有特别的「前缀匹配」
                self.mid_result.term.is_none()
            ) => self.consume_term(),
            // 3 标点 //
            (
                // ! 此处没有特别的「前缀匹配」 | 全靠「是否匹配成功」轮换流程
                self.mid_result.punctuation.is_none()
            ) => self.consume_punctuation(),
            // 4 时间戳 //
            (
                self.starts_with(self.format.sentence.stamp_brackets.0) &&
                self.mid_result.stamp.is_none()
            )  => self.consume_stamp(),
            // 5 真值 //
            (
                self.starts_with(self.format.sentence.truth_brackets.0) &&
                self.mid_result.truth.is_none()
            )  => self.consume_truth(),
            // 不会存在的情况 //
            _ => {
                // *【2024-02-21 23:39:30】目前选择报错
                match errs.is_empty() {
                    // 无追踪⇒直接呈现
                    true => self.err("没有可解析的条目"),
                    // 有追踪⇒链式呈现
                    false => {
                        // 链式呈现
                        self.err(&format!(
                            "没有可解析的条目 from [\n\t{}\n]",
                            errs.join("\n\t"),
                        ))
                    },
                }
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

    /// 消耗&置入/标点/判断
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn consume_punctuation_judgement(&mut self) -> ConsumeResult {
        // 索引跳过
        self.head_skip(self.format.sentence.punctuation_judgement);
        // 直接置入标点 | 因为先前`consume_one`已经假定「未曾置入标点」
        let _ = self.mid_result.punctuation.insert(Punctuation::Judgement);
        // 直接返回
        Self::ok_consume()
    }

    /// 消耗&置入/标点/目标
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn consume_punctuation_goal(&mut self) -> ConsumeResult {
        // 索引跳过
        self.head_skip(self.format.sentence.punctuation_goal);
        // 直接置入标点 | 因为先前`consume_one`已经假定「未曾置入标点」
        let _ = self.mid_result.punctuation.insert(Punctuation::Goal);
        // 直接返回
        Self::ok_consume()
    }

    /// 消耗&置入/标点/问题
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn consume_punctuation_question(&mut self) -> ConsumeResult {
        // 索引跳过
        self.head_skip(self.format.sentence.punctuation_question);
        // 直接置入标点 | 因为先前`consume_one`已经假定「未曾置入标点」
        let _ = self.mid_result.punctuation.insert(Punctuation::Question);
        // 直接返回
        Self::ok_consume()
    }

    /// 消耗&置入/标点/请求
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn consume_punctuation_quest(&mut self) -> ConsumeResult {
        // 索引跳过
        self.head_skip(self.format.sentence.punctuation_quest);
        // 直接置入标点 | 因为先前`consume_one`已经假定「未曾置入标点」
        let _ = self.mid_result.punctuation.insert(Punctuation::Quest);
        // 直接返回
        Self::ok_consume()
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
        while self.can_consume() && i < N {
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
                // 尾括弧⇒解析并存入数值&跳出循环 | 「跳出尾括弧」在循环外操作
                _ if self.starts_with(right_bracket) => {
                    // 解析并存入数值
                    match value_buffer.parse::<FloatPrecision>() {
                        // 有效数值
                        Ok(value) => {
                            // 填充数组
                            result[i] = value;
                            // 清空缓冲区
                            value_buffer.clear();
                            // 增加计数
                            i += 1;
                        }
                        // 无效数值⇒不做任何事
                        Err(_) => {}
                    }
                    // 跳出循环
                    break;
                } // 其它⇒无效字符
                c => return self.err(&format!("在解析浮点序列时出现无效字符{c:?}")),
            }
        }
        // 返回最终结果
        Ok((result, i /* 计数已在跳出时增加 */))
    }

    /// 工具函数/匹配有符号整数（`+/-` + digits）
    /// * 🎯用于解析「固定」时间戳
    /// * ⚠️非贪婪解析：解析到非法字符时停止
    /// * 📌返回值：解析出的数值
    fn parse_isize(&mut self) -> ParseResult<IntPrecision> {
        // 顺序检索
        let start = self.head;
        let mut int_buffer = String::new();
        // 逐个字符匹配
        while self.can_consume()
            && (
                // 使用`is_ascii_digit`，数值/正负号 均可 | ✅已在EVCXR中实验过
                self.head_char().is_ascii_digit() // ! 此处「混合直接匹配与带守卫匹配」导致无法使用`match`
                    || self.head_char() == '+'
                 || self.head_char() == '-'
            )
        {
            // 向目标添加字符
            int_buffer.push(self.head_char());
            // 直接递进
            self.head_step_one();
        }
        // 扫描后检查「是否有递进」 | 无递进⇒空整数值
        if self.head == start {
            return self.err("空的无符号整数值");
        }
        // 解析并存入数值
        match int_buffer.parse::<IntPrecision>() {
            // 有效数值
            Ok(value) => Self::ok(value),
            // 无效数值
            Err(_) => {
                // 无效数值
                self.err(&format!("{int_buffer:?}不是有效的数值"))
            }
        }
    }

    /// 消耗&置入/时间戳
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn consume_stamp(&mut self) -> ConsumeResult {
        // 跳过左括弧
        self.head_skip_and_spaces(self.format.sentence.stamp_brackets.0);
        // 开始匹配时间戳类型标识符
        let stamp = first_method! {
            // 前缀匹配
            self.starts_with;
            // 固定
            self.format.sentence.stamp_fixed => {
                // 跳过自身
                self.head_skip(self.format.sentence.stamp_fixed);
                // 解析&跳过 整数值
                let time = self.parse_isize()?;
                // 生成时间戳
                Stamp::Fixed(time)
            },
            // 过去
            self.format.sentence.stamp_past => {
                // 跳过自身
                self.head_skip(self.format.sentence.stamp_past);
                // 生成时间戳
                Stamp::Past
            },
            // 现在
            self.format.sentence.stamp_present => {
                // 跳过自身
                self.head_skip(self.format.sentence.stamp_present);
                // 生成时间戳
                Stamp::Present
            },
            // 未来
            self.format.sentence.stamp_future => {
                // 跳过自身
                self.head_skip(self.format.sentence.stamp_future);
                // 生成时间戳
                Stamp::Future
            },
            // 无效类型
            _ => return self.err("无效时间戳类型"),
        };
        // 置入时间戳
        let _ = self.mid_result.stamp.insert(stamp);
        // 跳过右括弧 | // ! ⚠️默认「匹配完类型后就是右括弧」
        self.head_skip_after_spaces(self.format.sentence.stamp_brackets.1);
        // 返回
        Self::ok_consume()
    }

    /// 消耗&置入/真值
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn consume_truth(&mut self) -> ConsumeResult {
        // 跳过左括弧
        self.head_skip_and_spaces(self.format.sentence.truth_brackets.0);
        let ([f, c], num) = self.parse_separated_floats::<2>(
            self.format.sentence.truth_separator,
            self.format.sentence.truth_brackets.1,
        )?;
        // 验证真值合法性
        if !f.is_in_01() || !c.is_in_01() {
            return self.err("「0-1」区间外的值（建议：`0<x<1`）");
        }
        // 构造真值
        let truth = match num {
            // 无⇒空真值
            0 => Truth::new_empty(),
            // 单⇒单真值
            1 => Truth::new_single(f),
            // 双⇒双真值
            _ => Truth::new_double(f, c),
        };
        // 跳过右括弧
        self.head_skip_after_spaces(self.format.sentence.truth_brackets.1);
        // 直接置入真值 | 因为先前`consume_one`已经假定「未曾置入真值」
        let _ = self.mid_result.truth.insert(truth);
        Self::ok_consume()
    }

    /// 消耗&置入/预算值
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn consume_budget(&mut self) -> ConsumeResult {
        // 跳过左括弧
        self.head_skip_and_spaces(self.format.task.budget_brackets.0);
        let ([p, d, q], num) = self.parse_separated_floats::<3>(
            self.format.task.budget_separator,
            self.format.task.budget_brackets.1,
        )?;
        // 验证预算值合法性
        if !p.is_in_01() || !d.is_in_01() || !q.is_in_01() {
            return self.err("「0-1」区间外的值（建议：`0<x<1`）");
        }
        // 构造预算
        let budget = match num {
            // 无⇒空预算
            0 => Budget::new_empty(),
            // 单⇒单预算
            1 => Budget::new_single(p),
            // 双⇒双预算
            2 => Budget::new_double(p, d),
            // 三⇒三预算
            _ => Budget::new_triple(p, d, q),
        };
        // 跳过右括弧
        self.head_skip_after_spaces(self.format.task.budget_brackets.1);
        // 直接置入预算值 | 因为先前`consume_one`已经假定「未曾置入预算值」
        let _ = self.mid_result.budget.insert(budget);
        Self::ok_consume()
    }

    /// 消耗&置入/词项
    /// * 🚩消耗&解析出一个词项，然后置入「中间解析结果」中
    /// * 📌需要递归解析，因此不能直接开始「置入」
    fn consume_term(&mut self) -> ConsumeResult {
        // 先解析词项
        let term = self.parse_term()?;
        // 直接置入词项 | 因为先前`consume_one`已经假定「未曾置入词项」
        let _ = self.mid_result.term.insert(term);
        Self::ok_consume()
    }

    /// 消耗&解析/词项
    /// * 🎯仍然只负责分派方法
    /// * ⚠️解析的同时跳过词项
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

    /// 工具函数：解析系列词项（并置入相应数组）
    /// * ⚠️必须保证从「可消耗的词项」开始
    ///   * ✅"term1, term2"
    ///   * ❌" term1, term2"
    /// * 📌自动内联
    #[inline(always)]
    fn parse_compound_terms(
        &mut self,
        target: &mut Vec<Term>,
        right_bracket: &str,
    ) -> ConsumeResult {
        while self.can_consume() {
            first_method! {
                // 检查开头
                self.starts_with;
                // 空白⇒跳过
                self.format.space => self.head_skip(self.format.space),
                // 分隔符⇒跳过
                self.format.compound.separator => self.head_skip(self.format.compound.separator),
                // 右括号⇒停止 // ! 跳过的逻辑交由调用者
                right_bracket => break,
                // 其它⇒尝试置入词项
                _ => target.push(
                    // 消耗&解析词项
                    self.parse_term()?,
                ),
            };
        }
        // 返回成功
        Self::ok_consume()
    }

    /// 工具函数/解析形如`{词项, 词项, ...}`的「词项集」语法
    /// * ⚠️不允许空集
    /// * 📌自动内联
    #[inline(always)]
    fn parse_term_set(
        &mut self,
        mut terms: Vec<Term>,
        left_bracket: &str,
        right_bracket: &str,
    ) -> ParseResult<Vec<Term>> {
        // 跳过左括弧&连续空白
        self.head_skip_and_spaces(left_bracket);
        // 填充词项序列
        self.parse_compound_terms(&mut terms, right_bracket)?;
        // 跳过连续空白&右括弧
        self.head_skip_after_spaces(right_bracket);
        // 判空&返回
        match terms.is_empty() {
            // 空集⇒驳回
            true => self.err("词项集为空"),
            // 非空⇒成功
            false => Self::ok(terms),
        }
    }

    /// 消耗&置入/词项/复合（外延集）
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn parse_compound_set_extension(&mut self) -> ParseResult<Term> {
        // 解析词项集&组分
        let terms = self.parse_term_set(
            vec![],
            self.format.compound.brackets_set_extension.0,
            self.format.compound.brackets_set_extension.1,
        )?; // * 📝不用考虑空间开销，编译器自己懂得内联
            // 返回成功
        Self::ok(Term::new_set_extension(terms))
    }

    /// 消耗&置入/词项/复合（内涵集）
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn parse_compound_set_intension(&mut self) -> ParseResult<Term> {
        // 解析词项集&组分
        let terms = self.parse_term_set(
            vec![],
            self.format.compound.brackets_set_intension.0,
            self.format.compound.brackets_set_intension.1,
        )?; // * 📝不用考虑空间开销，编译器自己懂得内联
            // 返回成功
        Self::ok(Term::new_set_intension(terms))
    }

    /// 工具函数/像
    /// * 🚩找到并删除首个像占位符，并返回索引
    /// * 📌自动内联
    #[inline(always)]
    fn parse_terms_with_image(&self, terms: &mut Vec<Term>) -> ParseResult<usize> {
        // 找到首个像占位符的位置
        let placeholder_index = terms.iter().position(|term| *term == Term::Placeholder);
        // 分「找到/没找到」讨论
        match placeholder_index {
            // 找到⇒删除&返回
            Some(index) => {
                // 删除此处的像占位符
                terms.remove(index);
                // 返回成功
                Self::ok(index)
            }
            // 返回失败
            None => self.err("未在词项序列中找到占位符"),
        }
    }

    /// 消耗&置入/词项/复合（括弧）
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    /// * 🚩采用「先构造词项，再填充元素」的构造方法
    ///   * ❗因为需要「根据连接符取得相应类型」且「根据后边序列取得元素」
    ///   * 📌对于「创建时就需指定所有元素」的「一元复合词项」「二元复合词项」，使用「占位符」预先占位
    fn parse_compound(&mut self) -> ParseResult<Term> {
        // 跳过左括弧&连续空白
        self.head_skip_and_spaces(self.format.compound.brackets.0);
        // 解析连接符
        let mut term = first_prefix_and_skip_first! {
            self;
            // ! 暂不支持OpenNARS风格操作
            self.format.atom.prefix_operator => return self.err("暂不支持OpenNARS风格`(^操作名, 参数)`操作，建议使用`<(*, 参数) --> 操作名>`代替"),
            // NAL-5 // ! ⚠️长的`&&`必须比短的`&`先匹配（`||`、`--`同理）
            // 合取 | 🚩空数组
            self.format.compound.connecter_conjunction => Term::new_conjunction(vec![]),
            // 析取 | 🚩空数组
            self.format.compound.connecter_disjunction => Term::new_disjunction(vec![]),
            // 否定 | 🚩使用占位符初始化，后续将被覆盖
            self.format.compound.connecter_negation => Term::new_negation(Term::new_placeholder()),
            // NAL-7 //
            // 顺序合取 | 🚩空数组
            self.format.compound.connecter_conjunction_sequential => Term::new_conjunction_sequential(vec![]),
            // 平行合取 | 🚩空数组
            self.format.compound.connecter_conjunction_parallel => Term::new_conjunction_parallel(vec![]),
            // NAL-3 //
            // 外延交 | 🚩空数组
            self.format.compound.connecter_intersection_extension => Term::new_intersection_extension(vec![]),
            // 内涵交 | 🚩空数组
            self.format.compound.connecter_intersection_intension => Term::new_intersection_intension(vec![]),
            // 外延差 | 🚩使用占位符初始化，后续将被覆盖
            self.format.compound.connecter_difference_extension => Term::new_difference_extension(Term::new_placeholder(),Term::new_placeholder()),
            // 内涵差 | 🚩使用占位符初始化，后续将被覆盖
            self.format.compound.connecter_difference_intension => Term::new_difference_intension(Term::new_placeholder(),Term::new_placeholder()),
            // NAL-4 //
            // 乘积 | 🚩空数组
            self.format.compound.connecter_product => Term::new_product(vec![]),
            // 外延像 | 🚩空数组&0索引
            self.format.compound.connecter_image_extension => Term::new_image_extension(0, vec![]),
            // 内涵像 | 🚩空数组&0索引
            self.format.compound.connecter_image_intension => Term::new_image_intension(0, vec![]),
            // 未知 //
            _ => return self.err("未知的复合词项连接符"),
        };
        // 解析组分
        let mut terms = vec![];
        self.parse_compound_terms(&mut terms, self.format.compound.brackets.1)?;
        // ! 不允许空集
        if terms.is_empty() {
            return self.err("复合词项内容不能为空");
        }
        // 填充组分 | 此处类似「针对容量」但实际上还是需要「具体类型具体填充」
        match &mut term {
            // 一元复合词项：覆盖
            Term::Negation(inner_box) => {
                // 检查长度
                if terms.len() != 1 {
                    return self.err("一元内容长度不为1");
                }
                // 解包并追加进第一个元素
                // 📝Rust支持对函数结果（只要是引用）进行「解引用赋值」
                *inner_box.as_mut() = unsafe { terms.pop().unwrap_unchecked() };
                // ! ↑SAFETY: 上方「检查长度」已确保是非空集
            }
            // 二元序列⇒覆盖 | 📌实际上「蕴含」「等价」都算
            Term::DifferenceExtension(ref1, ref2)
            | Term::DifferenceIntension(ref1, ref2)
            | Term::Inheritance(ref1, ref2)
            | Term::Implication(ref1, ref2)
            | Term::Similarity(ref1, ref2)
            | Term::Equivalence(ref1, ref2) => {
                // 检查长度
                if terms.len() != 2 {
                    return self.err("二元序列长度不为2");
                }
                // 解包并倒序追加俩元素
                // ! ↑SAFETY: 上方「检查长度」已确保是非空集
                *ref2.as_mut() = unsafe { terms.pop().unwrap_unchecked() };
                *ref1.as_mut() = unsafe { terms.pop().unwrap_unchecked() };
            }
            // 二元集合⇒清空&重新添加 | ⚠️暂时没有
            // 像：特殊处理
            Term::ImageExtension(index, vec) | Term::ImageIntension(index, vec) => {
                // 计算词项序列（提取占位符索引）
                let i = self.parse_terms_with_image(&mut terms)?;
                // 更新索引
                *index = i;
                // 追加词项
                vec.extend(terms);
            }
            // 其它（序列/集合）⇒直接添加 | 📌其一定为复合词项，但对「二元词项」会报错
            _ => {
                // 直接识别并传播错误
                if let Err(err) = term.push_components(terms) {
                    return self.err(&err.to_string());
                }
            }
        }
        // 跳过连续空白&右括弧
        self.head_skip_after_spaces(self.format.compound.brackets.1);
        // 返回
        Self::ok(term)
    }

    /// 消耗&置入/词项/陈述
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn parse_statement(&mut self) -> ParseResult<Term> {
        // 跳过左括弧&连续空白
        self.head_skip_and_spaces(self.format.statement.brackets.0);
        // 解析主词
        let subject = self.parse_term()?;
        // 跳过空白
        self.head_skip_spaces();
        // 使用闭包简化「跳过空白⇒解析谓词」的操作
        // * 💭实际上是一种「先进行后处理，然后处理中间分派的结果」的思想
        // * 📌产生原因：先根据遇到的「连接词」生成词项，然后才能解析并置入后边的谓词
        // * 📝此中不能直接捕获`self`（会捕获所有权），需要引入`Self`类型的可变引用作为参数
        //    * 保证对象安全
        let parse_predicate = |self_: &mut Self| {
            // 跳过空白
            self_.head_skip_spaces();
            // 解析谓词
            self_.parse_term()
        };
        // 解析系词
        let term = first_prefix_and_skip_first! {
            // 先匹配，然后跳过，再执行分支内的代码
            self;
            // 继承
            self.format.statement.copula_inheritance => Term::new_inheritance(subject, parse_predicate(self)?),
            // 相似
            self.format.statement.copula_similarity => Term::new_similarity(subject, parse_predicate(self)?),
            // 蕴含
            self.format.statement.copula_implication => Term::new_implication(subject, parse_predicate(self)?),
            // 等价
            self.format.statement.copula_equivalence => Term::new_equivalence(subject, parse_predicate(self)?),
            // 实例
            self.format.statement.copula_instance => Term::new_instance(subject, parse_predicate(self)?),
            // 属性
            self.format.statement.copula_property => Term::new_property(subject, parse_predicate(self)?),
            // 实例属性
            self.format.statement.copula_instance_property => Term::new_instance_property(subject, parse_predicate(self)?),
            // 预测性蕴含
            self.format.statement.copula_implication_predictive => Term::new_implication_predictive(subject, parse_predicate(self)?),
            // 并发性蕴含
            self.format.statement.copula_implication_concurrent => Term::new_implication_concurrent(subject, parse_predicate(self)?),
            // 回顾性蕴含
            self.format.statement.copula_implication_retrospective => Term::new_implication_retrospective(subject, parse_predicate(self)?),
            // 预测性等价
            self.format.statement.copula_equivalence_predictive => Term::new_equivalence_predictive(subject, parse_predicate(self)?),
            // 并发性等价
            self.format.statement.copula_equivalence_concurrent => Term::new_equivalence_concurrent(subject, parse_predicate(self)?),
            // 回顾性等价 | ⚠️会在构造时自动转换
            self.format.statement.copula_equivalence_retrospective => Term::new_equivalence_retrospective(subject, parse_predicate(self)?),
            // 未知 //
            _ => return self.err("未知的陈述系词"),
        };
        // 跳过连续空白&右括弧
        self.head_skip_after_spaces(self.format.statement.brackets.1);
        // 返回
        Self::ok(term)
    }

    /// 工具函数/判断字符是否能作为「词项名」
    /// * 🎯用于判断「合法词项名」
    #[inline(always)]
    fn is_valid_atom_name(c: char) -> bool {
        match c {
            // 特殊：横杠/下划线
            // ! ↓【2024-02-22 14:46:16】现因需兼顾`<主词-->谓词>`的结构（防止系词中的`-`被消耗），故不再兼容`-`
            /* '-' |  */
            '_' => true,
            //  否则：判断是否为「字母/数字」
            _ => c.is_alphabetic() || c.is_numeric(),
        }
    }

    /// 消耗&置入/词项/原子
    /// * 📌传入之前提：已识别出相应的「特征开头」
    /// * 📌需要在此完成专有的挪位
    fn parse_atom(&mut self) -> ParseResult<Term> {
        // 匹配并消耗前缀，并以此预置「词项」
        let mut term = first_prefix_and_skip_first! {
            self;
            // 占位符 | 此举相当于识别以「_」开头的词项
            self.format.atom.prefix_placeholder => Term::new_placeholder(),
            // 独立变量
            self.format.atom.prefix_variable_independent => Term::new_variable_independent(""),
            // 非独变量
            self.format.atom.prefix_variable_dependent => Term::new_variable_dependent(""),
            // 查询变量
            self.format.atom.prefix_variable_query => Term::new_variable_query(""),
            // 间隔
            self.format.atom.prefix_interval => Term::new_interval(0),
            // 操作符
            self.format.atom.prefix_operator => Term::new_operator(""),
            // 词语 | ⚠️必须以此兜底（空字串也算前缀）
            self.format.atom.prefix_word => Term::new_word(""),
            _ => {
                return self.err("未知的原子词项前缀")
            }
        };
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
            self.mid_result.truth.take().unwrap_or(Truth::new_empty()),
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

    /// 主解析函数
    pub fn parse_multi<'a>(&'a self, inputs: impl IntoIterator<Item=&'a str>) -> Vec<ParseResult> {
        // 构造结果
        let mut result = vec![];
        // 构造空的解析状态
        let mut state: ParseState<&str> = self.build_parse_state("");
        // 复用状态进行解析
        for input in inputs {
            // 重置状态
            state.reset_to(input, 0);
            // 添加解析结果
            result.push(state.parse());
        }
        // 返回所有结果
        result
        // ! 随后丢弃状态
    }
}

/// 单元测试
#[cfg(test)]
mod tests_parse {
    use crate::{
        conversion::string::{NarseseFormat, FORMAT_ASCII},
        fail_tests, show, Sentence, Task, Term,
    };

    use super::NarseseResult;

    /// 生成「矩阵」
    /// * 结果：`Vec<(format, Vec<result>)>`
    macro_rules! f_matrix {
        [
            $f:ident;
            $($format:expr $(,)?)+ ;
            $($input:expr $(,)?)+ $(;)?
            // *【2024-02-22 15:32:02】↑现在所有逗号都可选了
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

    /// 通通用测试/尝试解析并返回错误
    fn __test_parse(format: &NarseseFormat<&str>, input: &str) -> NarseseResult {
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

    /// 通用测试/CommonNarsese对象
    fn _test_parse_common(format: &NarseseFormat<&str>, input: &str) {
        // 尝试解析并检验
        let narsese = __test_parse(format, input);
        match narsese {
            // 词项
            NarseseResult::Term(term) => {
                // 展示
                println!("[词项] {term:#?}");
            }
            // 语句
            NarseseResult::Sentence(sentence) => {
                // 展示
                println!("[语句] {sentence:#?}");
            }
            // 任务
            NarseseResult::Task(task) => {
                // 展示
                println!("[任务] {task:#?}");
            }
        }
    }

    /// 通用测试/词项
    fn _test_parse_term(format: &NarseseFormat<&str>, input: &str) {
        // 尝试解析并检验
        let term: Term = __test_parse(format, input).try_into().unwrap();
        // 展示
        show!(term);
    }

    /// 通用测试/语句
    fn _test_parse_sentence(format: &NarseseFormat<&str>, input: &str) {
        // 尝试解析并检验
        let sentence: Sentence = __test_parse(format, input).try_into().unwrap();
        // 展示
        show!(sentence);
    }

    /// 通用测试/任务
    fn _test_parse_task(format: &NarseseFormat<&str>, input: &str) {
        // 尝试解析并检验
        let task: Task = __test_parse(format, input).try_into().unwrap();
        // 展示
        show!(task);
    }

    /// 测试/原子词项
    #[test]
    fn test_parse_atom() {
        let format_ascii = FORMAT_ASCII;
        let matrix = f_matrix! [
            // 应用的函数
            _test_parse_term;
            // 格式×输入
            &format_ascii;
            "word", "_", "$i_var", "#d_var", "?q_var", "+137", "^op",
            // "^go-to" // * ←该操作符OpenNARS可解析，而ONA、PyNARS不能
            // ! ↑【2024-02-22 14:46:16】现因需兼顾`<主词-->谓词>`的结构（防止系词中的`-`被消耗），故不再兼容
        ];
        show!(matrix);
    }

    /// 宏/统一简化生成「失败测试」
    /// * 🎯针对重复代码再优化
    /// * 📌仅需输入必要的信息
    macro_rules! fail_tests_parse {
        // 匹配表达式
        {
            // 使用的格式
            $format:ident;
            // 使用的函数
            $test_f:ident;
            // 所有情况：函数⇒被解析文本
            $($name:ident => $to_parse:expr)*
        } => {
            $(
                /// 失败测试_$name
                #[test]
                #[should_panic]
                fn $name() {
                    $test_f(&$format, $to_parse);
                }
            )*
        };
    }

    // 测试/原子词项/失败
    fail_tests_parse! {
        // 格式 & 测试函数
        FORMAT_ASCII;
        _test_parse_term;
        // 情形
        test_parse_atom_fail_未知前缀 => "@word"
        test_parse_atom_fail_未知前缀2 => "`word"
        test_parse_atom_fail_非法字符1 => ","
        test_parse_atom_fail_非法字符2 => "wo:rd"
        test_parse_atom_fail_非法字符3 => "wo[rd"
        test_parse_atom_fail_非法字符4 => "wo啊/d"
    }

    /// 测试/复合词项
    #[test]
    fn test_parse_compound() {
        let format_ascii = FORMAT_ASCII;
        let matrix = f_matrix! [
            // 应用的函数
            _test_parse_term;
            // 格式×输入
            &format_ascii;
            "{word, w2}",
            "{{word}, {w2}}",
            "{{{{{{嵌套狂魔}}}}}}",
            "[1 , 2 , 3  , 4 ,   5 ]",
            "[_ , _ , _  , _ ,   _ ]", // ! 看起来是五个，实际上因为是「集合」只有一个
            "(&, word, $i_var, #d_var, ?q_var, _, +137, ^op)",
            "(|, word, $i_var, #d_var, ?q_var, _, +137, ^op)",
            "(-, {被减的}, [减去的])",
            "(~, {[被减的]}, [{减去的}])",
            "(~, (-, 被减的被减的, {[被减的减去的]}), [{减去的}])",
            "(*, word, $i_var, #d_var, ?q_var, _, +137, ^op)",
            "(/, word, _, $i_var, #d_var, ?q_var, +137, ^op)",
            "(\\,word,$i_var,#d_var,?q_var,_,+137,^op)",
            "(/, _, 0)",
            "(\\, 0, _)",
            "( &&  , word  , $i_var  , #d_var  , ?q_var  , _  , +137  , ^op )",
            "( ||  , word  , $i_var  , #d_var  , ?q_var  , _  , +137  , ^op )",
            "( --  , 我是被否定的)",
            "( &/  , word  , $i_var  , #d_var  , ?q_var  , _  , +137  , ^op )",
            "( &|  , word  , $i_var  , #d_var  , ?q_var  , _  , +137  , ^op )",
        ];
        show!(matrix);
    }

    // 测试/复合词项/失败
    fail_tests_parse! {
        // 格式/测试函数
        FORMAT_ASCII;
        _test_parse_term;
        // 情形
        test_parse_compound_fail_唯一操作表达式 => "(^操作名, 参数)"
        test_parse_compound_fail_无起始符1 => ")"
        test_parse_compound_fail_无起始符2 => "}"
        test_parse_compound_fail_无起始符3 => "]"
        test_parse_compound_fail_无终止符1 => "("
        test_parse_compound_fail_无终止符2 => "{"
        test_parse_compound_fail_无终止符3 => "["
        test_parse_compound_fail_空_外延集 => "{}"
        test_parse_compound_fail_空_内涵集 => "[]"
        test_parse_compound_fail_空_复合词项 => "(&/, )"
        test_parse_compound_fail_多余元素_外延差 => "( -, 要被减掉, 被减掉了, 我是多余的)"
        test_parse_compound_fail_多余元素_内涵差 => "( ~, 要被减掉, 被减掉了, 我是多余的)"
        test_parse_compound_fail_缺少占位符_外延像 => "( /, 为什么, 这里没有, 占位符呢)"
        test_parse_compound_fail_缺少占位符_内涵像 => "( \\, 为什么, 这里没有, 占位符呢)"
        test_parse_compound_fail_多余元素_否定 => "( --  , 我是被否定的, 我是多余的)"
        test_parse_compound_fail_未知连接符 => "(我是未知的, word, ^op)"
    }

    /// 测试/陈述
    #[test]
    fn test_parse_statement() {
        let format_ascii = FORMAT_ASCII;
        let matrix = f_matrix! [
            // 应用的函数
            _test_parse_term;
            // 格式×输入
            &format_ascii;
            // 普通情况
            "<外延-->内涵>",
            "<我是右边的外延 --> 我是左边的内涵>",
            "<前提 ==> 结论>",
            "<等价物 <=> 等價物>",
            // 派生系词
            "<实例 {-- 类型>",
            "<类型 --] 属性>",
            "<实例 {-] 属性>",
            r#"<当下行动 =/> 未来预期>"#,
            r#"<当下条件 =|> 当下结论>"#,
            r#"<当下结果 =\> 过往原因>"#,
            r#"<统一前提 </> 未来等价>"#,
            r#"<统一前提 <|> 当下等价>"#,
            r#"<统一前提 <\> 过往等价>"#, // ! ⚠️允许出现，但会被自动转换为「未来等价」

            // 集成测试：原子&复合
            "<[蕴含]==>{怪论}>",
            "<$我很相似 <-> #我也是>",
            "<^咱俩相同<->^咱俩相同>",
            "<+123<->加一二三>",
            "<(*, {SELF}) --> ^left>",
        ];
        show!(matrix);
    }

    // 测试/陈述/失败
    fail_tests! {}

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

    // 测试/标点/失败
    fail_tests_parse! {
        // 格式/测试函数
        FORMAT_ASCII;
        _test_parse_sentence;
        // 情形
        test_parse_compound_fail_无效标点1 => "无效~"
        test_parse_compound_fail_无效标点2 => "无效`"
        test_parse_compound_fail_无效标点3 => "无效#"
        test_parse_compound_fail_无效标点4 => "无效$"
        test_parse_compound_fail_无效标点5 => "无效%"
        test_parse_compound_fail_无效标点6 => "无效^"
        test_parse_compound_fail_无效标点7 => "无效&"
        test_parse_compound_fail_无效标点8 => "无效*"
        test_parse_compound_fail_无效标点9 => "无效|"
        test_parse_compound_fail_无效标点10 => "无效\\"
        test_parse_compound_fail_无效标点11 => "无效/"
        test_parse_compound_fail_重复标点1 => "无效.."
        test_parse_compound_fail_重复标点2 => "无效!!"
        test_parse_compound_fail_重复标点3 => "无效??"
        test_parse_compound_fail_重复标点4 => "无效@@"
    }

    /// 测试/真值（语句）
    #[test]
    fn test_parse_truth() {
        let matrix = f_matrix! [
            // 应用的函数
            _test_parse_sentence;
            // 格式×输入
            &FORMAT_ASCII;
            "判断. %1.0;0.9%", "目标! %.0;.9%", "问题?", "请求@",
            "单真值. %1.0%",
            "单真值. %00%",
            "单真值. %00.00%",
            "单真值2. %.0%",
            "空真值. %%", // * 视作空真值
            "空真值2. %", // * 这个会预先退出
            "空真值3.",
        ];
        show!(matrix);
    }

    // 测试/真值/失败
    fail_tests_parse! {
        // 格式/测试函数
        FORMAT_ASCII;
        _test_parse_sentence;
        // 情形
        test_parse_truth_fail_多个量 => "A. %1;1;1%"
        test_parse_truth_fail_超范围1 => "A. %-1;1%"
        test_parse_truth_fail_超范围2 => "A. %1;-1%"
        test_parse_truth_fail_超范围3 => "A. %2;1%"
        test_parse_truth_fail_超范围4 => "A. %1;2%"
    }

    /// 测试/预算值（任务）
    #[test]
    fn test_parse_budget() {
        let matrix = f_matrix! [
            // 应用的函数
            _test_parse_task;
            // 格式×输入
            &FORMAT_ASCII;
            "$0.5;0.5;0.5$ 判断. %1.0%",
            "$.7;.75;0.555$目标! %.0;.9%",
            "$1;1;1$ 问题?",
            "$0;0;0$请求@",
            "$0;0$双预算?",
            "$0$单预算@",
            "$$空预算?",
            "$$$独立变量vs空运算?",
        ];
        show!(matrix);
    }

    // 测试/预算值/失败
    fail_tests_parse! {
        // 格式/测试函数
        FORMAT_ASCII;
        _test_parse_task;
        // 情形
        test_parse_budget_fail_多个量 => "$1;1;1;1$ A."
        test_parse_budget_fail_超范围1 => "$-1;1;1$ A."
        test_parse_budget_fail_超范围2 => "$1;-1;1$ A."
        test_parse_budget_fail_超范围3 => "$1;1;-1$ A."
        test_parse_budget_fail_超范围4 => "$2;1;1$ A."
        test_parse_budget_fail_超范围5 => "$1;2;1$ A."
        test_parse_budget_fail_超范围6 => "$1;1;2$ A."
    }

    /// 测试/时间戳（语句）
    #[test]
    fn test_parse_stamp() {
        let matrix = f_matrix! [
            // 应用的函数
            _test_parse_sentence;
            // 格式×输入
            &FORMAT_ASCII;
            "固定.:!114514:",
            "固定正.:!+137:",
            "固定负.:!-442:",
            "过去.:\\:",
            "现在? :|:",
            "未来! :/:",
            "永恒.",
        ];
        show!(matrix);
    }

    // 测试/时间戳/失败
    fail_tests_parse! {
        // 格式/测试函数
        FORMAT_ASCII;
        _test_parse_sentence;
        // 情形
        test_parse_truth_fail_无效类型1 => "A. :~:"
        test_parse_truth_fail_无效类型2 => "A. :1:"
        test_parse_truth_fail_无效类型3 => "A. :无:"
        test_parse_truth_fail_无效类型4 => "A. :`:"
        test_parse_truth_fail_无效类型5 => "A. :@:"
        test_parse_truth_fail_无效类型6 => "A. :#:"
        test_parse_truth_fail_无效类型7 => "A. :$:"
        test_parse_truth_fail_无效类型8 => "A. :%:"
        test_parse_truth_fail_无效类型9 => "A. :^:"
        test_parse_truth_fail_无效类型10 => "A. :&:"
        test_parse_truth_fail_无效类型11 => "A. :*:"
        test_parse_truth_fail_无效类型12 => "A. :(:"
        test_parse_truth_fail_无效类型13 => "A. :):"
        test_parse_truth_fail_无效类型14 => "A. :-:"
        test_parse_truth_fail_无效类型15 => "A. :_:"
        test_parse_truth_fail_无效类型16 => "A. :+:"
        test_parse_truth_fail_无效类型17 => "A. :=:"
        test_parse_truth_fail_重复类型1 => r#"A. ://:"#
        test_parse_truth_fail_重复类型2 => r#"A. :||:"#
        test_parse_truth_fail_重复类型3 => r#"A. :\\:"#
        test_parse_truth_fail_固定_无效值1 => "A. :!:"
        test_parse_truth_fail_固定_无效值2 => "A. :!1.0:"
        test_parse_truth_fail_固定_无效值3 => "A. :!--1:"
        test_parse_truth_fail_固定_无效值4 => "A. :!+:"
        test_parse_truth_fail_固定_无效值5 => "A. :!-:"
    }

    /// 通用/健壮性测试
    /// * 🎯仅用于检测是否会panic
    fn _test_parse_stability(format: &NarseseFormat<&str>, input: &str) {
        // 解析，忽略结果
        let _ = format.parse(input);
    }

    /// 集成测试/健壮性测试
    /// * 🎯用于检验是否可能panic
    #[test]
    fn test_parse_stability_cases() {
        f_matrix! [
            // 应用的函数
            _test_parse_stability;
            // 格式×输入
            &FORMAT_ASCII;
            // 多个真值/预算值 // ! 可能的数组越界
            "1. %1;1;1%"
            "$1;1;1;1$ 1."
            "$1;1;1;1;1;1;1;1;1;1;1$ 1. %1;1;1;1;1;1;1;1;1%"
        ];
    }

    /// 集成测试/解析器
    #[test]
    fn test_parse_multi() {
        let format = &FORMAT_ASCII;
        let inputs = vec![
            "<(&&, <<$x-->A>==><$x-->B>>, <<$y-->C>==><$y-->D>>) ==> E>.",
            "<{tim} --> (/,livingIn,_,{graz})>. %0%",
            "<<(*,$1,sunglasses) --> own> ==> <$1 --> [aggressive]>>.",
            "<(*,{tom},sunglasses) --> own>.",
            "<<$1 --> [aggressive]> ==> <$1 --> murder>>.",
            "<<$1 --> (/,livingIn,_,{graz})> ==> <$1 --> murder>>.",
            "<{?who} --> murder>?",
            "<{tim} --> (/,livingIn,_,{graz})>.",
            "<{tim} --> (/,livingIn,_,{graz})>. %0%",
            "<<(*,$1,sunglasses) --> own> ==> <$1 --> [aggressive]>>.",
            "<(*,{tom},(&,[black],glasses)) --> own>.",
            "<<$1 --> [aggressive]> ==> <$1 --> murder>>.",
            "<<$1 --> (/,livingIn,_,{graz})> ==> <$1 --> murder>>.",
            "<sunglasses --> (&,[black],glasses)>.",
            "<{?who} --> murder>?",
        ];
        let results = format.parse_multi(inputs);
        show!(&results);
        for result in &results {
            assert!(result.is_ok());
        }
    }

    /// 集成测试/解析器
    #[test]
    fn test_parse_integrated() {
        let matrix = f_matrix! [
            // 应用的函数
            _test_parse_common;
            // 格式×输入
            &FORMAT_ASCII;
            // 变量测试1
            "<(&&, <<$x-->A>==><$x-->B>>, <<$y-->C>==><$y-->D>>) ==> E>."
            // `long_term_stability.nal`
            "<{tim} --> (/,livingIn,_,{graz})>. %0%"
            "<<(*,$1,sunglasses) --> own> ==> <$1 --> [aggressive]>>."
            "<(*,{tom},sunglasses) --> own>."
            "<<$1 --> [aggressive]> ==> <$1 --> murder>>."
            "<<$1 --> (/,livingIn,_,{graz})> ==> <$1 --> murder>>."
            "<{?who} --> murder>?"
            "<{tim} --> (/,livingIn,_,{graz})>."
            "<{tim} --> (/,livingIn,_,{graz})>. %0%"
            "<<(*,$1,sunglasses) --> own> ==> <$1 --> [aggressive]>>."
            "<(*,{tom},(&,[black],glasses)) --> own>."
            "<<$1 --> [aggressive]> ==> <$1 --> murder>>."
            "<<$1 --> (/,livingIn,_,{graz})> ==> <$1 --> murder>>."
            "<sunglasses --> (&,[black],glasses)>."
            "<{?who} --> murder>?"
            "<(*,toothbrush,plastic) --> made_of>."
            "<(&/,<(*,$1,plastic) --> made_of>,<(*,{SELF},$1) --> ^lighter>) =/> <$1 --> [heated]>>."
            "<<$1 --> [heated]> =/> <$1 --> [melted]>>."
            "<<$1 --> [melted]> <|> <$1 --> [pliable]>>."
            "<(&/,<$1 --> [pliable]>,<(*,{SELF},$1) --> ^reshape>) =/> <$1 --> [hardened]>>."
            "<<$1 --> [hardened]> =|> <$1 --> [unscrewing]>>."
            "<toothbrush --> object>."
            "(&&,<#1 --> object>,<#1 --> [unscrewing]>)!"
            "<{SELF} --> [hurt]>! %0%"
            "<{SELF} --> [hurt]>. :|: %0%"
            "<(&/,<(*,{SELF},wolf) --> close_to>,+1000) =/> <{SELF} --> [hurt]>>."
            "<(*,{SELF},wolf) --> close_to>. :|:"
            "<(&|,<(*,{SELF},$1,FALSE) --> ^want>,<(*,{SELF},$1) --> ^anticipate>) =|> <(*,{SELF},$1) --> afraid_of>>."
            "<(*,{SELF},?what) --> afraid_of>?"
            "<a --> A>. :|: %1.00;0.90%"
            "<b --> B>. :|: %1.00;0.90%"
            "<c --> C>. :|: %1.00;0.90%"
            "<a --> A>. :|: %1.00;0.90%"
            "<b --> B>. :|: %1.00;0.90%"
            "<?1 =/> <c --> C>>?"
            "<(*,cup,plastic) --> made_of>."
            "<cup --> object>."
            "<cup --> [bendable]>."
            "<toothbrush --> [bendable]>."
            "<toothbrush --> object>."
            "<(&/,<(*,$1,plastic) --> made_of>,<(*,{SELF},$1) --> ^lighter>) =/> <$1 --> [heated]>>."
            "<<$1 --> [heated]> =/> <$1 --> [melted]>>."
            "<<$1 --> [melted]> <|> <$1 --> [pliable]>>."
            "<(&/,<$1 --> [pliable]>,<(*,{SELF},$1) --> ^reshape>) =/> <$1 --> [hardened]>>."
            "<<$1 --> [hardened]> =|> <$1 --> [unscrewing]>>."
            "(&&,<#1 --> object>,<#1 --> [unscrewing]>)!"
        ];
        show!(matrix);
    }
}
