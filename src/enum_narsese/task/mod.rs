//! 实现和「任务」相关的结构
//! * 🎯仅用于表征语法结构
//!   * 后续多半需要再转换
//!
//! 实现内容
//! * 预算值
//! * 任务
//!
//! * 🚩【2024-03-20 02:11:05】现在内联`task`同名子模块，缩减规模并明确名称

// 预算值 //

mod budget;
pub use budget::*;

// 任务 //

use crate::api::{
    CastToTask, GetBudget, GetPunctuation, GetStamp, GetTerm, GetTruth, TryCastToSentence,
};
use crate::enum_narsese::sentence::{Punctuation, Sentence, Stamp, Truth};
use crate::enum_narsese::term::Term;

/// 直接用元组结构体定义「任务」
/// * 📌包含关系足够简单
/// * 🚩【2024-03-24 02:27:18】现在同[`Sentence`]，所有字段均开放
#[derive(Debug, Clone, PartialEq)]
pub struct Task(pub Sentence, pub Budget);

/// 实现/构造
impl Task {
    /// 构造函数
    pub fn new(sentence: Sentence, budget: Budget) -> Self {
        Task(sentence, budget)
    }
}

// 实现/转换 //
impl CastToTask<Task> for Sentence {
    /// 转换：默认加上空预算
    fn cast_to_task(self) -> Task {
        Task::new(self, Budget::Empty)
    }
}

impl TryCastToSentence<Sentence> for Task {
    /// 尝试（无损）转换为语句
    fn try_cast_to_sentence(self) -> Result<Sentence, Self> {
        match self.1.is_empty() {
            // 空预算⇒可无损转换
            true => Ok(self.0),
            // 其它⇒无法转换
            false => Err(self),
        }
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
