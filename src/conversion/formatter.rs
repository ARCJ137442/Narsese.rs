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

use crate::term::*;

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
    /// 时间戳/预测性 | `/`
    pub stamp_predictive: Content,
    /// 时间戳/并发性 | `|`
    pub stamp_concurrent: Content,
    /// 时间戳/回顾性 | `\`
    pub stamp_retrospective: Content,
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
    pub truth_brackets: (Content, Content),
    /// 预算值/分隔符 | `;`
    pub truth_separator: Content,
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
impl NarseseFormat<&str> {
    /// 工具函数/原子词项
    fn format_atom(&self, atom: &Term, prefix: &str) -> String {
        format!("{}{}", prefix, atom.get_atom_name_unchecked(),)
    }

    /// 工具函数/词项集
    fn format_set(
        &self,
        components: Vec<&Term>,
        bracket_left: &str,
        bracket_right: &str,
        separator: &str,
    ) -> String {
        // 格式化用分隔符
        let separator_format = &format!("{}{}", separator, self.space);
        format!(
            "{}{}{}",
            bracket_left,
            components
                .iter()
                .map(|term| self.format_term(term))
                .collect::<Vec<String>>()
                .join(separator_format),
            bracket_right
        )
    }

    /// 工具函数/复合词项
    fn format_compound(&self, components: Vec<&Term>, connecter: &str) -> String {
        // 格式化用分隔符
        let separator_format = &format!("{}{}", self.compound.separator, self.space);
        format!(
            "{}{}{}{}{}",
            self.compound.brackets.0,
            connecter,
            separator_format,
            &components
                .iter()
                .map(|term| self.format_term(term))
                .collect::<Vec<String>>()
                .join(separator_format),
            self.compound.brackets.1,
        )
    }

    /// 工具函数/像
    fn format_image(&self, index: usize, components: Vec<&Term>, connecter: &str) -> String {
        let mut s = String::new();
        // 左括号
        s.push_str(self.compound.brackets.0);
        // 连接符
        s.push_str(connecter);
        s.push_str(self.compound.separator);
        s.push_str(self.space);
        //各个元素
        for (i, term) in components.iter().enumerate() {
            // 插入占位符
            if i == index {
                s.push_str(self.atom.prefix_placeholder);
                s.push_str(self.compound.separator);
                s.push_str(self.space);
            }
            // 逗号
            if i > 0 {
                s.push_str(self.compound.separator);
                s.push_str(self.space);
            }
            // 词项
            s.push_str(&self.format_term(term));
        }
        // 右括号
        s.push_str(self.compound.brackets.1);
        // 返回
        s
    }

    /// 工具函数/陈述
    fn format_statement(&self, left: &Term, right: &Term, copula: &str) -> String {
        format!(
            "{}{}{}{}{}{}{}",
            self.statement.brackets.0,
            self.format_term(left),
            self.space,
            copula,
            self.space,
            self.format_term(right),
            self.statement.brackets.1
        )
    }

    /// 总格式化函数
    pub fn format_term(&self, term: &Term) -> String {
        match term {
            // 原子词项
            Word(..) => self.format_atom(term, self.atom.prefix_word),
            VariableIndependent(..) => {
                self.format_atom(term, self.atom.prefix_variable_independent)
            }
            VariableDependent(..) => self.format_atom(term, self.atom.prefix_variable_dependent),
            VariableQuery(..) => self.format_atom(term, self.atom.prefix_variable_query),
            Interval(..) => self.format_atom(term, self.atom.prefix_interval),
            Operator(..) => self.format_atom(term, self.atom.prefix_operator),
            // 复合词项
            SetExtension(..) => self.format_set(
                term.get_components(),
                self.compound.brackets_set_extension.0,
                self.compound.brackets_set_extension.1,
                self.compound.separator,
            ),
            SetIntension(..) => self.format_set(
                term.get_components(),
                self.compound.brackets_set_intension.0,
                self.compound.brackets_set_intension.1,
                self.compound.separator,
            ),
            IntersectionExtension(..) => self.format_compound(
                term.get_components(),
                self.compound.connecter_intersection_extension,
            ),
            IntersectionIntension(..) => self.format_compound(
                term.get_components(),
                self.compound.connector_intersection_intension,
            ),
            DifferenceExtension(..) => self.format_compound(
                term.get_components(),
                self.compound.connecter_difference_extension,
            ),
            DifferenceIntension(..) => self.format_compound(
                term.get_components(),
                self.compound.connecter_difference_intension,
            ),
            Product(..) => {
                self.format_compound(term.get_components(), self.compound.connecter_product)
            }
            ImageExtension(index, _) => self.format_image(
                *index,
                term.get_components(),
                self.compound.connecter_image_extension,
            ),
            ImageIntension(index, _) => self.format_image(
                *index,
                term.get_components(),
                self.compound.connecter_image_intension,
            ),
            Conjunction(..) => {
                self.format_compound(term.get_components(), self.compound.connecter_conjunction)
            }
            Disjunction(..) => {
                self.format_compound(term.get_components(), self.compound.connecter_disjunction)
            }
            Negation(..) => {
                self.format_compound(term.get_components(), self.compound.connecter_negation)
            }
            ConjunctionSequential(..) => self.format_compound(
                term.get_components(),
                self.compound.connecter_conjunction_sequential,
            ),
            ConjunctionParallel(..) => self.format_compound(
                term.get_components(),
                self.compound.connecter_conjunction_parallel,
            ),
            // 陈述
            Inheritance(left, right) => {
                self.format_statement(left, right, self.statement.copula_inheritance)
            }
            Similarity(left, right) => {
                self.format_statement(left, right, self.statement.copula_similarity)
            }
            Implication(left, right) => {
                self.format_statement(left, right, self.statement.copula_implication)
            }
            Equivalence(left, right) => {
                self.format_statement(left, right, self.statement.copula_equivalence)
            }
        }
    }
}
