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
///
/// ! 📌【2024-02-20 02:37:35】此处不派生[`Eq`]是因为[`f64`]没派生[`Eq`]
#[derive(Debug, Clone, PartialEq)]
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

/// 实现/构造
impl Sentence {
    /// 构造函数/判断
    pub fn new_judgement(term: Term, truth: Truth, stamp: Stamp) -> Self {
        Judgement(term, truth, stamp)
    }

    /// 构造函数/目标
    pub fn new_goal(term: Term, truth: Truth, stamp: Stamp) -> Self {
        Goal(term, truth, stamp)
    }

    /// 构造函数/问题
    pub fn new_question(term: Term, stamp: Stamp) -> Self {
        Question(term, stamp)
    }

    /// 构造函数/请求
    pub fn new_quest(term: Term, stamp: Stamp) -> Self {
        Quest(term, stamp)
    }
}


/// 实现/属性
impl Sentence {
    /// 获取内部时间戳
    pub fn get_stamp(&self) -> &Stamp {
        match self {
            Sentence::Judgement(_, _, stamp)
            | Sentence::Goal(_, _, stamp)
            | Sentence::Question(_, stamp)
            | Sentence::Quest(_, stamp) => stamp,
        }
    }

    /// 获取内部真值（不一定有）
    pub fn get_truth(&self) -> Option<&Truth> {
        match self {
            // 判断 | 目标 ⇒ 有真值
            Sentence::Judgement(_, truth, _) | Sentence::Goal(_, truth, _) => Some(truth),
            // 问题 | 请求 ⇒ 无真值
            Sentence::Question(..) | Sentence::Quest(..) => None,
        }
    }
}
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
