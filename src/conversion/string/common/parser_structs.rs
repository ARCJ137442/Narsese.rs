//! 用于「集中简化」有关「Narsese解析」相关的结构
//! * 🚩基本使用泛型进行定义，以便得到最大的通用性
//! * 🎯简化「枚举Narsese」与「词法Narsese」共用的模板代码

/// 定义一个「CommonNarsese结果」类型
/// * 🎯用于存储「最终被解析出来的词法CommonNarsese对象」
///   * 词项
///   * 语句
///   * 任务
/// * 📌复制并泛化自「枚举Narsese」相应版本
///   * 🚩有关「集成统一，避免模板代码」的问题：使用**泛型**解决
///   * 🔦允许**自定义其中的「词项」「语句」「任务」类型**
///   * ✨并在后续可使用「类型别名」达到与「分别定义一个『XXNarseseResult』struct」等价的效果
#[derive(Debug, Clone)]
pub enum NarseseResult<Term, Sentence, Task> {
    /// 解析出来的词项
    Term(Term),
    /// 解析出来的语句
    Sentence(Sentence),
    /// 解析出来的任务
    Task(Task),
}
