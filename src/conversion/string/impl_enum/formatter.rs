//! 实现/格式化器

use super::NarseseFormat;
use crate::{
    api::{
        FloatPrecision, FormatTo, GetBudget, GetPunctuation, GetStamp, GetTerm, GetTruth,
        UIntPrecision,
    },
    conversion::string::common_narsese_templates::*,
    enum_narsese::*,
};
use util::*;

/// 实现：转换
/// * 🚩【2024-04-05 01:47:08】目前保留方法调用上的兼容性
///   * 📌类型特定的[`format_term`]等方法调用与`term.format_to`并存
///   * 📌有利于代码组织紧凑性：相互调用的函数尽可能放在一起
///
/// ! ℹ️单元测试在[`super::formats`]模块中定义
///
impl NarseseFormat<&str> {
    // ! 🚩现在「纯字符串模板」已被提取到`common`模块

    // 针对「枚举Narsese」的格式化 //

    /// 工具函数/原子词项
    fn format_atom(&self, out: &mut String, atom: &Term, prefix: &str) {
        template_atom(out, prefix, &atom.get_atom_name_unchecked());
    }

    /// 工具函数/词项集
    fn format_set(
        &self,
        out: &mut String,
        components: Vec<&Term>,
        bracket_left: &str,
        bracket_right: &str,
    ) {
        template_compound_set(
            out,
            bracket_left,
            // 批量将内部词项转换成字符串
            components.iter().map(|term| self.format_term(term)),
            self.compound.separator,
            self.space.format_terms,
            bracket_right,
        );
    }

    /// 工具函数/复合词项
    fn format_compound(&self, out: &mut String, components: Vec<&Term>, connecter: &str) {
        template_compound(
            out,
            self.compound.brackets.0,
            connecter,
            components.iter().map(|term| self.format_term(term)),
            self.compound.separator,
            self.space.format_terms,
            self.compound.brackets.1,
        );
    }

    /// 工具函数/像
    fn format_image(
        &self,
        out: &mut String,
        index: UIntPrecision,
        components: Vec<&Term>,
        connecter: &str,
    ) {
        template_compound(
            out,
            self.compound.brackets.0,
            connecter,
            // 通过特殊的迭代器，连同占位符一起迭代
            ImageIterator::new(
                // * 建立迭代器并复制其中的引用（`&&Term => &Term`）
                // * 📝Clippy：可简化`.map(|&term| term)`为`.copied()`
                components.iter().copied(),
                index,
            )
            .map(|term| self.format_term(term)),
            self.compound.separator,
            self.space.format_terms,
            self.compound.brackets.1,
        )
    }

    /// 工具函数/陈述
    fn format_statement(&self, out: &mut String, left: &Term, right: &Term, copula: &str) {
        template_statement(
            out,
            self.statement.brackets.0,
            // 左边
            &self.format_term(left),
            // 连接符
            copula,
            // 右边
            &self.format_term(right),
            // 空格
            self.space.format_terms,
            // 右边
            self.statement.brackets.1,
        )
    }

    /// 格式化函数/词项
    /// * 返回一个新字符串
    pub fn format_term(&self, term: &Term) -> String {
        // 创建一个新字符串
        let mut s = String::new();
        // 对字符串注入格式化文本
        self._format_term(&mut s, term);
        // 返回注入后的字符串
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
        use Truth::*;
        match truth {
            // 空真值⇒直接为空
            Empty => {}
            // 单真值⇒单元素数组
            Single(f) => self.format_floats_truth(out, &[*f]),
            // 双真值⇒二元数组
            Double(f, c) => self.format_floats_truth(out, &[*f, *c]),
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
        use Stamp::*;
        match stamp {
            Past => out.push_str(self.sentence.stamp_past),
            Present => out.push_str(self.sentence.stamp_present),
            Future => out.push_str(self.sentence.stamp_future),
            Fixed(time) => {
                out.push_str(self.sentence.stamp_fixed);
                out.push_str(&time.to_string());
            }
            // * 这里实际上无需处理：默认为 Eternal
            Eternal => {}
        }
        // 括号结束
        out.push_str(self.sentence.stamp_brackets.1);
    }

    /// 总格式化函数/标点
    pub fn format_punctuation(&self, punctuation: &Punctuation) -> String {
        manipulate!(
            String::new()
            => [self._format_punctuation](_, punctuation)
        )
    }

    /// 格式化函数/标点
    fn _format_punctuation(&self, out: &mut String, punctuation: &Punctuation) {
        use Punctuation::*;
        out.push_str(match punctuation {
            Judgement => self.sentence.punctuation_judgement,
            Goal => self.sentence.punctuation_goal,
            Question => self.sentence.punctuation_question,
            Quest => self.sentence.punctuation_quest,
        })
    }

    /// 格式化函数/语句
    ///
    /// ! ⚠️注意：没有独立的「标点」一说
    pub fn format_sentence(&self, sentence: &Sentence) -> String {
        catch_flow!(self._format_sentence; sentence)
    }

    /// 总格式化函数/语句
    fn _format_sentence(&self, out: &mut String, sentence: &Sentence) {
        template_sentence(
            out,
            // 词项
            &catch_flow!(self._format_term; &sentence.get_term()),
            // 标点
            &catch_flow!(self._format_punctuation; &sentence.get_punctuation()),
            // 时间戳
            &catch_flow!(self._format_stamp; &sentence.get_stamp()),
            // 真值 | 默认空真值（对「问题」「请求」而言）
            &catch_flow!(self._format_truth; &sentence.get_truth().unwrap_or(&Truth::Empty)),
            // 分隔用空格
            self.space.format_terms,
        );
    }

    /// 格式化函数/预算值
    pub fn format_budget(&self, budget: &Budget) -> String {
        catch_flow!(self._format_budget; budget)
    }

    /// 总格式化函数/预算值
    fn _format_budget(&self, out: &mut String, budget: &Budget) {
        use Budget::*;
        match budget {
            // 空预算⇒空数组，仅含括弧 // ! 若无括弧，解析器将识别成语句
            Empty => self.format_floats_budget(out, &[]),
            // 单预算⇒单元素数组
            Single(p) => self.format_floats_budget(out, &[*p]),
            // 双预算⇒二元数组
            Double(p, d) => self.format_floats_budget(out, &[*p, *d]),
            // 三预算⇒三元数组
            Triple(p, d, q) => self.format_floats_budget(out, &[*p, *d, *q]),
        }
    }

    /// 格式化函数/任务
    pub fn format_task(&self, task: &Task) -> String {
        catch_flow!(self._format_task; task)
    }

    /// 总格式化函数/任务
    fn _format_task(&self, out: &mut String, task: &Task) {
        // 临时缓冲区 | 用于「有内容⇒添加空格」的逻辑
        let mut buffer = String::new();
        // 预算值
        self._format_budget(out, task.get_budget());
        // 语句
        self._format_sentence(&mut buffer, task.get_sentence());
        // 添加空格
        add_space_if_necessary_and_flush_buffer(out, &mut buffer, self.space.format_items);
    }

    /// 格式化函数/Narsese
    /// * 🚩自动分派
    pub fn format_narsese(&self, narsese: &Narsese) -> String {
        catch_flow!(self._format_narsese; narsese)
    }

    /// 总格式化函数/Narsese
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

    /// 总格式化函数/基于[`FormatTo`]特征
    pub fn format<'a>(&'a self, from: &impl FormatTo<&'a Self, String>) -> String {
        from.format_to(self)
    }
}

/// 词项的格式化接口
impl FormatTo<&NarseseFormat<&str>, String> for Term {
    fn format_to(&self, formatter: &NarseseFormat<&str>) -> String {
        formatter.format_term(self)
    }
}

/// 真值的格式化接口
impl FormatTo<&NarseseFormat<&str>, String> for Truth {
    fn format_to(&self, formatter: &NarseseFormat<&str>) -> String {
        formatter.format_truth(self)
    }
}

/// 时间戳的格式化接口
impl FormatTo<&NarseseFormat<&str>, String> for Stamp {
    fn format_to(&self, formatter: &NarseseFormat<&str>) -> String {
        formatter.format_stamp(self)
    }
}

/// 标点的格式化接口
impl FormatTo<&NarseseFormat<&str>, String> for Punctuation {
    fn format_to(&self, formatter: &NarseseFormat<&str>) -> String {
        formatter.format_punctuation(self)
    }
}

/// 语句的格式化接口
impl FormatTo<&NarseseFormat<&str>, String> for Sentence {
    fn format_to(&self, formatter: &NarseseFormat<&str>) -> String {
        formatter.format_sentence(self)
    }
}

/// 预算值的格式化接口
impl FormatTo<&NarseseFormat<&str>, String> for Budget {
    fn format_to(&self, formatter: &NarseseFormat<&str>) -> String {
        formatter.format_budget(self)
    }
}

/// 任务的格式化接口
impl FormatTo<&NarseseFormat<&str>, String> for Task {
    fn format_to(&self, formatter: &NarseseFormat<&str>) -> String {
        formatter.format_task(self)
    }
}

// * ✅Narsese的格式化接口已自动实现

/// 单元测试
#[cfg(test)]
mod test {
    use super::{
        super::{format_instances::*, tests::_sample_task},
        *,
    };
    use util::{f_parallel, show};

    /// 测试其中一个格式
    fn _test(format: NarseseFormat<&str>, name: &str, expected: &str) {
        // 声明
        println!("Test of {name}");
        // 构造样本任务
        let task = _sample_task();
        // 格式化
        let formatted = format.format_task(&task);
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
            FORMAT_ASCII "ascii"   "$0.5;0.75;0.4$ <(&/, <{ball} --> [left]>, <(*, {SELF}, $any, #some) --> ^do>) ==> <{SELF} --> [good]>>. :!-1: %1;0.9%";
            FORMAT_LATEX "latex" r#"\$0.5;0.75;0.4\$ \left<\left(,\; \left<\left\{ball\right\} \rightarrow{} \left[left\right]\right>\; \left<\left(\times{}\; \left\{SELF\right\}\; \$any\; \#some\right) \rightarrow{} \Uparrow{}do\right>\right) \Rightarrow{} \left<\left\{SELF\right\} \rightarrow{} \left[good\right]\right>\right>. t=-1 \langle{}1,0.9\rangle{}"#;
            FORMAT_HAN   "漢"      "预0.5、0.75、0.4算 「（接连，「『ball』是【left】」，「（积，『SELF』，任一any，其一some）是操作do」）得「『SELF』是【good】」」。发生在-1真1、0.9值";
        ];
    }
}
