use super::Term;
use crate::api::{GetPunctuation, GetStamp, GetTerm, GetTruth};

/// 独立出来的「真值」类型
/// * 🚩实际上是「字符串数组」的别名
/// * ✅对「作为数据结构的真值」的最大适配
///   * 📄空真值、单真值、双真值…
pub type Truth = Vec<String>;

/// 独立出来的「时间戳」类型
/// * 🚩实际上是「字符串」的别名
pub type Stamp = String;

/// 独立出来的「标点」类型
/// * 🚩实际上是「字符串」的别名
pub type Punctuation = String;

/// 词法上的「语句」：词项+标点+时间戳+真值
/// * 仅作为「最大并集」，不考虑「问题/请求 无真值」等情况
/// * 🚩【2024-03-15 22:03:48】现在不再特别加上「Lexical」前缀，而是使用命名空间区分
///   * 实际上就是`lexical::Sentence`或`use crate::lexical::Sentence as LexicalSentence`的事儿
/// * 🚩【2024-03-22 17:54:42】现在不再让「真值」「预算值」糊成一块（作为一个整体而不区分其内的部分）
///   * 改为使用「数值的字串形式」
///   * ✅对于「变成数值后还要决定浮点精度，但为通用性不应强制精度」的问题：使用字符串形式，交给「词法折叠」过程
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sentence {
    /// 词法词项
    pub term: Term,
    /// 标点（字符串）
    pub punctuation: Punctuation,
    /// 时间戳（字符串）
    pub stamp: Stamp,
    /// 真值（字符串）
    pub truth: Truth,
}

/// 自身方法
impl Sentence {
    /// 从位置参数构造语句
    pub fn new(term: Term, punctuation: &str, stamp: &str, truth: impl Into<Truth>) -> Self {
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
    // 统一形式 | 允许可选逗号分隔
    // * 🚩通过`into`自动处理`String`和`&str`
    [ $( $value:expr $(,)? )* ] => {
        vec![$($value.into()),*]
    };
}

// 实现
impl GetTerm<Term> for Sentence {
    fn get_term(&self) -> &Term {
        &self.term
    }
}

impl GetPunctuation<Punctuation> for Sentence {
    fn get_punctuation(&self) -> &Punctuation {
        &self.punctuation
    }
}

impl GetStamp<Stamp> for Sentence {
    fn get_stamp(&self) -> &Stamp {
        &self.stamp
    }
}

impl GetTruth<Truth> for Sentence {
    /// ! 缩减[`Option`]失败：参见[`GetTruth`]的描述
    fn get_truth(&self) -> Option<&Truth> {
        // 此处一定返回有值（数组）
        // * 🚩没真值的当空真值对待
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
            term "." ":|:" lexical_truth!["1.0", "0.9%"]
        ];
        show!(sentence);
    }
}
