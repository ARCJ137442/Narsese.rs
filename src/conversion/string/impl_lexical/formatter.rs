//! 实现/格式化器

use super::NarseseFormat;
use crate::{
    api::{FormatTo, GetBudget, GetTerm},
    conversion::string::common_narsese_templates::*,
    lexical::{Budget, Narsese, Sentence, Task, Term, Truth},
};
use nar_dev_utils::{add_space_if_necessary_and_flush_buffer, catch_flow, join_to};

/// 实现：转换
///
/// ! ℹ️单元测试在[`super::formats`]模块中定义
impl NarseseFormat {
    /// 工具函数/词项
    fn _format_term(&self, out: &mut String, term: &Term) {
        match term {
            // 原子词项
            Term::Atom { prefix, name } => template_atom(out, prefix, name),
            // 复合词项（包括「像」）
            Term::Compound { connecter, terms } => template_compound(
                out,
                &self.compound.brackets.0,
                connecter,
                terms.iter().map(|term| self.format_term(term)),
                &self.compound.separator,
                &self.space.format_terms,
                &self.compound.brackets.1,
            ),
            // 复合词项集合
            Term::Set {
                left_bracket,
                terms,
                right_bracket,
            } => template_compound_set(
                out,
                left_bracket,
                terms.iter().map(|term| self.format_term(term)),
                &self.compound.separator,
                &self.space.format_terms,
                right_bracket,
            ),
            // 陈述
            Term::Statement {
                copula,
                subject,
                predicate,
            } => template_statement(
                out,
                &self.statement.brackets.0,
                &self.format_term(subject),
                copula,
                &self.format_term(predicate),
                &self.space.format_terms,
                &self.statement.brackets.1,
            ),
        }
    }

    /// 格式化函数/词项
    /// * 返回一个新字符串
    #[inline(always)]
    pub fn format_term(&self, term: &Term) -> String {
        catch_flow!(self._format_term; term)
    }

    /// 格式化函数/真值
    /// * 🚩【2024-03-22 23:19:22】返回的是**紧凑**形式，没有额外空白符！
    fn _format_truth(&self, out: &mut String, truth: &Truth) {
        // 空真值⇒提前返回
        if truth.is_empty() {
            return;
        }
        // 左括弧
        out.push_str(&self.sentence.truth_brackets.0);
        // 中间内容
        join_to(out, truth.iter(), &self.sentence.truth_separator);
        // 右括弧
        out.push_str(&self.sentence.truth_brackets.1);
    }

    /// 格式化函数/真值
    /// * 返回一个新字符串
    pub fn format_truth(&self, truth: &Truth) -> String {
        catch_flow!(self._format_truth; truth)
    }

    /// 格式化函数/语句
    fn _format_sentence(&self, out: &mut String, sentence: &Sentence) {
        template_sentence(
            out,
            &self.format_term(sentence.get_term()),
            &sentence.punctuation,
            &sentence.stamp,
            &self.format_truth(&sentence.truth),
            // ! ↑此处不用`.get_truth`，因为「可能没有」
            // * 并且「语义明确」失败：无法兼顾地让`get_truth`同时支持返回`Option<&Truth>`与`&Truth`
            // * 📄参考：[`GetTruth`]
            &self.space.format_items,
        )
    }

    /// 格式化函数/语句
    /// * 返回一个新字符串
    #[inline(always)]
    pub fn format_sentence(&self, sentence: &Sentence) -> String {
        catch_flow!(self._format_sentence; sentence)
    }

    /// 格式化函数/预算值
    /// * ❌【2024-03-24 03:14:29】不能「在空白时省略」：会遇到「空预算⇒被解析回语句」的混淆情况
    ///   * 📌目前面向「命令行输入」的解决方案：尝试将空预算转换成语句，然后按语句进行格式化并置入
    /// * 🚩【2024-03-22 23:19:22】返回的是**紧凑**形式，没有额外空白符！
    fn _format_budget(&self, out: &mut String, budget: &Budget) {
        // 左括弧
        out.push_str(&self.task.budget_brackets.0);
        // 中间内容
        join_to(out, budget.iter(), &self.task.budget_separator);
        // 右括弧
        out.push_str(&self.task.budget_brackets.1);
    }

    /// 格式化函数/预算值
    /// * 返回一个新字符串
    pub fn format_budget(&self, budget: &Budget) -> String {
        catch_flow!(self._format_budget; budget)
    }

    /// 格式化函数/任务
    fn _format_task(&self, out: &mut String, task: &Task) {
        // 临时缓冲区 | 用于「有内容⇒添加空格」的逻辑
        let mut buffer = String::new();
        // 预算值 | 第一个直接添加
        self._format_budget(out, task.get_budget());
        // 语句
        self._format_sentence(&mut buffer, task.get_sentence());
        add_space_if_necessary_and_flush_buffer(out, &mut buffer, &self.space.format_items);
    }

    /// 格式化函数/任务
    /// * 返回一个新字符串
    #[inline(always)]
    pub fn format_task(&self, task: &Task) -> String {
        catch_flow!(self._format_task; task)
    }

    /// 格式化函数/Narsese
    fn _format_narsese(&self, out: &mut String, narsese: &Narsese) {
        match narsese {
            // 词项
            Narsese::Term(term) => self._format_term(out, term),
            // 语句
            Narsese::Sentence(sentence) => self._format_sentence(out, sentence),
            // 任务
            Narsese::Task(task) => self._format_task(out, task),
        }
    }

    /// 格式化函数/Narsese
    /// * 🚩自动分派
    pub fn format_narsese(&self, narsese: &Narsese) -> String {
        catch_flow!(self._format_narsese; narsese)
    }

    /// 总格式化函数/基于[`FormatTo`]特征
    pub fn format<'a>(&'a self, from: &impl FormatTo<&'a Self, String>) -> String {
        from.format_to(self)
    }
}

/// 词项的格式化接口
impl FormatTo<&NarseseFormat, String> for Term {
    fn format_to(&self, formatter: &NarseseFormat) -> String {
        formatter.format_term(self)
    }
}

/// 真值的格式化接口
/// * ⚠️【2024-04-05 02:29:09】目前实际上是「字符串数组」而非独立的类型
impl FormatTo<&NarseseFormat, String> for Truth {
    fn format_to(&self, formatter: &NarseseFormat) -> String {
        formatter.format_truth(self)
    }
}

/// 语句的格式化接口
impl FormatTo<&NarseseFormat, String> for Sentence {
    fn format_to(&self, formatter: &NarseseFormat) -> String {
        formatter.format_sentence(self)
    }
}

// /// 预算值的格式化接口
// /// * ⚠️【2024-04-05 02:29:09】目前实际上是「字符串数组」
// ///   * 🚩故与「真值」冲突，不再独立实现
// impl FormatTo<&NarseseFormat, String> for Budget {
//     fn format_to(&self, formatter: &NarseseFormat) -> String {
//         formatter.format_budget(self)
//     }
// }

/// 任务的格式化接口
impl FormatTo<&NarseseFormat, String> for Task {
    fn format_to(&self, formatter: &NarseseFormat) -> String {
        formatter.format_task(self)
    }
}

// * ✅Narsese的格式化接口已自动实现

/// 单元测试
#[cfg(test)]
mod tests {

    #![allow(unused)]
    use super::*;
    use crate::{
        conversion::string::impl_lexical::format_instances::*,
        lexical::tests::_sample_task_ascii as _sample_task,
    };
    use nar_dev_utils::f_parallel;

    /// 测试其中一个格式
    fn _test(format: &NarseseFormat, name: &str, expected: &str) {
        // 声明
        println!("Test of {name}");
        // 构造样本任务
        let task = _sample_task();
        // 格式化
        let formatted = format.format_task(&task);
        // 展示
        dbg!(&formatted);
        // 断言
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test() {
        // let truth_str = "$0.5; 0.75; 0.4";
        // let budget_str = "$0.5; 0.75; 0.4";
        // let stamp_str = ":!-1:";
        // 平行测试
        f_parallel![
            _test;
            // ! 注意：此处是「用ASCII的值套对应的本地格式」
            //   ! 不受影响的词项元素有：复合词项连接词、集合词项左右括弧、陈述系词等
            // ! 词法格式对「时间戳」保留原状不解析
            //   ! 【2024-03-22 23:23:01】现在对「真值」「预算值」能应用相应格式了
            // ! 🚩【2024-03-22 23:21:19】对于「真值」「预算值」一律采用「紧凑模式」
            &FORMAT_ASCII "ascii"   "$0.5;0.75;0.4$ <(&/, <ball {-] left>, <(*, {SELF}, $any, #some) --> ^go-to>) ==> <SELF {-] good>>. :!-1: %1.0;0.9%";
            &FORMAT_LATEX "latex" r#"\$0.5;0.75;0.4\$ \left<\left(&/\; \left<ball {-] left\right>\; \left<\left(*\; {SELF}\; $any\; #some\right) --> ^go-to\right>\right) ==> \left<SELF {-] good\right>\right>. :!-1: \langle{}1.0,0.9\rangle{}"#;
            &FORMAT_HAN   "漢"      "预0.5、0.75、0.4算 「（&/，「ball{-]left」，「（*，{SELF}，$any，#some）-->^go-to」）==>「SELF{-]good」」. :!-1: 真1.0、0.9值";
        ];
    }
}

/// 单元测试 & 枚举Narsese
/// * 🚩只用到了「使用枚举Narsese生成的测试用例」而不会用到其它东西
///   * 🏗️仍需继续处理与「枚举Narsese」的关系
#[cfg(feature = "enum_narsese")]
#[cfg(test)]
mod tests_with_enum_narsese {

    use super::super::tests_with_enum_narsese::_sample_task;
    use crate::conversion::string::{
        impl_enum::{
            format_instances::{
                FORMAT_ASCII as F_E_ASCII, FORMAT_HAN as F_E_HAN, FORMAT_LATEX as F_E_LATEX,
            },
            NarseseFormat as EnumNarseseFormat,
        },
        impl_lexical::{format_instances::*, NarseseFormat},
    };
    use nar_dev_utils::f_parallel;

    /// 测试其中一个格式
    fn _test(
        format_enum: &EnumNarseseFormat<&str>,
        format: &NarseseFormat,
        name: &str,
        expected: &str,
    ) {
        // 声明
        println!("Test of {name}");
        // 构造样本任务
        let task = _sample_task(format_enum);
        // 格式化
        let formatted = format.format_task(&task);
        // 展示
        dbg!(&formatted);
        // 断言
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test() {
        // 平行测试
        f_parallel![
            _test;
            // ! 此处是根据「由『枚举Narsese』提供的信息生成的『词法Narsese』」格式化而来
            // ! 所以能穿透真值、预算值、时间戳的格式化（本地化）
            &F_E_ASCII, &FORMAT_ASCII, "ascii",   "$0.5;0.75;0.4$ <(&/, <ball {-] left>, <(*, {SELF}, $any, #some) --> ^do>) ==> <SELF {-] good>>. :!-1: %1.0;0.9%";
            &F_E_LATEX, &FORMAT_LATEX, "latex", r#"\$0.5;0.75;0.4\$ \left<\left(,\; \left<ball \circ\!\!\!\rightarrow\!\!\!\circ{} left\right>\; \left<\left(\times{}\; \left\{SELF\right\}\; \$any\; \#some\right) \rightarrow{} \Uparrow{}do\right>\right) \Rightarrow{} \left<SELF \circ\!\!\!\rightarrow\!\!\!\circ{} good\right>\right>. t=-1 \langle{}1.0,0.9\rangle{}"#;
            &F_E_HAN,   &FORMAT_HAN,   "漢",      "预0.5、0.75、0.4算 「（接连，「ball具有left」，「（积，『SELF』，任一any，其一some）是操作do」）得「SELF具有good」」. 发生在-1 真1.0、0.9值";
        ];
    }
}
