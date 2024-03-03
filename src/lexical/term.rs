/// 词法上的「词项」
/// * 📌只在词法（字符串语法）上表征词项
/// * 📌所有最终字段都是字符串
/// * 📌所有组分容器都是有序向量[`Vec`]
/// * ⚠️不同于[`crate::Term`]，不在语义上区分「像」与「复合词项」
///   * 在**词法**上将「像」视作一个【内含占位符】的复合词项
///   * 如 `(\, _, R)` => `Compound { connecter: "/", terms: [Atom { prefix: "_", name: "" }, Atom { prefix: "", name: "R" }]}`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexicalTerm {
    /// 原子词项：前缀+名称
    Atom { prefix: String, name: String },
    /// 复合词项：连接符+组分
    Compound {
        connecter: String,
        terms: Vec<LexicalTerm>,
    },
    /// 集合：左右括号+组分
    /// * 应对词法上特殊的「外延集/内涵集」
    Set {
        left_bracket: String,
        terms: Vec<LexicalTerm>,
        right_bracket: String,
    },
    /// 陈述：系词+主词+谓词
    Statement {
        copula: String,
        subject: Box<LexicalTerm>,
        predicate: Box<LexicalTerm>,
    },
}

/// 实现
impl LexicalTerm {
    /// 位置参数新建原子词项
    pub fn new_atom(prefix: &str, name: &str) -> LexicalTerm {
        LexicalTerm::Atom {
            prefix: prefix.into(),
            name: name.into(),
        }
    }
    /// 位置参数新建复合词项
    pub fn new_compound(connecter: &str, terms: Vec<LexicalTerm>) -> LexicalTerm {
        LexicalTerm::Compound {
            connecter: connecter.into(),
            terms,
        }
    }
    /// 位置参数新建集合
    pub fn new_set(
        left_bracket: &str,
        terms: Vec<LexicalTerm>,
        right_bracket: &str,
    ) -> LexicalTerm {
        LexicalTerm::Set {
            left_bracket: left_bracket.into(),
            terms,
            right_bracket: right_bracket.into(),
        }
    }
    /// 位置参数新建陈述
    pub fn new_statement(
        copula: &str,
        subject: LexicalTerm,
        predicate: LexicalTerm,
    ) -> LexicalTerm {
        LexicalTerm::Statement {
            copula: copula.into(),
            subject: Box::new(subject),
            predicate: Box::new(predicate),
        }
    }
    /// 位置参数新建陈述（中缀）
    pub fn new_statement_infix(
        subject: LexicalTerm,
        copula: &str,
        predicate: LexicalTerm,
    ) -> LexicalTerm {
        LexicalTerm::new_statement(copula, subject, predicate)
    }
}

// * 📝快速构建约定：原子词项使用圆括号`()`，词项容器（陈述、复合词项、集合）使用方括号`[]`

/// 快速构建原子词项
#[macro_export]
macro_rules! lexical_atom {
    // 无间隔形式
    ( $prefix:tt $name:expr ) => {
        LexicalTerm::new_atom($prefix, $name)
    };
    // 有逗号形式
    ( $prefix:expr, $name:expr ) => {
        LexicalTerm::new_atom($prefix, $name)
    };
    // 空前缀形式
    ( $name:expr ) => {
        LexicalTerm::new_atom("", $name)
    };
}

/// 快速构建复合词项
#[macro_export]
macro_rules! lexical_compound {
    // (连接符, 内容...) | 模拟不定长参数
    [ $connecter:expr, $($term:expr),* $(,)? ] => {
        LexicalTerm::new_compound($connecter, vec![$($term),*])
    };
    // [连接符; 内容1 内容2]
    [$connecter:expr; $($term:expr)*] => {
        LexicalTerm::new_compound($connecter, vec![$($term),*])
    };
}

/// 快速构建集合
#[macro_export]
macro_rules! lexical_set {
    // 左括号；中间内容（可选逗号）；右括号
    [ $left:expr ; $($term:expr $(,)?)* ; $right:expr ] => {
        LexicalTerm::new_set($left, vec![$($term),*], $right)
    };
}

/// 快速构建陈述
#[macro_export]
macro_rules! lexical_statement {
    // 主词 系词 谓词
    [$($ex:expr $(,)?)*] => {
        LexicalTerm::new_statement_infix($($ex),*)
    };
    // 系词; 主词 谓词
    [$copula:expr ; $($ex:expr $(,)?)*] => {
        LexicalTerm::new_statement($copula, $($ex),*)
    };
}

/// 单元测试@词项
#[cfg(test)]
#[allow(unused)]
mod tests {
    use crate::show;

    use super::*;

    #[test]
    fn main() {
        let lex_c = lexical_compound![
            "&&";
            lexical_atom!("^" "op")
            lexical_set![
                "{"; lexical_atom!("word1") lexical_atom!("word2"); "}"
            ]
            lexical_set![
                "{"; lexical_atom!("SELF"); "}"
            ]
            lexical_statement![lexical_atom!("+" "123") "-->" lexical_atom!("-" "1")]
            lexical_statement![lexical_atom!("$" "A") "=/>" lexical_atom!("#" "B")]
        ];
        show!(lex_c);
    }
}
