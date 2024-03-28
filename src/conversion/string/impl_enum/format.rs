//! 记录Narsese的格式（数据结构）
//! * 🎯提供CommonNarsese中所有的语法信息
//!   * ⚠️仅作为「信息」使用，不提供任何 解析时/格式化时 优化
//! * 📄部分定义参照自[JuNarsese](https://github.com/ARCJ137442/JuNarsese.jl)
//! * 🚩目前对此处的「格式」不进行重命名处理
//!   * 📌理由：可以用「路径限定」「use * as」绕开「重名问题」
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

/// Narsese格式/原子词项
/// * 格式预期：`{前缀}+词项字符串名`
///   * 📌将「占位符」也包含在内——相当于「只有前缀，没有内容」的词项
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
    /// 前缀/占位符 | `_`
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
    pub connecter_intersection_intension: Content,
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
    pub copula_implication_predictive: Content,
    /// 派生系词/并发性蕴含 | `=|>`
    pub copula_implication_concurrent: Content,
    /// 派生系词/回顾性蕴含 | `=\>`
    pub copula_implication_retrospective: Content,

    /// 派生系词/预测性等价 | `</>`
    pub copula_equivalence_predictive: Content,
    /// 派生系词/并发性等价 | `<|>`
    pub copula_equivalence_concurrent: Content,
    /// 派生系词/回顾性等价 | `<\>`
    pub copula_equivalence_retrospective: Content,
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

/// Narsese格式/空白符
#[derive(Debug)]
pub struct NarseseFormatSpace<Content> {
    /// 空白符（解析用）
    pub parse: Content,
    /// 空白符（格式化/分隔词项）
    /// * 🎯复合词项/陈述
    pub format_terms: Content,
    /// 空白符（格式化/分隔条目）
    /// * 🎯「预算 词项标点 时间戳 真值」
    pub format_items: Content,
}

/// Narsese格式
/// * 📌记录「枚举Narsese」的各类常量
///   * ⚠️只用于存储数据，后续需要载入「解析器状态」
#[derive(Debug)]
pub struct NarseseFormat<Content> {
    /// 空白符
    pub space: NarseseFormatSpace<Content>,

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
    // * 🚩【2024-03-28 14:33:47】现弃用「关键字截断」机制，直接使用「系词前缀匹配」判断
    // pub enable_keyword_truncation: bool,
}

impl NarseseFormat<&str> {
    /// 创建「系词」数组
    /// * 🎯在兼容`^go-to`的同时，解决「`外延--` `>` `内涵`」的兼容问题
    /// * 🚩保留完整的系词字串
    /// * ⚠️纯功能性：不判断「是否启用」
    /// * 🚩【2024-03-28 14:33:09】替代「保留关键字」，牺牲部分性能，换得对「作为原子词项内容的`-`」的兼容性
    pub fn copulas(&self) -> [&str; 13] {
        // 创建&填充数组
        [
            // * （主要）陈述系词
            self.statement.copula_inheritance,
            self.statement.copula_similarity,
            self.statement.copula_implication,
            self.statement.copula_equivalence,
            self.statement.copula_instance,
            self.statement.copula_property,
            self.statement.copula_instance_property,
            self.statement.copula_implication_predictive,
            self.statement.copula_implication_concurrent,
            self.statement.copula_implication_retrospective,
            self.statement.copula_equivalence_predictive,
            self.statement.copula_equivalence_concurrent,
            self.statement.copula_equivalence_retrospective,
        ]
    }
}
