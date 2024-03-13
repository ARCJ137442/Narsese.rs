//! 实现和「语句」相关的结构，并统一定义「语句」
//! * 🎯仅用于表征语法结构
//!   * 后续多半需要再转换
//!
//! 实现内容
//! * 真值
//! * 时间戳
//! * 语句
//!   * 标点 | 💭有些类型的语句不支持真值
//!
//! 📌语句的分类
//! * 判断
//! * 目标
//! * 问题
//! * 请求

// 真值 //
pub mod truth;
pub use truth::*;

// 时间戳 //
pub mod stamp;
pub use stamp::*;

// 标点 //
pub mod punctuation;
pub use punctuation::*;

// 语句 //
// * 🚩【2024-03-13 21:27:46】现在直接将内部的`sentence`进行内联，以彻底避免「重复重名路径」麻烦
//   * 📌即便屏蔽了Clippy的提示，问题在「IDE展示模块路径」以及[`std::any::get_type_id`]中仍然存在
use crate::api::{GetPunctuation, GetStamp, GetTerm, GetTruth};
use crate::enum_narsese::term::Term;

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
    /// 构造函数/从标点构造
    /// * 🚩若需明确真值，不如直接使用下边的专用构造函数
    /// * 此中真值在「无真值的语句类型」中会被舍去
    pub fn from_punctuation(
        term: Term,
        punctuation: Punctuation,
        stamp: Stamp,
        truth: Truth,
    ) -> Self {
        match punctuation {
            // 需要真值的
            Punctuation::Judgement => Judgement(term, truth, stamp),
            Punctuation::Goal => Goal(term, truth, stamp),
            // 无需真值的
            Punctuation::Question => Question(term, stamp),
            Punctuation::Quest => Quest(term, stamp),
        }
    }

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

// 实现/属性 //

impl GetTerm<Term> for Sentence {
    /// 获取内部词项
    fn get_term(&self) -> &Term {
        match self {
            Judgement(term, _, _) | Goal(term, _, _) | Question(term, _) | Quest(term, _) => term,
        }
    }
}

impl GetPunctuation<Punctuation> for Sentence {
    /// 获取内部标点
    fn get_punctuation(&self) -> &Punctuation {
        match self {
            Judgement(..) => &Punctuation::Judgement,
            Goal(..) => &Punctuation::Goal,
            Question(..) => &Punctuation::Question,
            Quest(..) => &Punctuation::Quest,
        }
    }
}

impl GetTruth<Truth> for Sentence {
    /// 获取内部真值（不一定有）
    fn get_truth(&self) -> Option<&Truth> {
        match self {
            // 判断 | 目标 ⇒ 有真值
            Judgement(_, truth, _) | Goal(_, truth, _) => Some(truth),
            // 问题 | 请求 ⇒ 无真值
            Question(..) | Quest(..) => None,
        }
    }
}

impl GetStamp<Stamp> for Sentence {
    /// 获取内部时间戳
    fn get_stamp(&self) -> &Stamp {
        match self {
            Judgement(_, _, stamp) | Goal(_, _, stamp) | Question(_, stamp) | Quest(_, stamp) => {
                stamp
            }
        }
    }
}
