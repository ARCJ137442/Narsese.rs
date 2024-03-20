use super::Term;
use crate::api::{GetPunctuation, GetStamp, GetTerm, GetTruth};

/// 词法上的「语句」：词项+标点+时间戳+真值
/// * 仅作为「最大并集」，不考虑「问题/请求 无真值」等情况
/// * 🚩【2024-03-15 22:03:48】现在不再特别加上「Lexical」前缀，而是使用命名空间区分
///   * 实际上就是`lexical::Sentence`或`use crate::lexical::Sentence as LexicalSentence`的事儿
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sentence {
    /// 词法词项
    pub term: Term,
    /// 标点（字符串）
    pub punctuation: String,
    /// 时间戳（字符串）
    pub stamp: String,
    /// 真值（字符串）
    pub truth: String,
}

/// 自身方法
impl Sentence {
    /// 从位置参数构造语句
    pub fn new(term: Term, punctuation: &str, stamp: &str, truth: &str) -> Self {
        Self {
            term,
            punctuation: punctuation.into(),
            stamp: stamp.into(),
            truth: truth.into(),
        }
    }
}

/// 快捷构造宏
#[macro_export]
macro_rules! lexical_sentence {
    [$($arg:expr)*] => {
        // * 📝引入`$crate::lexical`作为绝对路径
        $crate::lexical::Sentence::new($($arg),*)
    };
}

/// 快捷构造时间戳
/// * 🎯兼容「Narsese格式」
/// * ⚠️实际上还是字符串
#[macro_export]
macro_rules! lexical_stamp {
    // 有内部值的
    // * 🎯用于「固定」时间戳
    [
        $left:expr;
        $head:expr;
        $value:expr;
        $right:expr $(;)?
    ] => {
        $left.to_string() + $head + $value + $right
    };
    // 没内部值的
    [
        $left:expr;
        $head:expr;
        $right:expr $(;)?
    ] => {
        $left.to_string() + $head + $right
    };
}

/// 快捷构造真值
/// * 🎯兼容「Narsese格式」
/// * ⚠️实际上还是字符串
#[macro_export]
macro_rules! lexical_truth {
    // 内部有值的
    [
        $left:expr;
        $separator:expr;
        $($value:expr)*;
        $right:expr $(;)?
    ] => {
        $left.to_string() + &[$($value),*].join($separator) + $right
    };
    // 空真值
    [
        $left:expr;
        $separator:expr;
        $right:expr $(;)?
    ] => {
        $left.to_string() + $right
    };
}

// 实现
impl GetTerm<Term> for Sentence {
    fn get_term(&self) -> &Term {
        &self.term
    }
}

impl GetPunctuation<String> for Sentence {
    fn get_punctuation(&self) -> &String {
        &self.punctuation
    }
}

impl GetStamp<String> for Sentence {
    fn get_stamp(&self) -> &String {
        &self.stamp
    }
}

impl GetTruth<String> for Sentence {
    fn get_truth(&self) -> Option<&String> {
        Some(&self.truth)
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
        let sentence = lexical_sentence![
            term "." ":|:" "%1.0; 0.9%"
        ];
        show!(sentence);
    }
}
