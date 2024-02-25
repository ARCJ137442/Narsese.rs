//! 实现/格式化器

use super::format::*;
use crate::{sentence::*, task::*, term::*, util::*};

/// 实现：转换
///
/// ! ℹ️单元测试在[`super::formats`]模块中定义
impl NarseseFormat<&str> {
    /// 工具函数/原子词项
    fn format_atom(&self, out: &mut String, atom: &Term, prefix: &str) {
        out.push_str(prefix);
        out.push_str(&atom.get_atom_name_unchecked());
    }

    /// 工具函数/系列词项
    fn format_components(&self, out: &mut String, components: Vec<&Term>) {
        for (i, term) in components.iter().enumerate() {
            // 逗号
            if i != 0 {
                out.push_str(self.compound.separator);
                out.push_str(self.space.format_terms);
            }
            // 词项
            out.push_str(&self.format_term(term));
        }
    }

    /// 工具函数/词项集
    fn format_set(
        &self,
        out: &mut String,
        components: Vec<&Term>,
        bracket_left: &str,
        bracket_right: &str,
    ) {
        // 括号开始
        out.push_str(bracket_left);
        // 逐个词项加入
        self.format_components(out, components);
        // 括号结束
        out.push_str(bracket_right);
    }

    /// 工具函数/复合词项
    fn format_compound(&self, out: &mut String, components: Vec<&Term>, connecter: &str) {
        // 括号开始
        out.push_str(self.compound.brackets.0);
        // 连接符
        out.push_str(connecter);
        out.push_str(self.compound.separator);
        out.push_str(self.space.format_terms);
        // 逐个词项加入
        self.format_components(out, components);
        // 括号结束
        out.push_str(self.compound.brackets.1);
    }

    /// 工具函数/像
    fn format_image(
        &self,
        out: &mut String,
        index: usize,
        components: Vec<&Term>,
        connecter: &str,
    ) {
        // 左括号
        out.push_str(self.compound.brackets.0);
        // 连接符
        out.push_str(connecter);
        out.push_str(self.compound.separator);
        out.push_str(self.space.format_terms);
        //各个元素
        for (i, term) in components.iter().enumerate() {
            // 插入占位符
            if i == index {
                out.push_str(self.atom.prefix_placeholder);
                out.push_str(self.compound.separator);
                out.push_str(self.space.format_terms);
            }
            // 逗号
            if i != 0 {
                out.push_str(self.compound.separator);
                out.push_str(self.space.format_terms);
            }
            // 词项
            self._format_term(out, term);
        }
        // 右括号
        out.push_str(self.compound.brackets.1);
    }

    /// 工具函数/陈述
    fn format_statement(&self, out: &mut String, left: &Term, right: &Term, copula: &str) {
        out.push_str(self.statement.brackets.0);
        self._format_term(out, left);
        out.push_str(self.space.format_terms);
        out.push_str(copula);
        out.push_str(self.space.format_terms);
        self._format_term(out, right);
        out.push_str(self.statement.brackets.1);
    }

    /// 格式化函数/词项
    /// * 返回一个新字符串
    pub fn format_term(&self, term: &Term) -> String {
        let mut s = String::new();
        self._format_term(&mut s, term);
        s
    }

    /// 【内部】总格式化函数/词项
    fn _format_term(&self, out: &mut String, term: &Term) {
        match term {
            // 原子词项
            Word(..) => self.format_atom(out, term, self.atom.prefix_word),
            Placeholder => self.format_atom(out, term, self.atom.prefix_placeholder),
            VariableIndependent(..) => {
                self.format_atom(out, term, self.atom.prefix_variable_independent)
            }
            VariableDependent(..) => {
                self.format_atom(out, term, self.atom.prefix_variable_dependent)
            }
            VariableQuery(..) => self.format_atom(out, term, self.atom.prefix_variable_query),
            Interval(..) => self.format_atom(out, term, self.atom.prefix_interval),
            Operator(..) => self.format_atom(out, term, self.atom.prefix_operator),
            // 复合词项
            SetExtension(..) => self.format_set(
                out,
                term.get_components(),
                self.compound.brackets_set_extension.0,
                self.compound.brackets_set_extension.1,
            ),
            SetIntension(..) => self.format_set(
                out,
                term.get_components(),
                self.compound.brackets_set_intension.0,
                self.compound.brackets_set_intension.1,
            ),
            IntersectionExtension(..) => self.format_compound(
                out,
                term.get_components(),
                self.compound.connecter_intersection_extension,
            ),
            IntersectionIntension(..) => self.format_compound(
                out,
                term.get_components(),
                self.compound.connecter_intersection_intension,
            ),
            DifferenceExtension(..) => self.format_compound(
                out,
                term.get_components(),
                self.compound.connecter_difference_extension,
            ),
            DifferenceIntension(..) => self.format_compound(
                out,
                term.get_components(),
                self.compound.connecter_difference_intension,
            ),
            Product(..) => {
                self.format_compound(out, term.get_components(), self.compound.connecter_product)
            }
            ImageExtension(index, _) => self.format_image(
                out,
                *index,
                term.get_components(),
                self.compound.connecter_image_extension,
            ),
            ImageIntension(index, _) => self.format_image(
                out,
                *index,
                term.get_components(),
                self.compound.connecter_image_intension,
            ),
            Conjunction(..) => self.format_compound(
                out,
                term.get_components(),
                self.compound.connecter_conjunction,
            ),
            Disjunction(..) => self.format_compound(
                out,
                term.get_components(),
                self.compound.connecter_disjunction,
            ),
            Negation(..) => {
                self.format_compound(out, term.get_components(), self.compound.connecter_negation)
            }
            ConjunctionSequential(..) => self.format_compound(
                out,
                term.get_components(),
                self.compound.connecter_conjunction_sequential,
            ),
            ConjunctionParallel(..) => self.format_compound(
                out,
                term.get_components(),
                self.compound.connecter_conjunction_parallel,
            ),
            // 陈述
            Inheritance(left, right) => {
                self.format_statement(out, left, right, self.statement.copula_inheritance)
            }
            Similarity(left, right) => {
                self.format_statement(out, left, right, self.statement.copula_similarity)
            }
            Implication(left, right) => {
                self.format_statement(out, left, right, self.statement.copula_implication)
            }
            Equivalence(left, right) => {
                self.format_statement(out, left, right, self.statement.copula_equivalence)
            }
            ImplicationPredictive(left, right) => self.format_statement(
                out,
                left,
                right,
                self.statement.copula_implication_predictive,
            ),
            ImplicationConcurrent(left, right) => self.format_statement(
                out,
                left,
                right,
                self.statement.copula_implication_concurrent,
            ),
            ImplicationRetrospective(left, right) => self.format_statement(
                out,
                left,
                right,
                self.statement.copula_implication_retrospective,
            ),
            EquivalencePredictive(left, right) => self.format_statement(
                out,
                left,
                right,
                self.statement.copula_equivalence_predictive,
            ),
            EquivalenceConcurrent(left, right) => self.format_statement(
                out,
                left,
                right,
                self.statement.copula_equivalence_concurrent,
            ), // ! 「回顾性等价」未有
        }
    }

    /// 工具函数/浮点序列
    fn format_floats(
        &self,
        out: &mut String,
        bracket_left: &str,
        bracket_right: &str,
        separator: &str,
        floats: &[FloatPrecision],
    ) {
        out.push_str(bracket_left);
        for (i, f) in floats.iter().enumerate() {
            // 分隔符
            if i != 0 {
                out.push_str(separator);
                // out.push_str(self.space); // * 目前在OpenNARS、PyNARS中均未使用分隔符
            }
            out.push_str(&f.to_string());
        }
        out.push_str(bracket_right);
    }

    /// 工具函数/浮点序列/真值
    fn format_floats_truth(&self, out: &mut String, floats: &[FloatPrecision]) {
        self.format_floats(
            out,
            self.sentence.truth_brackets.0,
            self.sentence.truth_brackets.1,
            self.sentence.truth_separator,
            floats,
        );
    }

    /// 工具函数/浮点序列/预算值
    fn format_floats_budget(&self, out: &mut String, floats: &[FloatPrecision]) {
        self.format_floats(
            out,
            self.task.budget_brackets.0,
            self.task.budget_brackets.1,
            self.task.budget_separator,
            floats,
        );
    }

    /// 格式化函数/真值
    pub fn format_truth(&self, truth: &Truth) -> String {
        let mut out = String::new();
        self._format_truth(&mut out, truth);
        out
    }

    /// 总格式化函数/真值
    fn _format_truth(&self, out: &mut String, truth: &Truth) {
        match truth {
            // 空真值⇒直接为空
            Truth::Empty => {}
            // 单真值⇒单元素数组
            Truth::Single(f) => self.format_floats_truth(out, &[*f]),
            // 双真值⇒二元数组
            Truth::Double(f, c) => self.format_floats_truth(out, &[*f, *c]),
        }
    }

    /// 格式化函数/时间戳
    pub fn format_stamp(&self, stamp: &Stamp) -> String {
        let mut out = String::new();
        self._format_stamp(&mut out, stamp);
        out
    }

    /// 总格式化函数/时间戳
    fn _format_stamp(&self, out: &mut String, stamp: &Stamp) {
        // 永恒⇒无内容
        if stamp.is_eternal() {
            return;
        }
        // 括号开始
        out.push_str(self.sentence.stamp_brackets.0);
        // 添加内容
        match stamp {
            Stamp::Past => out.push_str(self.sentence.stamp_past),
            Stamp::Present => out.push_str(self.sentence.stamp_present),
            Stamp::Future => out.push_str(self.sentence.stamp_future),
            Stamp::Fixed(time) => {
                out.push_str(self.sentence.stamp_fixed);
                out.push_str(&time.to_string());
            }
            // * 这里实际上不可能出现
            Stamp::Eternal => {}
        }
        // 括号结束
        out.push_str(self.sentence.stamp_brackets.1);
    }

    /// 工具函数/有内容时前缀分隔符
    /// * 关键在「避免无用分隔符」
    fn add_space_if_necessary_and_flush_buffer(&self, out: &mut String, buffer: &mut String) {
        match buffer.is_empty() {
            // 空⇒不做动作
            true => {}
            // 非空⇒预置分隔符，推送并清空
            false => {
                out.push_str(self.space.format_items);
                out.push_str(buffer);
                buffer.clear();
            }
        }
    }

    /// 工具函数/标点
    fn format_punctuation(&self, out: &mut String, sentence: &Sentence) {
        out.push_str(match sentence {
            Judgement(..) => self.sentence.punctuation_judgement,
            Goal(..) => self.sentence.punctuation_goal,
            Question(..) => self.sentence.punctuation_question,
            Quest(..) => self.sentence.punctuation_quest,
        })
    }

    /// 格式化函数/语句
    ///
    /// ! ⚠️注意：没有独立的「标点」一说
    pub fn format_sentence(&self, sentence: &Sentence) -> String {
        let mut out = String::new();
        self._format_sentence(&mut out, sentence);
        out
    }

    /// 总格式化函数/语句
    fn _format_sentence(&self, out: &mut String, sentence: &Sentence) {
        // 临时缓冲区 | 用于「有内容⇒添加空格」的逻辑
        let mut buffer = String::new();
        // 词项 | 第一个直接输入
        self._format_term(out, sentence.get_term());
        // 标点 | 紧跟词项，无需分离
        self.format_punctuation(out, &sentence);
        // 时间戳
        self._format_stamp(&mut buffer, &sentence.get_stamp());
        self.add_space_if_necessary_and_flush_buffer(out, &mut buffer);
        // 真值 | 若无⇒当空真值对待
        self._format_truth(&mut buffer, sentence.get_truth().unwrap_or(&Truth::Empty));
        self.add_space_if_necessary_and_flush_buffer(out, &mut buffer);
    }

    /// 格式化函数/预算值
    pub fn format_budget(&self, budget: &Budget) -> String {
        let mut out = String::new();
        self._format_budget(&mut out, budget);
        out
    }

    /// 总格式化函数/预算值
    fn _format_budget(&self, out: &mut String, budget: &Budget) {
        match budget {
            // 空预算⇒直接为空
            Budget::Empty => {}
            // 单预算⇒单元素数组
            Budget::Single(p) => self.format_floats_budget(out, &[*p]),
            // 双预算⇒二元数组
            Budget::Double(p, d) => self.format_floats_budget(out, &[*p, *d]),
            // 三预算⇒三元数组
            Budget::Triple(p, d, q) => self.format_floats_budget(out, &[*p, *d, *q]),
        }
    }

    /// 格式化函数/任务
    pub fn format_task(&self, task: &Task) -> String {
        let mut out = String::new();
        self._format_task(&mut out, task);
        out
    }

    /// 总格式化函数/任务
    fn _format_task(&self, out: &mut String, task: &Task) {
        // 临时缓冲区 | 用于「有内容⇒添加空格」的逻辑
        let mut buffer = String::new();
        // 预算值
        self._format_budget(out, task.get_budget());
        // 语句
        self._format_sentence(&mut buffer, &task.get_sentence());
        self.add_space_if_necessary_and_flush_buffer(out, &mut buffer);
    }
}
