use super::{LexicalSentence, LexicalTerm};

/// 词法上的「任务」：预算值+语句
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexicalTask {
    budget: String,
    sentence: LexicalSentence,
}

/// 实现
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
}

/// 快捷构造宏
#[macro_export]
macro_rules! lexical_task {
    [$($arg:expr)*] => {
        LexicalTask::new($($arg),*)
    };
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
