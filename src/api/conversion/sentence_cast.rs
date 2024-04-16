//! 用于（隐式）将语句转换为任务
//! * 🎯一般用于「缺省预算值=空预算↔语句」的「语句≈任务」兼容
//!   * 📄NAVM中「语句」的输入（在字符串上）亦为合法，但需要在词法上通过「空预算」进行兼容

/// 特征：将语句转换为任务
pub trait CastToTask<Task> {
    /// 将语句转换为任务
    /// * 🎯用于特定情况下「语句」对「空预算（采用默认预算值）任务」的等价自动转换
    fn cast_to_task(self) -> Task;
}

/// 特征：尝试将任务转换为语句
/// * 🎯用于NAVM后续「因`$$`无法通过CIN解析，需要将任务隐式转换为语句」的情况
/// * ⚠️需要[`Result`]存储失败（非空值）情况
/// * 🚩两个功能
///   * ✨内部「空预算任务→语句」的尝试（手动实现）
///   * ✨Narsese值中「空预算任务→语句」的尝试（自动实现）
pub trait TryCastToSentence<Sentence>
where
    Self: Sized,
{
    /// 将任务转换为语句
    fn try_cast_to_sentence(self) -> Result<Sentence, Self>;
}
