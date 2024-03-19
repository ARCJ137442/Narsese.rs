//! 向「枚举Narsese」的折叠
#![allow(unused_imports)]

use super::*;
use crate::{
    enum_narsese::{Narsese as EnumNarsese, Term as EnumTerm},
    lexical::{Narsese, Term},
};

// 实现/原子词项
// impl FoldInto<EnumTerm> for Term {
//     // TODO: 需要一个「折叠格式」做支持
// }

// impl FoldInto<EnumNarsese> for Narsese {
//     fn fold_into(self) -> EnumNarsese {
//         match self {
//             Narsese::Atom(a) => a.fol
//         }
//     }
// }
