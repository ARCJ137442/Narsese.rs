//! 词法上的Narsese
//! * 🎯不考虑内容的语义（只在字段上存储纯字符串，不检查语义）
//! * 🎯不考虑内容的组织方式（有序性/可重性）
//! * 🎯不考虑内容的存储方式（数据类型统一为字符串）
//!
//! 想法示例：
//! ```plaintext
//! Atom("^", "op")
//! Compound("&&", Atom("", "word"))
//! Statement("-->", Atom("+", "123"), Compound("\", Atom("_", ""), Atom("$", "1"), Set("{}", Atom("", "SELF"))))
//! ```

// 词项部分
mod term;
pub use term::*;

// 语句部分
mod sentence;
pub use sentence::*;

// 任务部分
mod task;
pub use task::*;

// 统合部分

/// 用于归并表示「词法上的Narsese」
pub enum LexicalNarsese {
    /// 词法性词项
    Term(LexicalTerm),
    /// 词法性语句
    Sentence(LexicalSentence),
    /// 词法性任务
    Task(LexicalTask),
}

/// 单元测试：词项+语句+任务
#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::{
        lexical_atom, lexical_compound, lexical_set, lexical_statement, lexical_task, util::*,
    };

    #[test]
    fn main() {
        let task = lexical_task![
            "$0.5; 0.5; 0.5$" lexical_compound![
                "复合词项连接词";
                lexical_atom!("word term")
                lexical_atom!("^", "操作")
                lexical_set![
                    "{"; lexical_atom!("SELF"); "}"
                ]
                lexical_statement![
                    lexical_set![
                        "{"; lexical_atom!("word1"), lexical_atom!("word2"); "}"
                    ]
                    "-->"
                    lexical_set![
                        "["; lexical_atom!("word1"), lexical_atom!("word2"); "]"
                    ]
                ]
            ] "." ":|:" "%1.0; 0.9%"
        ];
        show!(task);
    }
}
