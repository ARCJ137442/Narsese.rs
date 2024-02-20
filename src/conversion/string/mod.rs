//! 统一定义「字符串格式」「字符串解析器/格式化器」
//!
//! 📝【2024-02-20 16:30:57】模块符号组织策略：「命名空间+局部名称」🆚「唯一名称」
//! * 第一种如`StringParser`，第二种如`impl_parser::Parser`
//! * 📄标准库大量采用名如`Iter`的结构名称
//!   * 💭而并不担心「重名冲突」
//! * 📄[tomllib/parser.rs](https://github.com/joelself/tomllib/blob/master/src/internals/parser.rs)同样采用了第二种方法
//! * 第二种设计的弊端：无法简单使用`use impl_parser::*`导入模块内容
//! * 🚩目前采用第二种组织方式
//!   * 📌一是为了**简化名称**
//!   * 📌二是第一种可以使用`use impl_parser::{Parser as StringParser}`模拟

// 格式（数据结构）
pub mod format;
pub use format::*;

// 实现/格式化
pub mod impl_formatter;

// 实现/解析器
pub mod impl_parser;

// 具体的格式 //
pub mod instances;
pub use instances::*;
