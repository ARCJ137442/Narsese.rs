//! 词法Narsese的Typst格式化
//! * ❌【2024-04-05 19:56:02】废弃：词法Narsese无法有效承载「语义信息」
//!   * ℹ️词法Narsese仅提供「词法层面的内容」，而不提供「语义层面的信息」
//!     * ❌因此无法将「原子词项前缀」「复合词项连接词」「陈述系词」等元素与Typst公式一一对应
//!     * 📄即便知道「陈述系词」是`"-->"`，除非联动「枚举Narsese」，也不能就直接映射到`arrow.r`
//! * 📌【2024-04-05 20:08:40】只有「枚举Narsese」才能真正提供「语义支持」
//!   * ❗无需顾忌「哪种陈述系词对应哪个Typst公式」：如`"==>"`🆚`"=/>"`
//!   * ❗无需顾忌「一种陈述系词在各个Narsese格式中如何表示」：如`-->`🆚`是`
//!   * ❌相比之下，词法Narsese中`"-->"`和`"是"`不是同一种系词——即便语义相同
//! * ❓几个可能的替代使用方案
//!   * 🔦「词法折叠」方法：尝试折叠到「枚举Narsese」，再格式化为Typst公式
//!   * 🔦「尽可能回归枚举Narsese」方法：尽可能映射到「枚举Narsese」的情况
//!     * 建立「原子词项前缀/复合词项连接词/陈述系词 → Typst公式」的映射
//!     * 若在映射表内，将其特别转换为Typst公式
//!     * 若不在映射表内，使用默认转换方式

// use super::FormatterTypst;
// use crate::{api::FormatTo, lexical::Term};
// use util::ToDebug;

// /// 【占位符】将「需要转换为Typst公式的内容」转换为Typst公式
// /// * 🚩【2024-04-05 19:45:50】目前仅将其稍作「引用」处理
// ///   * 📌附带转义
// ///   * 🎯仅要求其能在Typst处正常显示
// fn to_typst(s: &str) -> String {
//     s.to_debug()
// }

// /// 格式化/词项
// impl FormatTo<&FormatterTypst, String> for Term {
//     fn format_to(&self, formatter: &FormatterTypst) -> String {
//         match self {
//             Term::Atom { prefix, name } => format!("{} {}", to_typst(prefix), name),
//             Term::Compound { connecter, terms } => todo!(),
//             Term::Set {
//                 left_bracket,
//                 terms,
//                 right_bracket,
//             } => todo!(),
//             Term::Statement {
//                 copula,
//                 subject,
//                 predicate,
//             } => todo!(),
//         }
//     }
// }
