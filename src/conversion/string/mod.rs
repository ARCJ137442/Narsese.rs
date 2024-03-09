//! 统一定义「字符串格式」「字符串解析器/格式化器」
//!
//! 📝【2024-02-20 16:30:57】模块符号组织策略：「命名空间+局部名称」🆚「唯一名称」
//! * 第一种如`StringParser`，第二种如`impl_parser::Parser`
//! * 📄标准库大量采用名如`Iter`的结构名称
//!   * 💭而并不担心「重名冲突」
//! * 📄[tomllib/parser.rs](https://github.com/joelself/tomllib/blob/master/src/internals/parser.rs)同样采用了第二种方法
//! * 第二种设计的弊端：无法简单使用`use impl_parser::*`导入模块内容
//! * 🚩目前采用第二种组织方式
//!   * 📌一是为了**简化名称**
//!   * 📌二是第一种可以使用`use impl_parser::{Parser as StringParser}`模拟

// 格式（数据结构）
pub mod format;
pub use format::*;

// 实现/格式化
#[cfg(feature = "enum_narsese")]
pub mod impl_formatter;
// #[cfg(feature = "enum_narsese")]
// pub use impl_formatter::*; // !【2024-03-09 17:54:14】实际上没有新导出任何东西
#[cfg(feature = "lexical_narsese")]
pub mod impl_formatter_lexical;
// #[cfg(feature = "lexical_narsese")]
// pub use impl_formatter_lexical::*; // !【2024-03-09 17:54:14】实际上没有新导出任何东西

// 实现/解析器
#[cfg(feature = "enum_narsese")]
pub mod impl_parser;
// #[cfg(feature = "enum_narsese")]
// pub use impl_parser::*; // !🚩【2024-03-09 18:01:35】暂且禁用：有歧义的导出
#[cfg(feature = "lexical_narsese")]
pub mod impl_parser_lexical;
// #[cfg(feature = "lexical_narsese")]
// pub use impl_parser_lexical::*; // !🚩【2024-03-09 18:01:35】暂且禁用：有歧义的导出

// 具体的格式 //
pub mod instances;
pub use instances::*;

/// 集成测试@枚举Narsese/字符串解析&格式化
#[cfg(test)]
#[cfg(feature = "enum_narsese")]
mod tests_enum {

    use self::impl_parser::NarseseResult;

    use super::*;
    use format::NarseseFormat;

    /// 用于给格式加上「自动解包并格式化内容」功能
    trait FormatResult {
        fn format_result(&self, result: &NarseseResult) -> String;
    }

    impl FormatResult for NarseseFormat<&str> {
        fn format_result(&self, result: &NarseseResult) -> String {
            match result {
                NarseseResult::Term(term) => self.format_term(term),
                NarseseResult::Sentence(sentence) => self.format_sentence(sentence),
                NarseseResult::Task(task) => self.format_task(task),
            }
        }
    }

    use crate::{
        enum_narsese::{Budget, Sentence, Stamp, Task, Term, Truth},
        f_tensor, show,
    };

    /// 先解析然后格式化
    fn _test_parse_and_format(format: &NarseseFormat<&str>, input: &str) -> String {
        // 解析
        let narsese = format.parse(input).unwrap();
        // 格式化
        let formatted = format.format_result(&narsese);
        // 展示
        show!(narsese);
        show!(formatted)
    }

    /// 先格式化然后解析
    /// * 直接从任务开始
    fn _test_format_and_parse(format: &NarseseFormat<&str>, input: Task) -> NarseseResult {
        // 格式化
        let formatted = format.format_task(&input);
        // 解析
        let narsese = format.parse(&formatted).unwrap();
        // 展示
        show!(formatted);
        show!(narsese)
    }

    /// 生成「矩阵」
    /// 🎯一个格式，多个函数，多个参数
    /// * 无需返回值
    macro_rules! test_matrix {
        [
            $format:expr;
            $(
                $f:ident => [$( $input:expr $(,)? )+]
            )+
            // *【2024-02-22 15:32:02】↑现在所有逗号都可选了
        ] => {
            {
                $({
                    // 告知测试
                    println!("Test in `{}`", stringify!($f));

                    // 生成矩阵 | 执行测试
                    let matrix = f_tensor![
                        $f [&$format] [ $($input)+ ]
                    ];

                    // 展示矩阵
                    show!(&matrix);
                })+;
            }
        };
    }

    /// （通用）构造一个格式化样本
    /// * 基本涵盖其所属模块的全部内容
    pub fn _sample_task() -> Task {
        // 构造词项
        let ball_left = Term::new_instance_property(Term::new_word("ball"), Term::new_word("left"));
        let conditional_operation = Term::new_conjunction_sequential(vec![
            ball_left.clone(),
            Term::new_inheritance(
                Term::new_product(vec![
                    Term::new_set_extension(vec![Term::new_word("SELF")]),
                    Term::new_variable_independent("any"),
                    Term::new_variable_dependent("some"),
                ]),
                Term::new_operator("do"),
            ),
        ]);
        let self_good = Term::new_instance_property(Term::new_word("SELF"), Term::new_word("good"));
        let term = Term::new_implication(conditional_operation.clone(), self_good.clone());

        // 构造语句
        let truth = Truth::Double(1.0, 0.9);
        let stamp = Stamp::Fixed(-1);
        let sentence = Sentence::new_judgement(term.clone(), truth, stamp);

        // 构造任务并返回
        let budget = Budget::Triple(0.5, 0.75, 0.4);
        Task::new(sentence.clone(), budget) // * 📝【2024-03-09 10:48:31】Clippy推荐直接返回构造之后的值
    }

    #[test]
    fn tests_ascii() {
        test_matrix! {
            FORMAT_ASCII;
            _test_parse_and_format => [
                "<(&/, <{powerup_good_front} --> [seen]>, +30000, <(*, {SELF}) --> ^right>, +30000) =/> <{SELF} --> [powered]>>. :|: %1.0;0.99%"
                "$$ 空预算要表示出来_空真值因为标点而无需必要. :|:"
            ]
            _test_format_and_parse => [
                _sample_task()
            ]
        }
    }

    #[test]
    fn tests_latex() {
        test_matrix! {
            FORMAT_LATEX;
            _test_parse_and_format => [
                r"\$0.5;0.75;0.4\$ \left<\left(,  \left<\left\{ball\right\} \rightarrow  \left[left\right]\right>  \left<\left(\times   \left\{SELF\right\}  \$any  \#some\right) \rightarrow  \Uparrow do\right>\right) \Rightarrow  \left<\left\{SELF\right\} \rightarrow  \left[good\right]\right>\right>. t=-1 \langle1,0.9\rangle"
            ]
            _test_format_and_parse => [
                _sample_task()
            ]
        }
    }

    #[test]
    fn tests_han() {
        test_matrix! {
            FORMAT_HAN;
            _test_parse_and_format => [
                "预0.5、0.75、0.4算「（接连，「『ball』是【left】」，「（积，『SELF』，任一any，其一some ）是操作do」）得「『SELF』是【good】」」。 发生在-1 真1、0.9值"
                "「我是谁」" // ! 先前的failed case
            ]
            _test_format_and_parse => [
                _sample_task()
            ]
        }
    }
}

/// 集成测试@枚举Narsese/字符串解析&格式化
#[cfg(test)]
#[cfg(feature = "enum_narsese")]
mod tests_lexical {
    use crate::{
        lexical::{LexicalTask, LexicalTerm},
        lexical_atom, lexical_budget, lexical_compound, lexical_set, lexical_stamp,
        lexical_statement, lexical_task, lexical_truth,
    };

    use super::NarseseFormat;

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
