//! 统一定义「任务」

use crate::term::{GetTerm, Term};

use super::*;

/// 直接用元组结构体定义「任务」
/// * 📌包含关系足够简单
#[derive(Debug, Clone, PartialEq)]
pub struct Task(Sentence, Budget);

/// 实现/属性
impl Task {
    /// 构造函数
    pub fn new(sentence: Sentence, budget: Budget) -> Self {
        Task(sentence, budget)
    }

    /// 获取内部语句
    pub fn get_sentence(&self) -> &Sentence {
        &self.0
    }

    /// 获取内部预算值
    pub fn get_budget(&self) -> &Budget {
        &self.1
    }
}

impl GetTerm for Task {
    /// 获取内部词项
    fn get_term(&self) -> &Term {
        self.get_sentence().get_term()
    }
}