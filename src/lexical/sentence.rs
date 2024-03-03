use super::LexicalTerm;

/// 词法上的「语句」：词项+标点+时间戳+真值
/// * 仅作为「最大并集」，不考虑「问题/请求 无真值」等情况
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexicalSentence {
    term: LexicalTerm,
    punctuation: String,
    stamp: String,
    truth: String,
}

/// 实现
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
