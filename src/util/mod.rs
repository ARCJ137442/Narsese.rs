//! 一些实用工具、定义、函数
//! * 📌宏定义专门放在[`macros.rs`]中
//!   * 📄参考标准库与其它包（如`winnow`）

// 浮点
mod floats;
pub use floats::*;

// 字符串处理
mod str_process;
pub use str_process::*;

// 迭代器
mod iterators;
pub use iterators::*;
