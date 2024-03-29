//! 定义集成「词项/语句/任务」的通用Narsese枚举
//! * 🎯提供「与具体实现无关」的Narsese数据结构表征

use super::{CastToTask, GetTerm, TryCastToSentence};
use std::io::ErrorKind;

/// 定义「CommonNarsese值」类型
/// * 🎯用于存储「词项/语句/任务」三者其一
///   * 词项
///   * 语句
///   * 任务
/// * 📌复制并泛化自「枚举Narsese」相应版本，并从「解析结果」上升到「Narsese值」
///   * 🚩有关「集成统一，避免模板代码」的问题：使用**泛型**解决
///   * 🔦允许**自定义其中的「词项」「语句」「任务」类型**
///   * ✨并在后续可使用「类型别名」达到与「分别定义一个『XXNarseseResult』struct」等价的效果
/// * 🚩【2024-03-14 00:30:52】为方便外部调用，此处亦作派生处理
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NarseseValue<Term, Sentence, Task> {
    Term(Term),
    Sentence(Sentence),
    Task(Task),
}

/// ! 无法自动实现[`TryFrom`]和[`TryInto`]：违反「孤儿规则」
/// ! ⚠️亦即：禁止在泛型枚举中实现类似`impl<Term, Sentence, Task> TryFrom<NarseseValue<Term, Sentence, Task>> for Term`的代码
/// * 📝经验：尽可能不要使用「没有经过约束就应用到所有类型」的实现
impl<Term, Sentence, Task> NarseseValue<Term, Sentence, Task> {
    /// 获取名称（简体中文）
    pub(crate) fn type_name(&self) -> &str {
        match self {
            NarseseValue::Term(..) => "词项",
            NarseseValue::Sentence(..) => "语句",
            NarseseValue::Task(..) => "任务",
        }
    }

    /// 判断是否为词项
    pub fn is_term(&self) -> bool {
        matches!(self, NarseseValue::Term(..))
    }

    /// 判断是否为语句
    pub fn is_sentence(&self) -> bool {
        matches!(self, NarseseValue::Sentence(..))
    }

    /// 判断是否为任务
    pub fn is_task(&self) -> bool {
        matches!(self, NarseseValue::Task(..))
    }

    /// 尝试转换到词项
    /// * 🚩判断是否为其中的「词项」变体，然后向下转换
    ///   * 若否，则返回错误
    pub fn try_into_term(self) -> Result<Term, std::io::Error> {
        match self {
            NarseseValue::Term(term) => Ok(term),
            _ => Err(std::io::Error::new(
                ErrorKind::InvalidData,
                format!("类型「{}」不匹配，无法转换为词项", self.type_name()),
            )),
        }
    }

    /// 尝试转换到语句
    /// * 🚩判断是否为其中的「语句」变体，然后向下转换
    ///   * 若否，则返回错误
    pub fn try_into_sentence(self) -> Result<Sentence, std::io::Error> {
        match self {
            NarseseValue::Sentence(sentence) => Ok(sentence),
            _ => Err(std::io::Error::new(
                ErrorKind::InvalidData,
                format!("类型「{}」不匹配，无法转换为语句", self.type_name()),
            )),
        }
    }

    /// 尝试转换到任务
    /// * 🚩判断是否为其中的「任务」变体，然后向下转换
    ///   * 若否，则返回错误
    pub fn try_into_task(self) -> Result<Task, std::io::Error> {
        match self {
            NarseseValue::Task(task) => Ok(task),
            _ => Err(std::io::Error::new(
                ErrorKind::InvalidData,
                format!("类型「{}」不匹配，无法转换为任务", self.type_name()),
            )),
        }
    }

    /// 尝试转换到任务（兼容语句）
    /// * 🚩类似`try_into_task`，但若语句类型实现了[`CastToTask`]，则可进行自动转换
    pub fn try_into_task_compatible(self) -> Result<Task, std::io::Error>
    where
        Sentence: CastToTask<Task>,
    {
        match self {
            // 一般的「任务」：直接解包
            NarseseValue::Task(task) => Ok(task),
            // 语句：自动转换成任务
            NarseseValue::Sentence(sentence) => Ok(sentence.cast_to_task()),
            // 其他类型：报错
            _ => Err(std::io::Error::new(
                ErrorKind::InvalidData,
                format!("类型「{}」不匹配，无法转换为任务", self.type_name()),
            )),
        }
    }

    /// 从词项到Narsese值
    /// * 🚩直接打包
    ///
    /// * 📝虽说通过[`From`]实现不违反「孤儿规则」：「实现者」[`NarseseValue`]是在此定义的
    /// ! ⚠️但若继续通过[`From`]实现（代码：`impl<Term, Sentence, Task> From<Sentence> for NarseseValue<Term, Sentence, Task>`）的话，
    /// * 则「词项→Narsese值」「语句→Narsese值」「任务→Narsese值」会相互冲突
    ///   * 📌编译器无法断定「词项」「语句」「任务」三者**一定不相同**
    ///   * ❌因此可能会有「重复实现」⇒报错「冲突的实现」
    pub fn from_term(value: Term) -> Self {
        NarseseValue::Term(value)
    }

    /// 从语句到Narsese值
    /// * 🚩直接打包
    pub fn from_sentence(value: Sentence) -> Self {
        NarseseValue::Sentence(value)
    }

    /// 从任务到Narsese值
    /// * 🚩直接打包
    pub fn from_task(value: Task) -> Self {
        NarseseValue::Task(value)
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
            NarseseValue::Term(..) => Err(self),
            // 语句⇒总是成功
            NarseseValue::Sentence(..) => Ok(self),
            // 任务⇒尝试单独转换
            NarseseValue::Task(task) => match task.try_cast_to_sentence() {
                // 单独转换成功⇒作为语句封装
                Ok(sentence) => Ok(NarseseValue::Sentence(sentence)),
                // 单独转换失败⇒原样返回
                Err(task) => Err(NarseseValue::Task(task)),
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
            NarseseValue::Term(term) => term,
            // 语句⇒总是成功
            NarseseValue::Sentence(sentence) => sentence.get_term(),
            // 任务⇒尝试单独转换
            NarseseValue::Task(task) => task.get_term(),
        }
    }
}
