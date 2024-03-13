//! 枚举Narsese与「字符串转换」有关的模块

// 格式
// * 【2024-03-13 14:42:13】最初源自enum_narsese
mod format;
pub use format::*;

// 格式化
// * 🚩直接对「枚举Narsese格式」实现「格式化」方法
//   * 所以没导出模块内容
mod formatter;

// 解析器
mod parser;
pub use parser::*;

// 解析格式的实例
// * 🚩目前仍作为单独的子模块导出，而**不导出其内元素**
//  * 其内元素可能会造成名称混淆
// * 📝导入并【以别名重新导出】模块，实际上不太实用
//  * 🚩此处弃用
pub mod format_instances;
// pub use format_instances as instances;

/// 集成测试@枚举Narsese/字符串解析&格式化
#[cfg(test)]
mod tests {

    use self::parser::NarseseResult;
    use super::format_instances::*;
    use super::*;
    use crate::enum_narsese::{Budget, Sentence, Stamp, Task, Term, Truth};
    use util::{f_tensor, show};

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
