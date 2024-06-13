//! 库的主模块

// ! 🚩【2024-06-13 19:44:25】现已删除「工具宏」定义，以减少`util`/`crate::util`歧义

// 共用API //
pub mod api;

// 枚举Narsese //
#[cfg(feature = "enum_narsese")]
pub mod enum_narsese;

// 词法Narsese //
#[cfg(feature = "lexical_narsese")]
pub mod lexical;

// 转换 //
pub mod conversion;
