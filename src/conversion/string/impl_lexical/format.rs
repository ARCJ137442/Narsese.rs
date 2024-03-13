//! 定义「词法Narsese」的格式
//! * 📌WHY：原先「枚举Narsese」中的「Narsese格式」提前定义了所有「Narsese词项类型」的范围
//!   * 🎯希望能扩宽「原子词项」「复合词项」「陈述」的类型
//! * 🎯提供CommonNarsese中所有的词法信息
//!   * ⚠️仅作为「信息」使用，不提供任何 解析时/格式化时 优化
//! * 🚩目前对此处的「格式」不进行重命名处理
//!   * 📌理由：可以用「路径限定」「use * as」绕开「重名问题」
//! * 🚩此处不再开放「内容`Content`」类型
//!   * 📌「词法Narsese」数据结构中已固定类型为[`String`]/&[`str`]
//!   * 因此整个「词法Narsese格式」已经和字符串绑定了

// TODO: 定义「前缀匹配字典」，🎯解决「短的先匹配到截断了，长的因此无法被匹配到」的问题
pub type PrefixMatchDict = Vec<String>;
pub type PrefixMatchDictPairs = Vec<(String, String)>;

/// Narsese格式/空白符
#[derive(Debug, Clone)]
pub struct NarseseFormatSpace {
    /// 空白符（解析用）
    pub parse: String,
    /// 空白符（格式化/分隔词项）
    /// * 🎯复合词项/陈述
    pub format_terms: String,
    /// 空白符（格式化/分隔条目）
    /// * 🎯「预算 词项标点 时间戳 真值」
    pub format_items: String,
}

/// 原子词项格式
/// * 📌格式：[前缀] + (标识符)
#[derive(Debug, Clone)]
pub struct NarseseFormatAtom {
    /// 合法的「原子词项前缀」
    /// * 词语
    /// * 独立变量
    /// * 非独变量
    /// * 查询变量
    /// * 间隔
    /// * 操作符
    pub prefixes: PrefixMatchDict,
}

/// 复合词项格式
#[derive(Debug, Clone)]
pub struct NarseseFormatCompound {
    /// 合法的「集合复合词项括弧对」
    /// * 外延集
    /// * 内涵集
    pub set_brackets: PrefixMatchDictPairs,

    /// 通用的「复合词项括弧对」
    pub brackets: (String, String),

    /// 复合词项元素分隔符
    pub separator: String,

    /// 合法的「复合词项连接符」
    /// * 外延交/内涵交
    /// * 外延差/内涵差
    /// * 乘积
    /// * 外延像/内涵像
    /// * 合取/析取
    /// * 否定
    /// * 顺序合取/平行合取
    pub connecters: PrefixMatchDict,
}

/// 陈述格式
#[derive(Debug, Clone)]
pub struct NarseseFormatStatement {
    /// 通用的「陈述括弧对」
    pub brackets: (String, String),

    /// 合法的「中缀系词」
    /// * 继承
    /// * 相似
    /// * 蕴含
    /// * 等价
    /// * 实例/属性/实例属性
    /// * 预测性/并发性/回顾性 蕴含
    /// * 预测性/并发性/回顾性 等价
    pub copulas: PrefixMatchDict,
}

/// 语句格式（含标点、真值、时间戳）
#[derive(Debug, Clone)]
pub struct NarseseFormatSentence {
    /// 合法的「标点」
    pub punctuations: PrefixMatchDict,

    /// 真值括弧
    /// * 🚩仅通过括弧捕获整个「真值」字符串，而**不再细分内部结构**
    pub truth_brackets: (String, String),

    /// 时间戳括弧
    /// * 🚩仅通过括弧捕获整个「时间戳」字符串，而**不再细分内部结构**
    pub stamp_brackets: (String, String),
}

/// 任务格式（含预算值）
#[derive(Debug, Clone)]
pub struct NarseseFormatTask {
    /// 预算值括弧
    /// * 🚩仅通过括弧捕获整个「预算值」字符串，而**不再细分内部结构**
    pub truth_brackets: (String, String),
}

/// 总「词法Narsese格式」
/// * ⚙️包括：
///   * 原子词项格式
///   * 复合词项格式
///   * 陈述格式
///   * 语句格式（含标点、真值、时间戳）
///   * 任务格式（含预算值）
/// * 🚩不特化符号为`LexicalNarseseFormat`
///   * 📌这种「符号特化」交给调用方处理
#[derive(Debug, Clone)]
pub struct NarseseFormat {
    /// 空白符格式
    pub space: NarseseFormatSpace,

    /// 原子词项格式
    pub atom: NarseseFormatAtom,

    /// 复合词项格式
    pub compound: NarseseFormatCompound,

    /// 陈述格式
    pub statement: NarseseFormatStatement,

    /// 语句格式（含标点、真值、时间戳）
    pub sentence: NarseseFormatSentence,

    /// 任务格式（含预算值）
    pub task: NarseseFormatTask,
}
