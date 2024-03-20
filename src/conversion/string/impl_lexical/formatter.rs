//! 实现/格式化器

use super::NarseseFormat;
use crate::{
    api::{GetBudget, GetTerm},
    conversion::string::common_narsese_templates::*,
    lexical::{Sentence, Task, Term},
    util::{add_space_if_necessary_and_flush_buffer, catch_flow},
};

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
    pub fn format_term(&self, term: &Term) -> String {
        catch_flow!(self._format_term; term)
    }

    /// 格式化函数/语句
    /// * 返回一个新字符串
    pub fn format_sentence(&self, sentence: &Sentence) -> String {
        catch_flow!(self._format_sentence; sentence)
    }

    /// 总格式化函数/语句
    fn _format_sentence(&self, out: &mut String, sentence: &Sentence) {
        template_sentence(
            out,
            &self.format_term(sentence.get_term()),
            &sentence.punctuation,
            &sentence.stamp,
            &sentence.truth,
            &self.space.format_items,
        )
    }

    /// 格式化函数/任务
    /// * 返回一个新字符串
    pub fn format_task(&self, task: &Task) -> String {
        catch_flow!(self._format_task; task)
    }

    /// 总格式化函数/任务
    fn _format_task(&self, out: &mut String, task: &Task) {
        // 临时缓冲区 | 用于「有内容⇒添加空格」的逻辑
        let mut buffer = String::new();
        // 预算值
        out.push_str(task.get_budget());
        // 语句
        self._format_sentence(&mut buffer, task.get_sentence());
        add_space_if_necessary_and_flush_buffer(out, &mut buffer, &self.space.format_items);
    }
}

/// 单元测试
#[cfg(test)]
mod tests {

    #![allow(unused)]
    use super::*;
    use crate::{
        conversion::string::impl_lexical::format_instances::*,
        lexical::tests::_sample_task_ascii as _sample_task,
    };
    use util::f_parallel;

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
            // ! 词法格式对「真值」「预算值」「时间戳」保留原状不解析
            &FORMAT_ASCII "ascii"   "$0.5; 0.75; 0.4$ <(&/, <ball {-] left>, <(*, {SELF}, $any, #some) --> ^do>) ==> <SELF {-] good>>. :!-1: %1.0; 0.9%";
            &FORMAT_LATEX "latex" r#"$0.5; 0.75; 0.4$ \left<\left(&/\; \left<ball {-] left\right>\; \left<\left(*\; {SELF}\; $any\; #some\right) --> ^do\right>\right) ==> \left<SELF {-] good\right>\right>. :!-1: %1.0; 0.9%"#;
            &FORMAT_HAN   "漢"      "$0.5; 0.75; 0.4$ 「（&/，「ball{-]left」，「（*，{SELF}，$any，#some）-->^do」）==>「SELF{-]good」」. :!-1: %1.0; 0.9%";
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
    use util::f_parallel;

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
