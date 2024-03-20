//! 统一定义「标点」
//! * 🎯可以和「语句」对象相互转换
//! * ⚠️不直接出现在「语句」中，而是作为「语句」的枚举项出现
//!
//! 📌分类
//! * 判断
//! * 目标
//! * 问题
//! * 请求

/// 基于枚举定义的「标点」
/// * 有关转换交由[`super::Sentence`]实现
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Punctuation {
    /// 判断
    Judgement,
    /// 目标
    Goal,
    /// 问题
    Question,
    /// 请求
    Quest,
}
