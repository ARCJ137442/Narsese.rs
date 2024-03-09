//! 实现/词法解析器
//! * 🎯字符串→词法Narsese

use std::{error::Error, fmt::Display};

use crate::{
    lexical::{LexicalSentence, LexicalTask, LexicalTerm},
    util::BufferIterator,
};

use super::NarseseFormat;

/// 定义一个「词法CommonNarsese结果」类型
/// * 🎯用于存储「最终被解析出来的词法CommonNarsese对象」
///   * 词项
///   * 语句
///   * 任务
#[derive(Debug, Clone)]
pub enum LexicalNarseseResult {
    /// 解析出来的词项
    Term(LexicalTerm),
    /// 解析出来的语句
    Sentence(LexicalSentence),
    /// 解析出来的任务
    Task(LexicalTask),
}

/// 用于表征「解析环境」
/// * 具有所有权
type ParseEnv<T = char> = Vec<T>; // TODO: 改为「字符缓冲迭代器」
/// 用于表征「解析索引」
type ParseIndex = usize;

/// 用于表征「解析结果」
/// * 用于表示「解析对象」
///
/// ! 📝原先基于「返回『(解析出的对象, 下一起始索引)』」的方法已无需使用
/// * 现在是基于「解析器状态」的「状态机模型」
///   * 📌关键差异：附带可设置的「中间解析结果」与「可变索引」
///   * 🚩子解析函数在解析之后，直接填充「中间解析结果」并修改「可变索引」
type ParseResult<T = LexicalNarseseResult> = Result<T, ParseError>;
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
    env_slice: String,
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
            env_slice: todo!(),
            // env_slice: ParseError::generate_env_slice(env, index),
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
        write!(f, "Narsese解析错误：{} @ {}", self.message, self.index,)
    }
}
impl Error for ParseError {}

/// 词法Narsese的「解析状态」
/// * 其中的`C`一般为「字符」
pub struct ParseState<'a, C> {
    /// 内置的「缓冲迭代器」
    /// * 🚩使用[`Box`]封装原始迭代器
    iter: BufferIterator<C, Box<dyn Iterator<Item = C> + 'a>>,
}

/// 通用实现
impl<'a, C> ParseState<'a, C> {
    /// 构造函数
    /// * 🚩传入迭代器进行构造
    pub fn new(iter: impl Iterator<Item = C> + 'a) -> Self {
        Self {
            iter: BufferIterator::new(Box::new(iter)),
        }
    }
}

/// 字符实现
impl<'a> ParseState<'a, char> {
    pub fn parse(&mut self) -> ParseResult {
        // 用状态进行解析
        todo!()
    }
}

/// 总定义
impl NarseseFormat<&str> {
    /// 构造解析状态
    /// * 索引默认从开头开始
    pub fn build_parse_state_lexical<'a>(&'a self, input: &'a str) -> ParseState<'a, &str> {
        // ParseState::new(self, input, 0)
        todo!()
    }

    /// 主解析函数
    /// TODO: 使用[`IntoIterator`]
    pub fn parse_lexical<'a>(&'a self, input: &'a str) -> ParseResult {
        // 构造解析状态
        let mut state: ParseState<char> = self.build_parse_state_lexical(input);
        // 用状态进行解析
        state.parse()
        // ! 随后丢弃状态
    }

    /// 主解析函数
    pub fn parse_lexical_multi<'a>(
        &'a self,
        inputs: impl IntoIterator<Item = &'a str>,
    ) -> Vec<ParseResult> {
        // 构造结果
        let mut result = vec![];
        // 构造空的解析状态
        let mut state: ParseState<&str> = self.build_parse_state_lexical("");
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
