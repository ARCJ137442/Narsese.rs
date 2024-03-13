use crate::api::{GetBudget, GetPunctuation, GetStamp, GetTerm, GetTruth};

use super::{LexicalSentence, LexicalTerm};

/// 词法上的「任务」：预算值+语句
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexicalTask {
    budget: String,
    sentence: LexicalSentence,
}

/// 自身方法
impl LexicalTask {
    /// 从位置参数构造语句 | 对语句[`LexicalSentence`]部分进行展开
    pub fn new(
        budget: &str,
        term: LexicalTerm,
        punctuation: &str,
        stamp: &str,
        truth: &str,
    ) -> Self {
        Self {
            budget: budget.into(),
            sentence: LexicalSentence::new(term, punctuation, stamp, truth),
        }
    }

    // 获取内部语句
    pub fn get_sentence(&self) -> &LexicalSentence {
        &self.sentence
    }
}

/// 快捷构造宏
#[macro_export]
macro_rules! lexical_task {
    [$($arg:expr)*] => {
        LexicalTask::new($($arg),*)
    };
}

/// 快捷构造预算
/// * 🎯兼容「Narsese格式」
/// * ⚠️实际上还是字符串
#[macro_export]
macro_rules! lexical_budget {
    [
        $left:expr;
        $separator:expr;
        $($value:expr)+;
        $right:expr $(;)?
    ] => {
        $left.to_string() + &[$($value),+].join($separator) + $right
    };
}

// 实现
impl GetTerm<LexicalTerm> for LexicalTask {
    /// 获取内部词项
    fn get_term(&self) -> &LexicalTerm {
        self.sentence.get_term()
    }
}

impl GetBudget<String> for LexicalTask {
    /// 获取内部预算值
    fn get_budget(&self) -> &String {
        &self.budget
    }
}

impl GetPunctuation<String> for LexicalTask {
    /// 获取内部标点
    fn get_punctuation(&self) -> &String {
        self.sentence.get_punctuation()
    }
}

impl GetStamp<String> for LexicalTask {
    /// 获取内部时间戳
    fn get_stamp(&self) -> &String {
        self.sentence.get_stamp()
    }
}

impl GetTruth<String> for LexicalTask {
    /// 获取内部真值（不一定有）
    fn get_truth(&self) -> Option<&String> {
        self.sentence.get_truth()
    }
}

/// 单元测试
#[cfg(test)]
#[allow(unused)]
mod tests {
    use crate::{lexical_atom, show};

    use super::*;

    #[test]
    fn main() {
        let term = lexical_atom!("word in sentence");
        let task = lexical_task![
            "$0.5; 0.5; 0.5$" term "." ":|:" "%1.0; 0.9%"
        ];
        show!(task);
    }
}
