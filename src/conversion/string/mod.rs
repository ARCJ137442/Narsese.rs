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
pub mod impl_formatter;

// 实现/解析器
pub mod impl_parser;

// 具体的格式 //
pub mod instances;
pub use instances::*;

/// 集成测试@字符串解析&格式化
#[cfg(test)]
mod tests {

    use self::impl_parser::NarseseResult;

    use super::*;
    use format::NarseseFormat;

    trait FormatResult {
        fn format_result(&self, result: &NarseseResult) -> String;
    }

    impl FormatResult for NarseseFormat<&str> {
        fn format_result(&self, result: &NarseseResult) -> String {
            match result {
                NarseseResult::Term(term) => self.format_term(&term),
                NarseseResult::Sentence(sentence) => self.format_sentence(&sentence),
                NarseseResult::Task(task) => self.format_task(&task),
            }
        }
    }

    use crate::{show, Budget, Sentence, Stamp, Task, Term, Truth};

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
                    // 生成行列
                    let inputs = [$($input),+];
                    // 新建一个列
                    let mut col = vec![];
                    // 生成列元素
                    for input in inputs {
                        col.push($f(&$format, input))
                    }
                })+;
            }
        };
    }

    /// 构造一个格式化样本
    fn _sample_task() -> Task {
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
        // 构造任务
        let budget = Budget::Triple(0.5, 0.75, 0.4);
        let task = Task::new(sentence.clone(), budget);
        // 返回
        task
    }

    #[test]
    fn tests() {
        test_matrix! {
            FORMAT_ASCII;
            _test_parse_and_format => [
                "<(&/, <{powerup_good_front} --> [seen]>, +30000, <(*, {SELF}) --> ^right>, +30000) =/> <{SELF} --> [powered]>>. :|: %1.0;0.99%"
            ]
            _test_format_and_parse => [
                _sample_task()
            ]
        }
        test_matrix! {
            FORMAT_LATEX;
            _test_parse_and_format => [
                r"\$0.5;0.75;0.4\$ \left<\left(;  \left<\left\{ball\right\} \rightarrow  \left[left\right]\right>  \left<\left(\times   \left\{SELF\right\}  \$any  \#some\right) \rightarrow  \Uparrow do\right>\right) \Rightarrow  \left<\left\{SELF\right\} \rightarrow  \left[good\right]\right>\right>. t=-1 \langle1,0.9\rangle"
            ]
            _test_format_and_parse => [
                _sample_task()
            ]
        }
        test_matrix! {
            FORMAT_HAN;
            _test_parse_and_format => [
                "预0.5、0.75、0.4算「（同时，「『ball』是【left】」，「（积，『SELF』，任一any，其一some ）是操作do」）得「『SELF』是【good】」」。时刻=-1真值=1真0.9信"
                "「我是谁」" // ! 先前的failed case
            ]
            _test_format_and_parse => [
                _sample_task()
            ]
        }
    }
}
