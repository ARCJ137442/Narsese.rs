use crate::common_api::{GetPunctuation, GetStamp, GetTerm, GetTruth};

use super::LexicalTerm;

/// 词法上的「语句」：词项+标点+时间戳+真值
/// * 仅作为「最大并集」，不考虑「问题/请求 无真值」等情况
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexicalSentence {
    term: LexicalTerm,
    pub punctuation: String,
    pub stamp: String,
    pub truth: String,
}

/// 自身方法
impl LexicalSentence {
    /// 从位置参数构造语句
    pub fn new(term: LexicalTerm, punctuation: &str, stamp: &str, truth: &str) -> Self {
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
        LexicalSentence::new($($arg),*)
    };
}

// 实现
impl GetTerm<LexicalTerm> for LexicalSentence {
    fn get_term(&self) -> &LexicalTerm {
        &self.term
    }
}

impl GetPunctuation<String> for LexicalSentence {
    fn get_punctuation(&self) -> &String {
        &self.punctuation
    }
}

impl GetStamp<String> for LexicalSentence {
    fn get_stamp(&self) -> &String {
        &self.stamp
    }
}

impl GetTruth<String> for LexicalSentence {
    fn get_truth(&self) -> Option<&String> {
        Some(&self.truth)
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
        let sentence = lexical_sentence![
            term "." ":|:" "%1.0; 0.9%"
        ];
        show!(sentence);
    }
}
