//! 定义集成「词项/语句/任务」的通用Narsese枚举
//! * 🎯提供「与具体实现无关」的Narsese数据结构表征

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
            Self::Term(..) => "词项",
            Self::Sentence(..) => "语句",
            Self::Task(..) => "任务",
        }
    }

    /// 判断是否为词项
    pub fn is_term(&self) -> bool {
        matches!(self, Self::Term(..))
    }

    /// 判断是否为语句
    pub fn is_sentence(&self) -> bool {
        matches!(self, Self::Sentence(..))
    }

    /// 判断是否为任务
    pub fn is_task(&self) -> bool {
        matches!(self, Self::Task(..))
    }

    /// 尝试转换到词项
    /// * 🚩判断是否为其中的「词项」变体，然后向下转换
    ///   * 若否，则返回错误
    pub fn try_into_term(self) -> Result<Term, std::io::Error> {
        match self {
            Self::Term(term) => Ok(term),
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
            Self::Sentence(sentence) => Ok(sentence),
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
            Self::Task(task) => Ok(task),
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
        Self::Term(value)
    }

    /// 从语句到Narsese值
    /// * 🚩直接打包
    pub fn from_sentence(value: Sentence) -> Self {
        Self::Sentence(value)
    }

    /// 从任务到Narsese值
    /// * 🚩直接打包
    pub fn from_task(value: Task) -> Self {
        Self::Task(value)
    }
}
