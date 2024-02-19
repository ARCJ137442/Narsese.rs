//! 统一定义「语句」
//!
//! 📌分类
//! * 判断
//! * 目标
//! * 问题
//! * 请求

use super::*;
use crate::{GetTerm, Term};

/// 使用枚举定义的「语句」类型
pub enum Sentence {
    /// 判断
    Judgement(Term, Truth, Stamp),
    /// 目标
    Goal(Term, Truth, Stamp),
    /// 问题
    Question(Term, Stamp),
    /// 请求
    Quest(Term, Stamp),
}

pub use Sentence::*;

// 💭无需实现特别的「构造函数」：足够简单

/// 实现/属性
impl GetTerm for Sentence {
    /// 获取内部词项
    fn get_term(&self) -> &Term {
        match self {
            Sentence::Judgement(term, _, _)
            | Sentence::Goal(term, _, _)
            | Sentence::Question(term, _)
            | Sentence::Quest(term, _) => term,
        }
    }
}
