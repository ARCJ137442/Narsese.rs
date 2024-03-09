//! 实现和「语句」相关的结构
//! * 🎯仅用于表征语法结构
//!   * 后续多半需要再转换
//!
//! 实现内容
//! * 真值
//! * 时间戳
//! * 语句
//!   * 标点 | 💭有些类型的语句不支持真值

// 真值 //
pub mod truth;
pub use truth::*;

// 时间戳 //
pub mod stamp;
pub use stamp::*;

// 标点 //
pub mod punctuation;
pub use punctuation::*;

// 语句 //
#[allow(clippy::module_inception)] // * 告知Clippy忽略（私有模块）
mod sentence;
pub use sentence::*;
