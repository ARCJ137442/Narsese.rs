//! 定义「词法Narsese」的格式
//! * 📌WHY：原先「枚举Narsese」中的「Narsese格式」提前定义了所有「Narsese词项类型」的范围
//!   * 🎯希望能扩宽「原子词项」「复合词项」「陈述」的类型
//! * 🎯提供CommonNarsese中所有的词法信息
//!   * ⚠️仅作为「信息」使用，不提供任何 解析时/格式化时 优化
//! * 🚩目前对此处的「格式」不进行重命名处理
//!   * 📌理由：可以用「路径限定」「use * as」绕开「重名问题」
//! * 🚩此处不再开放「内容`Content`」类型
//!   * 📌「词法Narsese」数据结构中已固定类型为[`&'a str`]/&[`str`]
//!   * 因此整个「词法Narsese格式」已经和字符串绑定了

use util::{PrefixMatchDict, PrefixMatchDictPair};

/// Narsese格式/空白符
pub struct NarseseFormatSpace<'a> {
    /// 用于判断字符是否为空白符（解析用）
    /// * 📝Rust中若需定义静态常量，需要对常量确保线程安全
    ///   * 📄线程安全的类型⇔实现`Send + Sync`特征
    ///   * ⚠️`Box`类型无法作为常量初始化⇒退而求其次，变为「静态变量」⇒不可变`static`仍然要求常量表达式
    ///   * ⚠️任何闭包类型都不默认实现`Send + Sync`：直接`static`无法实现线程安全
    ///   * 🚩最终方案
    ///     * ✅常量表达式：使用[`lazy_statics`]实现「静态懒加载」绕开「`static`要求常量表达式」限制
    ///     * ✅线程安全：限制下边闭包为`dyn Fn(char) -> bool + Send + Sync`
    ///       * 📌其通常就是个纯函数
    pub is_for_parse: Box<dyn Fn(char) -> bool + Send + Sync>,

    /// 解析前是否筛除空白符
    /// 🎯用于决定在「解析环境理想化」时是否要「预筛除空白符」
    pub remove_spaces_before_parse: bool,

    /// 空白符（格式化/分隔词项）
    /// * 🎯复合词项/陈述
    ///   * 📄复合词项：`(&&, A, B, C)`
    ///   * 📄陈述：`<A --> B>`
    pub format_terms: &'a str,

    /// 空白符（格式化/分隔条目）
    /// * 🎯「预算 词项标点 时间戳 真值」
    pub format_items: &'a str,
}

/// 原子词项格式
/// * 📌格式：[前缀] + (标识符)
pub struct NarseseFormatAtom {
    /// 合法的「原子词项前缀」
    /// * 词语
    /// * 独立变量
    /// * 非独变量
    /// * 查询变量
    /// * 间隔
    /// * 操作符
    pub prefixes: PrefixMatchDict,

    /// 用于判断字符是否为「合法原子标识符」的函数
    pub is_identifier: Box<dyn Fn(char) -> bool + Send + Sync>,
}

/// 复合词项格式
#[derive(Debug, Clone)]
pub struct NarseseFormatCompound<'a> {
    /// 合法的「集合复合词项括弧对」
    /// * 外延集
    /// * 内涵集
    pub set_brackets: PrefixMatchDictPair<&'a str>,

    /// 通用的「复合词项括弧对」
    pub brackets: (&'a str, &'a str),

    /// 复合词项元素分隔符
    pub separator: &'a str,

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
pub struct NarseseFormatStatement<'a> {
    /// 通用的「陈述括弧对」
    pub brackets: (&'a str, &'a str),

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
pub struct NarseseFormatSentence<'a> {
    /// 合法的「标点」
    pub punctuations: PrefixMatchDict,

    /// 真值括弧
    /// * 🚩仅通过括弧捕获整个「真值」字符串，而**不再细分内部结构**
    pub truth_brackets: (&'a str, &'a str),

    /// 判断是否为「真值内部允许的字符」
    /// * 🎯用于提供信息以更快分割边界（从预算值而来）
    pub is_truth_content: Box<dyn Fn(char) -> bool + Send + Sync>,

    /// 时间戳括弧
    /// * 🚩仅通过括弧捕获整个「时间戳」字符串，而**不再细分内部结构**
    pub stamp_brackets: (&'a str, &'a str),
}

/// 任务格式（含预算值）
pub struct NarseseFormatTask<'a> {
    /// 预算值括弧
    /// * 🚩仅通过括弧捕获整个「预算值」字符串，而**不再细分内部结构**
    pub budget_brackets: (&'a str, &'a str),

    /// 判断是否为「预算值内部允许的字符」
    /// * 🎯用于解决可能的「预算值🆚独立变量」「误报的预算值范围」的问题
    /// * 📌在「总解析方法」中，以此为凭据分割「预算值」
    ///   * ❓似乎实际上的case并不存在：预算只会在开头进行匹配
    /// * 🚩若开头匹配了预算值左括弧，则
    ///   * 前缀匹配右括弧（提早结束）
    ///   * 收入前【通过此函数】**确认**将收入的字符是否合法
    /// * 📄case@ASCII: `$$$independent.`⇒空预算、词项为`$independent`、判断、永恒、空真值
    ///   * ✅解析过程：遇到第二个`$`视作闭括弧，提早结束
    /// * 📄case@ASCII: `$$independent.`⇒空预算、词项为`independent`、判断、永恒、空真值
    ///   * ✅解析过程：遇到第二个`$`视作闭括弧，提早结束
    /// * 📄case@ASCII: `$independent.`⇒空预算、词项为`$independent`、判断、永恒、空真值
    ///   * ✅解析过程：遇到非法内容`i`提前结束
    ///   * ❗无此函数的版本：没遇到闭括弧，提前结束
    /// * 📄case@漢文: `预算。`⇒解析错误：没有词项
    ///   * ✅解析过程：遇到右括弧`算`视作闭括弧，提早结束
    /// * 📄case@漢文: `预算预算。`⇒空预算、词项为`预算`、判断、永恒、空真值
    ///   * ✅解析过程：遇到右括弧`算`视作闭括弧，提早结束
    /// * 📄case@漢文: `预预算。`⇒空预算、词项为`预预算`、判断、永恒、空真值
    ///   * ✅解析过程：遇到非法内容`预`提前结束
    ///   * ⚠️无此函数的版本：截取到`预预算`，后边没词项⇒报错
    pub is_budget_content: Box<dyn Fn(char) -> bool + Send + Sync>,
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
pub struct NarseseFormat<'a> {
    /// 空白符格式
    pub space: NarseseFormatSpace<'a>,

    /// 原子词项格式
    pub atom: NarseseFormatAtom,

    /// 复合词项格式
    pub compound: NarseseFormatCompound<'a>,

    /// 陈述格式
    pub statement: NarseseFormatStatement<'a>,

    /// 语句格式（含标点、真值、时间戳）
    pub sentence: NarseseFormatSentence<'a>,

    /// 任务格式（含预算值）
    pub task: NarseseFormatTask<'a>,
    // ! 相比「枚举Narsese」不再有「关键词截断选项」
    // ! 🚩【2024-03-15 17:48:03】目前`enable_keyword_truncation`强制为`true`
}
