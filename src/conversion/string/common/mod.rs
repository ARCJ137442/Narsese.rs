//! 所有「字符串转换」共用的库

// 解析实用结构体
// * ⚠️包含如「NarseseResult」等「不同模块需要特化定义」的符号
//   * 因此不进行重导出
pub mod parser_structs;

// 字符串格式化模板
// * 进行重导出
pub mod common_narsese_templates;
pub use common_narsese_templates::*;
