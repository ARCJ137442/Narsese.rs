use crate::api::{GetBudget, GetPunctuation, GetStamp, GetTerm, GetTruth};

use super::{Sentence, Term};

/// 词法上的「任务」：预算值+语句
/// * 🚩【2024-03-15 22:03:48】现在不再特别加上「Lexical」前缀，而是使用命名空间区分
///   * 实际上就是`lexical::Task`或`use crate::lexical::Task as LexicalTask`的事儿
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    /// 预算值（字符串）
    pub budget: String,
    /// 词法语句
    pub sentence: Sentence,
}

/// 自身方法
impl Task {
    /// 从位置参数构造语句 | 对语句[`LexicalSentence`]部分进行展开
    pub fn new(budget: &str, term: Term, punctuation: &str, stamp: &str, truth: &str) -> Self {
        Self {
            budget: budget.into(),
            sentence: Sentence::new(term, punctuation, stamp, truth),
        }
    }

    // 获取内部语句
    pub fn get_sentence(&self) -> &Sentence {
        &self.sentence
    }
}

/// 快捷构造宏
#[macro_export]
macro_rules! lexical_task {
    [$($arg:expr)*] => {
        // * 📝引入`$crate::lexical`作为绝对路径
        $crate::lexical::Task::new($($arg),*)
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
impl GetTerm<Term> for Task {
    /// 获取内部词项
    fn get_term(&self) -> &Term {
        self.sentence.get_term()
    }
}

impl GetBudget<String> for Task {
    /// 获取内部预算值
    fn get_budget(&self) -> &String {
        &self.budget
    }
}

impl GetPunctuation<String> for Task {
    /// 获取内部标点
    fn get_punctuation(&self) -> &String {
        self.sentence.get_punctuation()
    }
}

impl GetStamp<String> for Task {
    /// 获取内部时间戳
    fn get_stamp(&self) -> &String {
        self.sentence.get_stamp()
    }
}

impl GetTruth<String> for Task {
    /// 获取内部真值（不一定有）
    fn get_truth(&self) -> Option<&String> {
        self.sentence.get_truth()
    }
}

/// 单元测试
#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::{lexical_atom, util::*};

    #[test]
    fn main() {
        let term = lexical_atom!("word in sentence");
        let task = lexical_task![
            "$0.5; 0.5; 0.5$" term "." ":|:" "%1.0; 0.9%"
        ];
        show!(task);
    }
}
