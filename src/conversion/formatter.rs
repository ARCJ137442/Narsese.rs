//! 记录Narsese的格式与格式化器
//! * 部分代码参照自JuNarsese
//!
//! 📝词项类型分类树
//! * 原子词项
//!   * 1 词语
//!   * 6 独立变量
//!   * 6 非独变量
//!   * 6 查询变量
//!   * 7 间隔
//! * 复合词项
//!   * 3 外延集
//!   * 3 内涵集
//!   * 3 外延交
//!   * 3 内涵交
//!   * 3 外延差
//!   * 3 内涵差
//!   * 4 乘积
//!   * 4 外延像
//!   * 4 内涵像
//!   * 5 合取
//!   * 5 析取
//!   * 5 否定
//!   * 7 顺序合取
//!   * 7 平行合取
//! * 陈述
//!   * 1 继承
//!   * 2 相似
//!   * 5 蕴含
//!   * 5 等价

use crate::{sentence::*, task::*, term::*, util::*};

/// Narsese格式/原子词项
/// * 格式预期：`{前缀}+词项字符串名`
///   * 📌将「像占位符」也包含在内——相当于「只有前缀，没有内容」的词项
/// * 核心：存储各个原子词项的**前缀**
#[derive(Debug)]
pub struct NarseseFormatAtom<Content> {
    /// 前缀/词语 | ``
    pub prefix_word: Content,
    /// 前缀/独立变量 | `$`
    pub prefix_variable_independent: Content,
    /// 前缀/非独变量 | `#`
    pub prefix_variable_dependent: Content,
    /// 前缀/查询变量 | `?`
    pub prefix_variable_query: Content,
    /// 前缀/间隔 | `+`
    pub prefix_interval: Content,
    /// 前缀/操作符 | `^`
    pub prefix_operator: Content,
    /// 前缀/像占位符 | `_`
    pub prefix_placeholder: Content,
}

/// Narsese格式/复合词项
/// * 格式预期：`({连接符}, 词项...)`
/// * 核心：存储各个原子词项的**连接符**
///
/// 📌此举专用于解析CommonNarsese
/// * 不考虑其它idea 如「将 外延集/内涵集 也变成`({连接符}, 词项...)`的形式」
#[derive(Debug)]
pub struct NarseseFormatCompound<Content> {
    // 通用 //
    /// 首尾括弧 | `(` `)`
    pub brackets: (Content, Content),
    /// 词项分隔符 | `,`
    pub separator: Content,

    // 专用 //
    /// 首尾括弧/外延集 | `{` `}`
    pub brackets_set_extension: (Content, Content),
    /// 首尾括弧/内涵集 | `[` `]`
    pub brackets_set_intension: (Content, Content),
    /// 连接符/外延交集 | `&`
    pub connecter_intersection_extension: Content,
    /// 连接符/内涵交集 | `|`
    pub connector_intersection_intension: Content,
    /// 连接符/外延差集 | `-`
    pub connecter_difference_extension: Content,
    /// 连接符/内涵差集 | `~`
    pub connecter_difference_intension: Content,
    /// 连接符/乘积 | `*`
    pub connecter_product: Content,
    /// 连接符/外延像 | `/`
    pub connecter_image_extension: Content,
    /// 连接符/内涵像 | `\`
    pub connecter_image_intension: Content,
    /// 连接符/合取 | `&&`
    pub connecter_conjunction: Content,
    /// 连接符/析取 | `||`
    pub connecter_disjunction: Content,
    /// 连接符/否定 | `--`
    pub connecter_negation: Content,
    /// 连接符/顺序合取 | `&/`
    pub connecter_conjunction_sequential: Content,
    /// 连接符/平行合取 | `&|`
    pub connecter_conjunction_parallel: Content,
}

/// Narsese格式/陈述
/// * 格式预期：`<词项 {系词} 词项>`
/// * 核心：存储各个陈述的**系词**
#[derive(Debug)]
pub struct NarseseFormatStatement<Content> {
    // 通用 //
    /// 首尾括弧 | `<` `>`
    pub brackets: (Content, Content),

    // 专用 //
    /// 系词/继承 | `-->`
    pub copula_inheritance: Content,
    /// 系词/相似 | `<->`
    pub copula_similarity: Content,
    /// 系词/蕴含 | `==>`
    pub copula_implication: Content,
    /// 系词/等价 | `<=>`
    pub copula_equivalence: Content,

    /// 派生系词/实例 | `{--`
    pub copula_instance: Content,
    /// 派生系词/属性 | `--]`
    pub copula_property: Content,
    /// 派生系词/实例属性 | `{-]`
    pub copula_instance_property: Content,

    /// 派生系词/预测性蕴含 | `=/>`
    pub copula_predictive_implication: Content,
    /// 派生系词/并发性蕴含 | `=|>`
    pub copula_concurrent_implication: Content,
    /// 派生系词/回顾性蕴含 | `=\>`
    pub copula_retrospective_implication: Content,

    /// 派生系词/预测性等价 | `</>`
    pub copula_predictive_equivalence: Content,
    /// 派生系词/并发性等价 | `<|>`
    pub copula_concurrent_equivalence: Content,
    /// 派生系词/回顾性等价 | `<\>`
    pub copula_retrospective_equivalence: Content,
}

/// Narsese格式/语句
/// * 格式预期：`词项{标点} {时间戳} {真值}`
#[derive(Debug)]
pub struct NarseseFormatSentence<Content> {
    /// 标点/判断 | `.`
    pub punctuation_judgement: Content,
    /// 标点/目标 | `!`
    pub punctuation_goal: Content,
    /// 标点/问题 | `?`
    pub punctuation_question: Content,
    /// 标点/请求 | `@`
    pub punctuation_quest: Content,

    /// 时间戳/括弧 | `:` `:`
    pub stamp_brackets: (Content, Content),
    /// 时间戳/过去 | `/`
    pub stamp_past: Content,
    /// 时间戳/现在 | `|`
    pub stamp_present: Content,
    /// 时间戳/未来 | `\`
    pub stamp_future: Content,
    /// 时间戳/指定时刻 | `!`
    pub stamp_fixed: Content,

    /// 真值/括弧 | `%` `%`
    pub truth_brackets: (Content, Content),
    /// 真值/分隔符 | `;`
    pub truth_separator: Content,
}

/// Narsese格式/任务
/// * 格式预期：`{预算值}语句`
#[derive(Debug)]
pub struct NarseseFormatTask<Content> {
    /// 预算值/括弧 | `$` `$`
    pub budget_brackets: (Content, Content),
    /// 预算值/分隔符 | `;`
    pub budget_separator: Content,
}

/// Narsese格式
/// * 📌记录「枚举Narsese」的各类常量
///   * ⚠️只用于存储数据，后续需要载入「解析器状态」
#[derive(Debug)]
pub struct NarseseFormat<Content> {
    /// 空白符（装饰用）
    pub space: Content,

    /// 原子词项的格式
    pub atom: NarseseFormatAtom<Content>,

    /// 复合词项的格式
    pub compound: NarseseFormatCompound<Content>,

    /// 陈述的格式
    pub statement: NarseseFormatStatement<Content>,

    /// 语句的格式
    pub sentence: NarseseFormatSentence<Content>,
    /// 任务的格式
    pub task: NarseseFormatTask<Content>,
}

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
                out.push_str(self.space);
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
        out.push_str(self.space);
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
        out.push_str(self.space);
        //各个元素
        for (i, term) in components.iter().enumerate() {
            // 插入占位符
            if i == index {
                out.push_str(self.atom.prefix_placeholder);
                out.push_str(self.compound.separator);
                out.push_str(self.space);
            }
            // 逗号
            if i != 0 {
                out.push_str(self.compound.separator);
                out.push_str(self.space);
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
        out.push_str(self.space);
        out.push_str(copula);
        out.push_str(self.space);
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
                self.compound.connector_intersection_intension,
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

    /// 工具函数/语句
    /// * 关键在「避免无用分隔符」
    fn add_space_if_necessary_and_flush_buffer(&self, out: &mut String, buffer: &mut String) {
        match buffer.is_empty() {
            // 空⇒不做动作
            true => {}
            // 非空⇒预置分隔符，推送并清空
            false => {
                out.push_str(self.space);
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
