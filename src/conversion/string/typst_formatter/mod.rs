//! Typst专用的格式化器
//! * ❌无法作为「Narsese格式」纳入「枚举Narsese」和「词法Narsese」：其「原子词项名称需要前后引号」不符「原子词项仅需前缀」的假设
//! * ✨对「枚举Narsese」的基本支持
//! * ❌不对「词法Narsese」提供直接支持

// 格式化器定义
mod definition;
pub use definition::*;

// 枚举Narsese格式化器
#[cfg(feature = "enum_narsese")]
mod formatter_enum;
// #[cfg(feature = "enum_narsese")]
// pub use formatter_enum::*; // * 📌【2024-04-05 19:36:33】目前仅在为「格式化器」添加方法，本身并不导出符号

// 词法Narsese格式化器
// * ⚠️【2024-04-05 20:09:45】放弃支持
//   * ℹ️详见`formatter_lexical`自身的描述
#[cfg(feature = "lexical_narsese")]
mod formatter_lexical;
// #[cfg(feature = "lexical_narsese")]
// pub use formatter_lexical::*; // * 📌【2024-04-05 19:36:33】目前仅在为「格式化器」添加方法，本身并不导出符号
