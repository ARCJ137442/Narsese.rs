//! 词法Narsese

// 格式化
// * 🚩直接对「Narsese格式」实现「格式化」方法
//   * 所以没导出模块内容
mod formatter;

// 解析器
mod parser;
pub use parser::*;

/// 集成测试@词法Narsese/字符串解析&格式化
#[cfg(test)]
#[cfg(feature = "lexical_narsese")]
mod tests_lexical {
    use super::super::common::NarseseFormat;
    use crate::{
        lexical::{LexicalTask, LexicalTerm},
        lexical_atom, lexical_budget, lexical_compound, lexical_set, lexical_stamp,
        lexical_statement, lexical_task, lexical_truth,
    };

    /// （通用）构造一个格式化样本（ASCII自面量版本）
    /// * 基本涵盖其所属模块的全部内容
    fn _sample_task_ascii() -> LexicalTask {
        // 构造词项
        let ball_left = lexical_statement!(lexical_atom!("ball") "{-]" lexical_atom!("left"));
        let conditional_operation = lexical_compound!(
            "&/",
            ball_left.clone(),
            lexical_statement!(
                lexical_compound!(
                    "*",
                    lexical_set!("{"; "SELF" ;"}"),
                    lexical_atom!("$" "any"),
                    lexical_atom!("#" "some"),
                )
                "-->"
                lexical_atom!("^" "do")
            ),
        );
        let self_good = lexical_statement!(lexical_atom!("SELF") "{-]" lexical_atom!("good"));
        let term = lexical_statement!(
            conditional_operation.clone()
            "==>"
            self_good.clone()
        );

        // 构造语句
        let truth = "%1.0; 0.9%";
        let stamp = ":!-1:";
        let punctuation = ".";
        // let sentence = lexical_sentence!(
        //     term.clone() "." stamp truth
        // ); // ! 此处无需构建；直接构建任务

        // 构造任务并返回
        let budget = "$0.5; 0.75; 0.4$";
        lexical_task!(budget term.clone() punctuation stamp truth) // * 📝【2024-03-09 10:48:31】Clippy推荐直接返回构造之后的值
    }

    /// （通用）构造一个格式化样本
    /// * 基本涵盖其所属模块的全部内容
    /// * 📌其中还有一些「格式特有」的东西
    pub fn _sample_task(format: &NarseseFormat<&str>) -> LexicalTask {
        // 构造词项
        let ball_left = lexical_statement!(
            lexical_atom!(format.atom.prefix_word, "ball")
            format.statement.copula_instance_property
            lexical_atom!(format.atom.prefix_word, "left")
        );
        let conditional_operation = lexical_compound!(
            format.compound.connecter_conjunction_sequential,
            ball_left.clone(),
            lexical_statement!(
                lexical_compound!(
                    format.compound.connecter_product,
                    lexical_set!(
                        format.compound.brackets_set_extension.0;
                        // ! ↓此处不一定是「空字串前缀」了
                        lexical_atom!(format.atom.prefix_word, "SELF");
                        format.compound.brackets_set_extension.1
                    ),
                    lexical_atom!(format.atom.prefix_variable_independent, "any"),
                    lexical_atom!(format.atom.prefix_variable_dependent, "some"),
                )
                format.statement.copula_inheritance
                lexical_atom!(format.atom.prefix_operator, "do")
            ),
        );
        let self_good = lexical_statement!(
            lexical_atom!(format.atom.prefix_word, "SELF")
            format.statement.copula_instance_property
            lexical_atom!(format.atom.prefix_word, "good")
        );
        let term = lexical_statement!(
            conditional_operation.clone()
            format.statement.copula_implication
            self_good.clone()
        );

        // 构造语句
        let truth = &lexical_truth!(
            format.sentence.truth_brackets.0;
            format.sentence.truth_separator; // * 没有装饰性空格
            "1.0" "0.9";
            format.sentence.truth_brackets.1;
        );
        let stamp = &lexical_stamp!(
            format.sentence.stamp_brackets.0;
            format.sentence.stamp_fixed;
            "-1";
            format.sentence.stamp_brackets.1
        );
        let punctuation = ".";
        // let sentence = lexical_sentence!(
        //     term.clone() "." stamp truth
        // ); // ! 此处无需构建；直接构建任务

        // 构造任务并返回
        let budget = &lexical_budget!(
            format.task.budget_brackets.0;
            format.task.budget_separator; // * 没有装饰性空格
            "0.5" "0.75" "0.4";
            format.task.budget_brackets.1
        );
        lexical_task!(budget term.clone() punctuation stamp truth) // * 📝【2024-03-09 10:48:31】Clippy推荐直接返回构造之后的值
    }
}
