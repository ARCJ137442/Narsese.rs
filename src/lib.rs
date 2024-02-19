//! 使用枚举简单实现一个Narsese数据结构库
//! 三种数据结构
//! * 词项（首要）
//! * 语句（封装）
//! * 任务（封装）
//!
//! ⚠️【2024-02-19 10:58:46】暂不考虑通用性，仅考虑「MWE」

// 实用
pub mod util;

// 词项
pub mod term;
pub use term::*;

// 语句
pub mod sentence;
pub use sentence::*;

// 任务
pub mod task;
pub use task::*;

// 转换
pub mod conversion;

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;

    #[test]
    fn main() {}
}
