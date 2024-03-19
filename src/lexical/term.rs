/// 词法上的「词项」
/// * 📌只在词法（字符串语法）上表征词项
/// * 📌所有最终字段都是字符串
/// * 📌所有组分容器都是有序向量[`Vec`]
/// * ⚠️不同于[`crate::Term`]，不在语义上区分「像」与「复合词项」
///   * 在**词法**上将「像」视作一个【内含占位符】的复合词项
///   * 如 `(\, _, R)` => `Compound { connecter: "/", terms: [Atom { prefix: "_", name: "" }, Atom { prefix: "", name: "R" }]}`
/// * 🚩【2024-03-15 22:03:48】现在不再特别加上「Lexical」前缀，而是使用命名空间区分
///   * 实际上就是`lexical::Term`或`use crate::lexical::Term as LexicalTerm`的事儿
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    /// 原子词项：前缀+名称
    Atom { prefix: String, name: String },
    /// 复合词项：连接符+组分
    Compound { connecter: String, terms: Vec<Term> },
    /// 集合：左右括号+组分
    /// * 应对词法上特殊的「外延集/内涵集」
    Set {
        left_bracket: String,
        terms: Vec<Term>,
        right_bracket: String,
    },
    /// 陈述：系词+主词+谓词
    Statement {
        copula: String,
        subject: Box<Term>,
        predicate: Box<Term>,
    },
}

/// 实现
impl Term {
    /// 位置参数新建原子词项
    pub fn new_atom(prefix: &str, name: &str) -> Term {
        Term::Atom {
            prefix: prefix.into(),
            name: name.into(),
        }
    }
    /// 位置参数新建复合词项
    pub fn new_compound(connecter: &str, terms: Vec<Term>) -> Term {
        Term::Compound {
            connecter: connecter.into(),
            terms,
        }
    }
    /// 位置参数新建集合
    pub fn new_set(left_bracket: &str, terms: Vec<Term>, right_bracket: &str) -> Term {
        Term::Set {
            left_bracket: left_bracket.into(),
            terms,
            right_bracket: right_bracket.into(),
        }
    }
    /// 位置参数新建陈述
    pub fn new_statement(copula: &str, subject: Term, predicate: Term) -> Term {
        Term::Statement {
            copula: copula.into(),
            subject: Box::new(subject),
            predicate: Box::new(predicate),
        }
    }
    /// 位置参数新建陈述（中缀）
    pub fn new_statement_infix(subject: Term, copula: &str, predicate: Term) -> Term {
        Term::new_statement(copula, subject, predicate)
    }
}

// * 📝快速构建约定：原子词项使用圆括号`()`，词项容器（陈述、复合词项、集合）使用方括号`[]`

/// 快速构建原子词项
/// * 📝若需限定此中使用的类型的路径，建议加上`$crate`约束
///   * ✅这样可以避免与其它库的同名类型产生冲突
///   * ✅同时可以避免「导入这个宏之后，还要连带导入『其所定义的类型』」的繁杂问题
///     * 这个「连带导入」目前IDE还难以自动补全
///   * 📝本质原理：从「定义该宏的模块」而非「使用该宏的模块」引入符号`Term`
#[macro_export]
macro_rules! lexical_atom {
    // 无间隔形式
    ( $prefix:tt $name:expr ) => {
        $crate::lexical::Term::new_atom($prefix, $name)
    };
    // 有逗号形式
    ( $prefix:expr, $name:expr ) => {
        $crate::lexical::Term::new_atom($prefix, $name)
    };
    // 空前缀形式
    ( $name:expr ) => {
        $crate::lexical::Term::new_atom("", $name)
    };
}

/// 快速构建复合词项
#[macro_export]
macro_rules! lexical_compound {
    // (连接符, 内容...) | 模拟不定长参数
    [ $connecter:expr, $($term:expr),* $(,)? ] => {
        $crate::lexical::Term::new_compound($connecter, vec![$($term),*])
    };
    // [连接符; 内容1 内容2]
    [$connecter:expr; $($term:expr)*] => {
        $crate::lexical::Term::new_compound($connecter, vec![$($term),*])
    };
}

/// 快速构建集合
#[macro_export]
macro_rules! lexical_set {
    // 左括号；字符串自面量（直接作为「无参原子」加入）；右括号
    [ $left:expr ; $name:literal ; $right:expr ] => {
        $crate::lexical::Term::new_set($left, vec![$crate::lexical_atom!($name)], $right)
    };
    // 左括号；中间内容（可选逗号）；右括号
    [ $left:expr ; $($term:expr $(,)?)* ; $right:expr ] => {
        $crate::lexical::Term::new_set($left, vec![$($term),*], $right)
    };
}

/// 快速构建陈述
#[macro_export]
macro_rules! lexical_statement {
    // 主词 系词 谓词
    [$($ex:expr $(,)?)*] => {
        $crate::lexical::Term::new_statement_infix($($ex),*)
    };
    // 系词; 主词 谓词
    [$copula:expr ; $($ex:expr $(,)?)*] => {
        $crate::lexical::Term::new_statement($copula, $($ex),*)
    };
}

/// 单元测试@词项
#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use util::show;

    #[test]
    fn main() {
        lexical_atom!("^" "op");
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
