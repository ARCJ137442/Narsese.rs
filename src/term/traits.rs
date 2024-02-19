//! 统一定义词项特征
use super::Term;

/// 用于统一获取「内部词项」
pub trait GetTerm {
    /// 获取「内部词项」
    fn get_term(&self) -> &Term;
}
