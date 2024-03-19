//! 词法Narsese

util::mod_and_pub_use! {
    // 格式
    format
    // 解析器
    parser
}

// 格式化
// * 🚩直接对「词法Narsese格式」实现「格式化」方法
//   * 所以没导出模块内容
mod formatter;

// 解析格式的实例
// * 🚩目前仍作为单独的子模块导出，而**不导出其内元素**
//  * 其内元素可能会造成名称混淆
// * 📝导入并【以别名重新导出】模块，实际上不太实用
//  * 🚩此处弃用
pub mod format_instances;
// pub use format_instances as instances;

/// 集成测试@词法Narsese/字符串解析&格式化
#[cfg(test)]
mod tests {
    use super::NarseseFormat;
    use crate::{
        conversion::string::impl_lexical::format_instances::*,
        lexical::{Narsese, Task},
        lexical_atom, lexical_compound, lexical_set, lexical_statement, lexical_task,
    };
    use util::*;

    /// （通用）构造一个格式化样本（ASCII字面量版本）
    /// * 基本涵盖其所属模块的全部内容
    /// * 📌格式稳定版：基本所有其它格式以此为参照
    ///   * 为何此处版本不如「枚举Narsese」那样通用？
    ///   * 词项前缀、复合词项连接词、陈述系词都是不同的系统（本身就没法相互解析）
    pub(crate) fn _sample_task_ascii() -> Task {
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

    /// 构造一个格式化样本（LaTeX版本）
    /// * ⚠️其中有些部分可能会过时
    /// * 🚩【2024-03-20 01:22:26】目前就从相应字符串中解析得来
    pub(crate) fn _sample_task_latex() -> Task {
        // 直接从文本构造词项
        let input = r"\$0.5;0.75;0.4\$ \left<\left(,\; \left<\left\{ball\right\} \rightarrow{} \left[left\right]\right>\; \left<\left(\times{}\; \left\{SELF\right\}\; \$any\; \#some\right) \rightarrow{} \Uparrow{}do\right>\right) \Rightarrow{} \left<\left\{SELF\right\} \rightarrow{} \left[good\right]\right>\right>. t=-1 \langle{}1,0.9\rangle{}";
        FORMAT_LATEX.parse(input).unwrap().try_into_task().unwrap()
    }

    /// 构造一个格式化样本（漢文版本）
    /// * ⚠️其中有些部分可能会过时
    /// * 🚩【2024-03-20 01:22:26】目前就从相应字符串中解析得来
    pub(crate) fn _sample_task_han() -> Task {
        // 直接从文本构造词项
        let input = "预0.5、0.75、0.4算「（接连，「『ball』是【left】」，「（积，『SELF』，任一any，其一some ）是操作do」）得「『SELF』是【good】」」。 发生在-1 真1、0.9值";
        FORMAT_HAN.parse(input).unwrap().try_into_task().unwrap()
    }

    /// 用于给格式加上「自动解包并格式化内容」功能
    trait FormatResult {
        fn format_result(&self, result: &Narsese) -> String;
    }

    impl FormatResult for NarseseFormat {
        fn format_result(&self, result: &Narsese) -> String {
            match result {
                Narsese::Term(term) => self.format_term(term),
                Narsese::Sentence(sentence) => self.format_sentence(sentence),
                Narsese::Task(task) => self.format_task(task),
            }
        }
    }

    /// 先解析然后格式化
    fn _test_parse_and_format(format: &NarseseFormat, input: &str) -> String {
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
    fn _test_format_and_parse(format: &NarseseFormat, input: Task) -> Narsese {
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

    #[test]
    fn tests_ascii() {
        test_matrix! {
            FORMAT_ASCII;
            _test_parse_and_format => [
                "<(&/, <{powerup_good_front} --> [seen]>, +30000, <(*, {SELF}) --> ^right>, +30000) =/> <{SELF} --> [powered]>>. :|: %1.0;0.99%"
                "$$ 空预算要表示出来_空真值因为标点而无需必要. :|:"
            ]
            _test_format_and_parse => [
                _sample_task_ascii()
            ]
        }
    }

    #[test]
    fn tests_latex() {
        test_matrix! {
            FORMAT_LATEX;
            _test_parse_and_format => [
                r"\$0.5;0.75;0.4\$ \left<\left(,\; \left<\left\{ball\right\} \rightarrow{} \left[left\right]\right>\; \left<\left(\times{}\; \left\{SELF\right\}\; \$any\; \#some\right) \rightarrow{} \Uparrow{}do\right>\right) \Rightarrow{} \left<\left\{SELF\right\} \rightarrow{} \left[good\right]\right>\right>. t=-1 \langle{}1,0.9\rangle{}"
            ]
            _test_format_and_parse => [
                _sample_task_latex()
            ]
        }
    }

    #[test]
    fn tests_han() {
        test_matrix! {
            FORMAT_HAN;
            _test_parse_and_format => [
                "「我是谁」" // ! 先前的failed case
                "预0.5、0.75、0.4算
                「（接连，「『ball』是【left】」，「（积，『SELF』，任一any，其一some ）是操作do」）得「『SELF』是【good】」」。
                发生在-1 真1、0.9值"
            ]
            _test_format_and_parse => [
                _sample_task_han()
            ]
        }
    }
}

/// 集成测试 & 枚举Narsese
/// * 🎯利用「枚举Narsese」的「预置Narsese格式」生成「词法Narsese对象」
#[cfg(test)]
#[cfg(feature = "enum_narsese")]
mod tests_with_enum_narsese {
    use super::super::impl_enum::NarseseFormat as EnumNarseseFormat;
    use crate::lexical::{shortcut::*, Task};

    /// （通用）构造一个格式化样本
    /// * 基本涵盖其所属模块的全部内容
    /// * 📌其中还有一些「格式特有」的东西
    pub fn _sample_task(format: &EnumNarseseFormat<&str>) -> Task {
        // 构造词项
        let ball_left = statement!(
            atom!(format.atom.prefix_word, "ball")
            format.statement.copula_instance_property
            atom!(format.atom.prefix_word, "left")
        );
        let conditional_operation = compound!(
            format.compound.connecter_conjunction_sequential,
            ball_left.clone(),
            statement!(
                compound!(
                    format.compound.connecter_product,
                    set!(
                        format.compound.brackets_set_extension.0;
                        // ! ↓此处不一定是「空字串前缀」了
                        atom!(format.atom.prefix_word, "SELF");
                        format.compound.brackets_set_extension.1
                    ),
                    atom!(format.atom.prefix_variable_independent, "any"),
                    atom!(format.atom.prefix_variable_dependent, "some"),
                )
                format.statement.copula_inheritance
                atom!(format.atom.prefix_operator, "do")
            ),
        );
        let self_good = statement!(
            atom!(format.atom.prefix_word, "SELF")
            format.statement.copula_instance_property
            atom!(format.atom.prefix_word, "good")
        );
        let term = statement!(
            conditional_operation.clone()
            format.statement.copula_implication
            self_good.clone()
        );

        // 构造语句
        let truth = &truth!(
            format.sentence.truth_brackets.0;
            format.sentence.truth_separator; // * 没有装饰性空格
            "1.0" "0.9";
            format.sentence.truth_brackets.1;
        );
        let stamp = &stamp!(
            format.sentence.stamp_brackets.0;
            format.sentence.stamp_fixed;
            "-1";
            format.sentence.stamp_brackets.1
        );
        let punctuation = ".";
        // let sentence = sentence!(
        //     term.clone() "." stamp truth
        // ); // ! 此处无需构建；直接构建任务

        // 构造任务并返回
        let budget = &budget!(
            format.task.budget_brackets.0;
            format.task.budget_separator; // * 没有装饰性空格
            "0.5" "0.75" "0.4";
            format.task.budget_brackets.1
        );
        task!(budget term.clone() punctuation stamp truth) // * 📝【2024-03-09 10:48:31】Clippy推荐直接返回构造之后的值
    }
}
