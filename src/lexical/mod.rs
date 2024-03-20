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
//!
//! ! ⚠️【2024-03-20 02:13:50】注意：此模块导出了宏，故无法使用[`util::pub_mod_and_pub_use`]

use crate::api::NarseseValue;

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
/// * 🚩现在使用更抽象的「Narsese值」取代
pub type Narsese = NarseseValue<Term, Sentence, Task>;

/// 快捷方式：用于快速构建「词法Narsese」
/// * ⚠️不建议直接导出其中的符号，而是通过`lexical::shortcut`引入
pub mod shortcut {
    // 自动去掉其中的所有`lexical_`前缀
    pub use crate::{
        lexical_atom as atom, lexical_budget as budget, lexical_compound as compound,
        lexical_set as set, lexical_stamp as stamp, lexical_statement as statement,
        lexical_task as task, lexical_truth as truth,
    };
}

/// 单元测试：词项+语句+任务
/// * 🚩【2024-03-20 12:42:48】公开：共享测试集
#[cfg(test)]
#[allow(unused)]
pub(crate) mod tests {
    use super::*;
    use crate::lexical::shortcut::*;
    use util::*;

    /// （通用）构造一个格式化样本（ASCII字面量版本）
    /// * 基本涵盖其所属模块的全部内容
    /// * 📌格式稳定版：基本所有其它格式以此为参照
    ///   * 为何此处版本不如「枚举Narsese」那样通用？
    ///   * 词项前缀、复合词项连接词、陈述系词都是不同的系统（本身就没法相互解析）
    pub(crate) fn _sample_task_ascii() -> Task {
        // 构造词项
        let ball_left = statement!(atom!("ball") "{-]" atom!("left"));
        let conditional_operation = compound!(
            "&/",
            ball_left.clone(),
            statement!(
                compound!(
                    "*",
                    set!("{"; "SELF" ;"}"),
                    atom!("$" "any"),
                    atom!("#" "some"),
                )
                "-->"
                atom!("^" "do")
            ),
        );
        let self_good = statement!(atom!("SELF") "{-]" atom!("good"));
        let term = statement!(
            conditional_operation.clone()
            "==>"
            self_good.clone()
        );

        // 构造语句
        let truth = "%1.0; 0.9%";
        let stamp = ":!-1:";
        let punctuation = ".";
        // let sentence = sentence!(
        //     term.clone() "." stamp truth
        // ); // ! 此处无需构建；直接构建任务

        // 构造任务并返回
        let budget = "$0.5; 0.75; 0.4$";
        task!(budget term.clone() punctuation stamp truth) // * 📝【2024-03-09 10:48:31】Clippy推荐直接返回构造之后的值
    }

    /// 使用ASCII格式构造「样本任务」的最初版本
    pub(crate) fn _sample_task_ascii_0() -> Task {
        task![
            "$0.5; 0.5; 0.5$" compound![
                "复合词项连接词";
                atom!("word term")
                atom!("^", "操作")
                set![
                    "{"; atom!("SELF"); "}"
                ]
                statement![
                    set![
                        "{"; atom!("word1"), atom!("word2"); "}"
                    ]
                    "-->"
                    set![
                        "["; atom!("word1"), atom!("word2"); "]"
                    ]
                ]
            ] "." ":|:" "%1.0; 0.9%"
        ]
    }

    #[test]
    fn main() {
        let task = _sample_task_ascii_0();
        show!(task);
    }
}
