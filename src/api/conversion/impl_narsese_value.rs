//! 为「Narsese值」实现所有有关「转换」的API
//! * 🎯将「数据结构定义」和「具体方法实现」分离
//!   * 🎯避免「循环依赖」发生
use crate::api::{CastToTask, FormatTo, GetTerm, NarseseValue, TryCastToSentence};
use std::io::{Error as IoError, ErrorKind};

/// 继续实现有关「转换」的API函数
/// * 🎯数据结构与功能实现分离
impl<Term, Sentence, Task> NarseseValue<Term, Sentence, Task> {
    /// 尝试转换到任务（兼容语句）
    /// * 🚩类似`try_into_task`，但若语句类型实现了[`CastToTask`]，则可进行自动转换
    pub fn try_into_task_compatible(self) -> Result<Task, IoError>
    where
        Sentence: CastToTask<Task>,
    {
        match self {
            // 一般的「任务」：直接解包
            Self::Task(task) => Ok(task),
            // 语句：自动转换成任务
            Self::Sentence(sentence) => Ok(sentence.cast_to_task()),
            // 其他类型：报错
            _ => Err(IoError::new(
                ErrorKind::InvalidData,
                format!("类型「{}」不匹配，无法转换为任务", self.type_name()),
            )),
        }
    }
}

/// 对所有「其中的『任务』类型实现了『尝试转换到语句』特征」的「Narsese值」实现「尝试转换（其中的）任务到语句」
impl<Term, Sentence, Task> TryCastToSentence<NarseseValue<Term, Sentence, Task>>
    for NarseseValue<Term, Sentence, Task>
where
    Task: TryCastToSentence<Sentence>,
{
    fn try_cast_to_sentence(
        self,
    ) -> Result<NarseseValue<Term, Sentence, Task>, NarseseValue<Term, Sentence, Task>> {
        match self {
            // 词项⇒总是失败
            Self::Term(..) => Err(self),
            // 语句⇒总是成功
            Self::Sentence(..) => Ok(self),
            // 任务⇒尝试单独转换
            Self::Task(task) => match task.try_cast_to_sentence() {
                // 单独转换成功⇒作为语句封装
                Ok(sentence) => Ok(Self::Sentence(sentence)),
                // 单独转换失败⇒原样返回
                Err(task) => Err(Self::Task(task)),
            },
        }
    }
}

/// 对所有「实现了『获取内部词项』特征的Narsese值」实现「获取内部词项」
/// * 📌原理：不论是「词项」「语句」还是「任务」，都实现了「获取内部词项」
impl<Term, Sentence, Task> GetTerm<Term> for NarseseValue<Term, Sentence, Task>
where
    Sentence: GetTerm<Term>,
    Task: GetTerm<Term>,
{
    fn get_term(&self) -> &Term {
        match self {
            // 词项⇒总是失败
            Self::Term(term) => term,
            // 语句⇒总是成功
            Self::Sentence(sentence) => sentence.get_term(),
            // 任务⇒尝试单独转换
            Self::Task(task) => task.get_term(),
        }
    }
}

// ! ❌不适宜对`NarseseValue`实现`FromParse`特征
// * 📌解析可能有多种结果，即便可以最后转换成Narsese值，最初也无法选择「向哪个子类型解析」
// impl<'a, Term, Sentence, Task, Parser> FromParse<&'a str, Parser>
// for NarseseValue<Term, Sentence, Task>
// where
//     Term: FromParse<&'a str, Parser>,
//     Sentence: FromParse<&'a str, Parser>,
//     Task: FromParse<&'a str, Parser>

/// 为「三种子类都实现『格式化』」的「Narsese值」自动实现「格式化到」特征
/// * 📝格式化可以通过「变种分派」的方式批量实现
impl<Term, Sentence, Task, Formatter, Target> FormatTo<Formatter, Target>
    for NarseseValue<Term, Sentence, Task>
where
    Term: FormatTo<Formatter, Target>,
    Sentence: FormatTo<Formatter, Target>,
    Task: FormatTo<Formatter, Target>,
{
    fn format_to(&self, formatter: Formatter) -> Target {
        // 根据自身变种转发
        match self {
            Self::Term(term) => term.format_to(formatter),
            Self::Sentence(sentence) => sentence.format_to(formatter),
            Self::Task(task) => task.format_to(formatter),
        }
    }
}
