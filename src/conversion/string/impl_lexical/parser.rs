//! 实现/词法解析器
//! * 🎯字符串→词法Narsese

use crate::{
    conversion::string::common::NarseseFormat,
    lexical::{LexicalSentence, LexicalTask, LexicalTerm},
    util::{BufferIterator, IntoChars},
};
use std::{error::Error, fmt::Display, io::ErrorKind};

/// 定义一个「词法CommonNarsese结果」类型
/// * 🎯用于存储「最终被解析出来的词法CommonNarsese对象」
///   * 词项
///   * 语句
///   * 任务
/// * 📌复制并修改自EnumNarsese相应版本
///   * ❓后续是否集成统一
#[derive(Debug, Clone)]
pub enum LexicalNarseseResult {
    /// 解析出来的词项
    Term(LexicalTerm),
    /// 解析出来的语句
    Sentence(LexicalSentence),
    /// 解析出来的任务
    Task(LexicalTask),
}

// 实现`(try_)From/To`转换方法
// * 📌目前只需要「词法解析结果→词项/语句/任务」而无需其它做法
impl TryFrom<LexicalNarseseResult> for LexicalTerm {
    type Error = std::io::Error;
    fn try_from(value: LexicalNarseseResult) -> Result<Self, Self::Error> {
        match value {
            LexicalNarseseResult::Term(term) => Ok(term),
            _ => Err(Self::Error::new(
                ErrorKind::InvalidData,
                format!("类型不匹配，无法转换为词项：{value:?}"),
            )),
        }
    }
}
impl TryFrom<LexicalNarseseResult> for LexicalSentence {
    type Error = std::io::Error;
    fn try_from(value: LexicalNarseseResult) -> Result<Self, Self::Error> {
        match value {
            LexicalNarseseResult::Sentence(sentence) => Ok(sentence),
            _ => Err(Self::Error::new(
                ErrorKind::InvalidData,
                format!("类型不匹配，无法转换为语句：{value:?}"),
            )),
        }
    }
}
impl TryFrom<LexicalNarseseResult> for LexicalTask {
    type Error = std::io::Error;
    fn try_from(value: LexicalNarseseResult) -> Result<Self, Self::Error> {
        match value {
            LexicalNarseseResult::Task(task) => Ok(task),
            _ => Err(Self::Error::new(
                ErrorKind::InvalidData,
                format!("类型不匹配，无法转换为任务：{value:?}"),
            )),
        }
    }
}

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

/// 词法Narsese的「解析状态」
/// * 其中的`C`一般为「字符」
/// * 其中的`T`一般为「文本」（字符串）
pub struct ParseState<'a, C, T> {
    /// 引用的「解析格式」
    format: &'a NarseseFormat<T>,
    /// 内置的「缓冲迭代器」
    /// * 🚩使用[`Box`]封装原始迭代器
    iter: BufferIterator<C, Box<dyn Iterator<Item = C> + 'a>>,
}

/// 通用实现
impl<'a, Item, Text> ParseState<'a, Item, Text> {
    /// 构造函数
    /// * 🚩传入迭代器进行构造
    pub fn new(format: &'a NarseseFormat<Text>, iter: impl Iterator<Item = Item> + 'a) -> Self {
        Self {
            format,
            iter: BufferIterator::new(Box::new(iter)),
        }
    }

    /// 快捷构造解析结果/Ok
    pub fn ok<T>(value: T) -> ParseResult<T> {
        ParseResult::Ok(value)
    }
}

/// 字符实现
/// * 🚩解析逻辑正式开始
impl<'a> ParseState<'a, char, &str> {
    /// 快速构造解析结果/Err
    pub fn err(&self, message: &str) -> ParseResult {
        Err(ParseError::new(
            // 传入的错误消息
            message,
            // 自身缓冲区内容
            self.iter.buffer_iter().copied().collect(),
            // 自身缓冲区头索引（相对滞后）
            self.iter.buffer_head(),
        ))
    }

    /// 🔦入口
    /// * 🚩使用自身（从迭代器中）解析出一个结果
    pub fn parse(&mut self) -> ParseResult {
        // 用状态进行解析
        todo!("开发中！") // TODO: 前缀匹配+缓冲区捕获 思路
    }
}

/// 总定义
impl NarseseFormat<&str> {
    /// 构造解析状态
    /// * 索引默认从开头开始
    pub fn build_parse_state_lexical<'a>(
        &'a self,
        input: impl IntoIterator<Item = char> + 'a,
    ) -> ParseState<'a, char, &str> {
        ParseState::new(self, input.into_iter())
    }

    /// 主解析函数@字符串
    pub fn parse_lexical(&self, input: &str) -> ParseResult {
        // 转发到（有所有权的）迭代器
        self.parse_lexical_from_iter(input.into_chars())
    }

    /// 主解析函数@迭代器
    /// * 🚩从一个字符迭代器开始解析
    /// * 📝放弃使用类似`trait CanLexicalParse`的「方法重载」架构
    ///   * ❌无法解决的冲突：trait无法同时对「所有实现了某特征的类型」和「特别指定的类型」实现
    ///     * 📄case：字符串🆚字符迭代器
    ///     * 📌原因：有可能「某特征」会在其它地方对「特别指定的类型」进行实现，这时候分派方法就会出歧义（走「通用」还是「专用」？）
    ///     * 💭Julia的多分派借「层级类型系统」选择了「偏袒特定类型」的方案，但Rust不同
    pub fn parse_lexical_from_iter(&self, input: impl Iterator<Item = char>) -> ParseResult {
        // 构造解析状态
        let iter_char: Box<dyn Iterator<Item = char>> = Box::new(input);
        let mut state = self.build_parse_state_lexical(iter_char);
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
    fn __test_parse(format: &NarseseFormat<&str>, input: &str) -> LexicalNarseseResult {
        // 解析
        let result = format.parse_lexical(input);
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
