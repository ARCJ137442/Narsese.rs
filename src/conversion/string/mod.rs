//! 统一定义「字符串格式」「字符串解析器/格式化器」

// 格式（数据结构）
pub mod format;
pub use format::*;

// 实现/格式化
pub mod impl_formatter;

// 具体的格式 //
pub mod instances;
pub use instances::*;
