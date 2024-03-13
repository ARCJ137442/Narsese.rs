//! 所有「字符串转换」共用的库

// 解析格式
// * 【2024-03-13 14:42:13】最初源自enum_narsese
mod format;
pub use format::*;

// 解析格式的实例
// * 🚩目前仍作为单独的子模块导出，而**不导出其内元素**
//  * 其内元素可能会造成名称混淆
// * 📝导入并【以别名重新导出】模块，实际上不太实用
//  * 🚩此处弃用
pub mod format_instances;
// pub use format_instances as instances;
