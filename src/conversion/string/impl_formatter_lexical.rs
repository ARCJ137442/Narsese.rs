//! 实现/格式化器

use super::format::*;
use crate::{catch_flow, lexical::*, traits::*, util::*};

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
        self._format_lexical_sentence(&mut buffer, &task.get_sentence());
        add_space_if_necessary_and_flush_buffer(out, &mut buffer, self.space.format_items);
    }
}
