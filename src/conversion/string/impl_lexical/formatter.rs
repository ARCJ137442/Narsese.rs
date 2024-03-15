//! 实现/格式化器

use crate::{
    api::{GetBudget, GetTerm},
    conversion::string::common_narsese_templates::*,
    lexical::{LexicalSentence, LexicalTask, LexicalTerm},
    util::{add_space_if_necessary_and_flush_buffer, catch_flow},
};

use super::NarseseFormat;

/// 实现：转换
///
/// ! ℹ️单元测试在[`super::formats`]模块中定义
impl<'a> NarseseFormat<'a> {
    /// 工具函数/词项
    fn _format_term(&self, out: &mut String, term: &LexicalTerm) {
        match term {
            // 原子词项
            LexicalTerm::Atom { prefix, name } => template_atom(out, prefix, name),
            // 复合词项（包括「像」）
            LexicalTerm::Compound { connecter, terms } => template_compound(
                out,
                self.compound.brackets.0,
                connecter,
                terms.iter().map(|term| self.format_term(term)),
                self.compound.separator,
                self.space.format_terms,
                self.compound.brackets.1,
            ),
            // 复合词项集合
            LexicalTerm::Set {
                left_bracket,
                terms,
                right_bracket,
            } => template_compound_set(
                out,
                left_bracket,
                terms.iter().map(|term| self.format_term(term)),
                self.compound.separator,
                self.space.format_terms,
                right_bracket,
            ),
            // 陈述
            LexicalTerm::Statement {
                copula,
                subject,
                predicate,
            } => template_statement(
                out,
                self.statement.brackets.0,
                &self.format_term(subject),
                copula,
                &self.format_term(predicate),
                self.space.format_terms,
                self.statement.brackets.1,
            ),
        }
    }

    /// 格式化函数/词项
    /// * 返回一个新字符串
    pub fn format_term(&self, term: &LexicalTerm) -> String {
        catch_flow!(self._format_term; term)
    }

    /// 格式化函数/语句
    pub fn format_sentence(&self, sentence: &LexicalSentence) -> String {
        catch_flow!(self._format_sentence; sentence)
    }

    /// 总格式化函数/语句
    fn _format_sentence(&self, out: &mut String, sentence: &LexicalSentence) {
        template_sentence(
            out,
            &self.format_term(sentence.get_term()),
            &sentence.punctuation,
            &sentence.stamp,
            &sentence.truth,
            self.space.format_items,
        )
    }

    /// 格式化函数/任务
    pub fn format_task(&self, task: &LexicalTask) -> String {
        catch_flow!(self._format_task; task)
    }

    /// 总格式化函数/任务
    fn _format_task(&self, out: &mut String, task: &LexicalTask) {
        // 临时缓冲区 | 用于「有内容⇒添加空格」的逻辑
        let mut buffer = String::new();
        // 预算值
        out.push_str(task.get_budget());
        // 语句
        self._format_sentence(&mut buffer, task.get_sentence());
        add_space_if_necessary_and_flush_buffer(out, &mut buffer, self.space.format_items);
    }
}

/// 单元测试
#[cfg(test)]
mod tests {

    #![allow(unused)]
    use super::super::tests::_sample_task_ascii as _sample_task;
    use super::*;
    use util::{f_parallel, show};

    /// 测试其中一个格式
    fn _test(name: &str, expected: &str) {
        // 声明
        println!("Test of {name}");
        todo!("🚧先做好自己本地的Narsese格式");
        // // 构造样本任务
        // let task = _sample_task();
        // // 格式化
        // let formatted = format.format_task(&task);
        // // 展示
        // show!(&formatted);
        // // 断言
        // assert_eq!(formatted, expected);
    }

    #[test]
    fn test() {
        // 平行测试
        todo!("🚧先做好自己本地的Narsese格式");
        // f_parallel![
        //     _test;
        //     FORMAT_ASCII "ascii" "$0.5;0.75;0.4$ <(&/, <ball {-] left>, <(*, {SELF}, $any, #some) --> ^do>) ==> <SELF {-] good>>. :!-1: %1.0;0.9%";
        //     FORMAT_LATEX "latex" r#"\$0.5;0.75;0.4\$ \left<\left(,  \left<ball \circ\!\!\!\rightarrow\!\!\!\circ   left\right>  \left<\left(\times   \left\{SELF\right\}  \$any  \#some\right) \rightarrow  \Uparrow do\right>\right) \Rightarrow  \left<SELF \circ\!\!\!\rightarrow\!\!\!\circ   good\right>\right>. t=-1 \langle1.0,0.9\rangle"#;
        //     FORMAT_HAN "漢" "预0.5、0.75、0.4算 「（接连，「ball具有left」，「（积，『SELF』，任一any，其一some）是操作do」）得「SELF具有good」」. 发生在-1 真1.0、0.9值";
        // ];
    }
}

/// 单元测试 & 枚举Narsese
/// * 🚩只用到了「使用枚举Narsese生成的测试用例」而不会用到其它东西
///   * 🏗️仍需继续处理与「枚举Narsese」的关系
#[cfg(feature = "enum_narsese")]
#[cfg(test)]
mod tests_with_enum_narsese {

    #![allow(unused)]
    use super::super::tests_with_enum_narsese::_sample_task;
    use crate::conversion::string::impl_enum::NarseseFormat as EnumNarseseFormat;
    use util::{f_parallel, show};

    /// 测试其中一个格式
    fn _test(format: EnumNarseseFormat<&str>, name: &str, expected: &str) {
        // 声明
        println!("Test of {name}");
        // 构造样本任务
        let task = _sample_task(&format);
        todo!("❓后续需要「从『枚举Narsese格式』中生成」，以便支持『自枚举Narsese转换』")
        // // 格式化
        // let formatted = format.format_task(&task);
        // // 展示
        // show!(&formatted);
        // // 断言
        // assert_eq!(formatted, expected);
    }

    #[test]
    fn test() {
        // 平行测试
        todo!("❓后续需要「从『枚举Narsese格式』中生成」以便使用");
        // f_parallel![
        //     _test;
        //     FORMAT_ASCII "ascii" "$0.5;0.75;0.4$ <(&/, <ball {-] left>, <(*, {SELF}, $any, #some) --> ^do>) ==> <SELF {-] good>>. :!-1: %1.0;0.9%";
        //     FORMAT_LATEX "latex" r#"\$0.5;0.75;0.4\$ \left<\left(,  \left<ball \circ\!\!\!\rightarrow\!\!\!\circ   left\right>  \left<\left(\times   \left\{SELF\right\}  \$any  \#some\right) \rightarrow  \Uparrow do\right>\right) \Rightarrow  \left<SELF \circ\!\!\!\rightarrow\!\!\!\circ   good\right>\right>. t=-1 \langle1.0,0.9\rangle"#;
        //     FORMAT_HAN "漢" "预0.5、0.75、0.4算 「（接连，「ball具有left」，「（积，『SELF』，任一any，其一some）是操作do」）得「SELF具有good」」. 发生在-1 真1.0、0.9值";
        // ];
    }
}
