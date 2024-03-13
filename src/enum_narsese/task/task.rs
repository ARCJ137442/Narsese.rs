//! 统一定义「任务」
use crate::api::{GetBudget, GetPunctuation, GetStamp, GetTerm, GetTruth};
use crate::enum_narsese::sentence::{Punctuation, Sentence, Stamp, Truth};
use crate::enum_narsese::term::Term;

use super::*;

/// 直接用元组结构体定义「任务」
/// * 📌包含关系足够简单
#[derive(Debug, Clone, PartialEq)]
pub struct Task(Sentence, Budget);

// 实现/构造
impl Task {
    /// 构造函数
    pub fn new(sentence: Sentence, budget: Budget) -> Self {
        Task(sentence, budget)
    }
}

// 实现/属性 //
impl Task {
    /// 获取内部语句
    pub fn get_sentence(&self) -> &Sentence {
        &self.0
    }
}

impl GetBudget<Budget> for Task {
    /// 获取内部预算值
    fn get_budget(&self) -> &Budget {
        &self.1
    }
}

impl GetTerm<Term> for Task {
    /// 获取内部词项
    fn get_term(&self) -> &Term {
        self.get_sentence().get_term()
    }
}

impl GetPunctuation<Punctuation> for Task {
    /// 获取内部标点
    fn get_punctuation(&self) -> &Punctuation {
        self.get_sentence().get_punctuation()
    }
}

impl GetStamp<Stamp> for Task {
    /// 获取内部时间戳
    fn get_stamp(&self) -> &Stamp {
        self.get_sentence().get_stamp()
    }
}

impl GetTruth<Truth> for Task {
    /// 获取内部真值（不一定有）
    fn get_truth(&self) -> Option<&Truth> {
        self.get_sentence().get_truth()
    }
}
