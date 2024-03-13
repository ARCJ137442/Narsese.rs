//! 定义「获取内部元素」的特征
//! * 🎯最初用于抽象「从语句里获取词项」「从任务里获取真值」等用法

/// 用于统一获取「内部词项」
pub trait GetTerm<Term> {
    /// 获取「内部词项」
    fn get_term(&self) -> &Term;
}

/// 用于统一获取「真值」
/// * 🎯可能不一定有：for「问题/请求」
pub trait GetTruth<Truth> {
    /// 获取「真值」
    fn get_truth(&self) -> Option<&Truth>;
}

/// 用于统一获取「预算值」
pub trait GetBudget<Budget> {
    /// 获取「预算值」
    fn get_budget(&self) -> &Budget;
}

/// 用于统一获取「时间戳」
pub trait GetStamp<Stamp> {
    /// 获取「时间戳」
    fn get_stamp(&self) -> &Stamp;
}

/// 用于统一获取「标点」
pub trait GetPunctuation<Punctuation> {
    /// 获取「标点」
    fn get_punctuation(&self) -> &Punctuation;
}

// ! 📌【2024-03-03 20:40:55】暂且不定义「获取语句」的特征
//   * 原因：相比「词项」「时间戳」「真值」「预算」等，尚不通用
