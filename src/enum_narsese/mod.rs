//! 使用枚举简单实现一个Narsese数据结构库
//! 三种数据结构
//! * 词项（首要）
//! * 语句（封装）
//! * 任务（封装）

// 词项
pub mod term;
pub use term::*;

// 语句
pub mod sentence;
pub use sentence::*;

// 任务
pub mod task;
pub use task::*;