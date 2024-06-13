//! 库的主模块

// 实用 | 包括工具宏
// * 🚩对于在「后续实现中需要调用`util`库中的符号」的情况：
//   * 【统一使用`util`而非`crate::util`】
// * 📝↓此处进行了三个操作：导入外部库、改名、重新导出
// TODO: 【2024-06-13 19:44:25】删除，以减少`util`/`crate::util`歧义
extern crate nar_dev_utils as util;

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
