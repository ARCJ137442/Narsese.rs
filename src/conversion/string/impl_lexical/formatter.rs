//! 实现/格式化器

use crate::{
    api::{GetBudget, GetTerm},
    catch_flow,
    conversion::string::common::*,
    lexical::{LexicalSentence, LexicalTask, LexicalTerm},
    util::add_space_if_necessary_and_flush_buffer,
};

/// 实现：转换
///
/// ! ℹ️单元测试在[`super::formats`]模块中定义
impl NarseseFormat<&str> {
    /// 工具函数/词项
    fn _format_lexical_term(&self, out: &mut String, term: &LexicalTerm) {
        match term {
            // 原子词项
            LexicalTerm::Atom { prefix, name } => Self::template_atom(out, prefix, name),
            // 复合词项（包括「像」）
            LexicalTerm::Compound { connecter, terms } => Self::template_compound(
                out,
                self.compound.brackets.0,
                connecter,
                terms.iter().map(|term| self.format_lexical_term(term)),
                self.compound.separator,
                self.space.format_terms,
                self.compound.brackets.1,
            ),
            // 复合词项集合
            LexicalTerm::Set {
                left_bracket,
                terms,
                right_bracket,
            } => Self::template_compound_set(
                out,
                left_bracket,
                terms.iter().map(|term| self.format_lexical_term(term)),
                self.compound.separator,
                self.space.format_terms,
                right_bracket,
            ),
            // 陈述
            LexicalTerm::Statement {
                copula,
                subject,
                predicate,
            } => Self::template_statement(
                out,
                self.statement.brackets.0,
                &self.format_lexical_term(subject),
                copula,
                &self.format_lexical_term(predicate),
                self.space.format_terms,
                self.statement.brackets.1,
            ),
        }
    }

    /// 格式化函数/词项
    /// * 返回一个新字符串
    pub fn format_lexical_term(&self, term: &LexicalTerm) -> String {
        catch_flow!(self._format_lexical_term; term)
    }

    /// 格式化函数/语句
    pub fn format_lexical_sentence(&self, sentence: &LexicalSentence) -> String {
        catch_flow!(self._format_lexical_sentence; sentence)
    }

    /// 总格式化函数/语句
    fn _format_lexical_sentence(&self, out: &mut String, sentence: &LexicalSentence) {
        Self::template_sentence(
            out,
            &self.format_lexical_term(sentence.get_term()),
            &sentence.punctuation,
            &sentence.stamp,
            &sentence.truth,
            self.space.format_items,
        )
    }

    /// 格式化函数/任务
    pub fn format_lexical_task(&self, task: &LexicalTask) -> String {
        catch_flow!(self._format_lexical_task; task)
    }

    /// 总格式化函数/任务
    fn _format_lexical_task(&self, out: &mut String, task: &LexicalTask) {
        // 临时缓冲区 | 用于「有内容⇒添加空格」的逻辑
        let mut buffer = String::new();
        // 预算值
        out.push_str(task.get_budget());
        // 语句
        self._format_lexical_sentence(&mut buffer, task.get_sentence());
        add_space_if_necessary_and_flush_buffer(out, &mut buffer, self.space.format_items);
    }
}

/// 单元测试
#[cfg(test)]
mod tests {

    use super::super::super::common::format_instances::*;
    use super::super::tests_lexical::_sample_task;
    use super::*;
    use crate::{f_parallel, show};

    /// 测试其中一个格式
    fn _test(format: NarseseFormat<&str>, name: &str, expected: &str) {
        // 声明
        println!("Test of {name}");
        // 构造样本任务
        let task = _sample_task(&format);
        // 格式化
        let formatted = format.format_lexical_task(&task);
        // 展示
        show!(&formatted);
        // 断言
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test() {
        // 平行测试
        f_parallel![
            _test;
            FORMAT_ASCII "ascii" "$0.5;0.75;0.4$ <(&/, <ball {-] left>, <(*, {SELF}, $any, #some) --> ^do>) ==> <SELF {-] good>>. :!-1: %1.0;0.9%";
            FORMAT_LATEX "latex" r#"\$0.5;0.75;0.4\$ \left<\left(,  \left<ball \circ\!\!\!\rightarrow\!\!\!\circ   left\right>  \left<\left(\times   \left\{SELF\right\}  \$any  \#some\right) \rightarrow  \Uparrow do\right>\right) \Rightarrow  \left<SELF \circ\!\!\!\rightarrow\!\!\!\circ   good\right>\right>. t=-1 \langle1.0,0.9\rangle"#;
            FORMAT_HAN "漢" "预0.5、0.75、0.4算 「（接连，「ball具有left」，「（积，『SELF』，任一any，其一some）是操作do」）得「SELF具有good」」. 发生在-1 真1.0、0.9值";
        ];
    }
}
