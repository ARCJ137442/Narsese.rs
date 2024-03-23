use super::{Punctuation, Sentence, Stamp, Term, Truth};
use crate::api::{GetBudget, GetPunctuation, GetStamp, GetTerm, GetTruth};

/// 独立出来的「预算值」类型
/// * 🚩实际上是「字符串数组」的别名
/// * ✅对「作为数据结构的真值」的最大适配
///   * 📄空预算、单预算、双预算、三预算…
pub type Budget = Vec<String>;

/// 词法上的「任务」：预算值+语句
/// * 🚩【2024-03-15 22:03:48】现在不再特别加上「Lexical」前缀，而是使用命名空间区分
///   * 实际上就是`lexical::Task`或`use crate::lexical::Task as LexicalTask`的事儿
/// * 🚩【2024-03-22 17:54:42】现在不再让「真值」「预算值」糊成一块（作为一个整体而不区分其内的部分）
///   * 改为使用「数值的字串形式」
///   * ✅对于「变成数值后还要决定浮点精度，但为通用性不应强制精度」的问题：使用字符串形式，交给「词法折叠」过程
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    /// 预算值（数值字串）
    pub budget: Vec<String>,
    /// 词法语句
    pub sentence: Sentence,
}

/// 自身方法
impl Task {
    /// 从位置参数构造语句 | 对语句[`LexicalSentence`]部分进行展开
    pub fn new(
        budget: impl Into<Budget>,
        term: Term,
        punctuation: &str,
        stamp: &str,
        truth: impl Into<Truth>,
    ) -> Self {
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
    // 预算; 其它
    ($budget:expr; $($other:expr $(;)?)+) => {
        $crate::lexical_task![
            @NEW
            $budget;
            // * 🚩全部转发给语句 | 使用逗号分隔，避免歧义
            $crate::lexical_sentence![$($other),+]
        ]
    };
    // 无预算版本
    ($(;)? $(other:expr $(,)?)+) => {
        $crate::lexical_task![lexical_budget![]; $($punctuation)+]
    };
    // 一般转发，允许不写逗号 | 表达式字面量
    [$($arg:expr $(,)?)*] => {
        $crate::lexical_task![$( $arg );*]
    };
    // ! 使用内部括号包裹，以防「函数调用」歧义
    // [@NEW $( ($arg:expr) )*] => {
    //     // * 📝引入`$crate::lexical`作为绝对路径
    //     $crate::lexical::Task::new($($arg),*)
    // };
    [@NEW $budget:expr; $sentence:expr] => {
        // * 📝引入`$crate::lexical`作为绝对路径
        $crate::lexical::Task{
            budget: $budget,
            sentence: $sentence,
        }
    };
}

/// 快捷构造预算
/// * 🎯兼容「Narsese格式」
/// * ⚠️实际上还是字符串
#[macro_export]
macro_rules! lexical_budget {
    // 统一形式 | 允许可选逗号分隔
    // * 🚩通过`into`自动处理`String`和`&str`
    [ $( $value:literal $(,)? )* ] => {
        vec![$($value.to_string()),*]
    };
    [ $( $value:expr $(,)? )* ] => {
        vec![$($value),*]
    };
}

// 实现
impl GetTerm<Term> for Task {
    /// 获取内部词项
    fn get_term(&self) -> &Term {
        self.sentence.get_term()
    }
}

impl GetBudget<Budget> for Task {
    /// 获取内部预算值
    fn get_budget(&self) -> &Budget {
        &self.budget
    }
}

impl GetPunctuation<Punctuation> for Task {
    /// 获取内部标点
    fn get_punctuation(&self) -> &Punctuation {
        self.sentence.get_punctuation()
    }
}

impl GetStamp<Stamp> for Task {
    /// 获取内部时间戳
    fn get_stamp(&self) -> &Stamp {
        self.sentence.get_stamp()
    }
}

impl GetTruth<Truth> for Task {
    /// 获取内部真值（不一定有）
    fn get_truth(&self) -> Option<&Truth> {
        self.sentence.get_truth()
    }
}

/// 单元测试
#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::{lexical_atom, lexical_truth, util::*};

    #[test]
    fn main() {
        // 词项
        let term = lexical_atom!("word in task");

        // 完整形式
        let task = lexical_task![
            lexical_budget!["0.5" "0.5" "0.5"]
            term.clone() "." ":|:" lexical_truth!["1.0" "0.9"]
        ];
        show!(&task);
        asserts! {
            task.get_term() => &term, // 词项
            task.get_punctuation() => ".", // 标点
            task.get_stamp() => ":|:", // 时间戳
            task.get_budget() => &["0.5", "0.5", "0.5"], // 预算值
            task.get_truth().unwrap() => &["1.0", "0.9"], // 真值
        }

        // 像语句那样缺省
        let task = lexical_task![
            lexical_budget!["0.5" "0.5" "0.5"]
            term.clone() "."
        ];
        show!(&task);
        asserts! {
            task.get_budget() => &["0.5", "0.5", "0.5"], // 预算值
            task.get_term() => &term, // 词项
            task.get_punctuation() => ".", // 标点
            task.get_stamp() => "", // 时间戳（空）
            task.get_truth().unwrap().is_empty(), // 真值（空）
        }
    }
}
