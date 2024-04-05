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
//!
//! ! 【2024-03-18 20:33:31】当下统一使用「动态字串」[`String`]
//!   * ❌弃用`&str`的理由
//!     * 生命周期管理冗杂 | 💭允许牺牲一定性能，专注功能
//!     * 前缀匹配字典不兼容 | 无法合并「动态字串前缀匹配」与「静态字串前缀匹配」

use util::{
    BiFixMatchDict, BiFixMatchDictPair, PrefixMatchDict, SuffixMatchDict, SuffixMatchDictPair,
};

/// Narsese格式/空白符
pub struct NarseseFormatSpace<F = Box<dyn Fn(char) -> bool + Send + Sync>>
where
    F: Fn(char) -> bool + Send + Sync,
{
    /// 用于判断字符是否为空白符（解析用）
    /// * 📝Rust中若需定义静态常量，需要对常量确保线程安全
    ///   * 📄线程安全的类型⇔实现`Send + Sync`特征
    ///   * ⚠️`Box`类型无法作为常量初始化⇒退而求其次，变为「静态变量」⇒不可变`static`仍然要求常量表达式
    ///   * ⚠️任何闭包类型都不默认实现`Send + Sync`：直接`static`无法实现线程安全
    ///   * 🚩最终方案
    ///     * ✅常量表达式：使用[`lazy_statics`]实现「静态懒加载」绕开「`static`要求常量表达式」限制
    ///     * ✅线程安全：限制下边闭包为`dyn Fn(char) -> bool + Send + Sync`
    ///       * 📌其通常就是个纯函数
    pub is_for_parse: F,

    /// 解析前是否筛除空白符
    /// 🎯用于决定在「解析环境理想化」时是否要「预筛除空白符」
    pub remove_spaces_before_parse: bool,

    /// 空白符（格式化/分隔词项）
    /// * 🎯复合词项/陈述
    ///   * 📄复合词项：`(&&, A, B, C)`
    ///   * 📄陈述：`<A --> B>`
    pub format_terms: String,

    /// 空白符（格式化/分隔条目）
    /// * 🎯「预算 词项标点 时间戳 真值」
    pub format_items: String,
}

/// 原子词项格式
/// * 📌格式：[前缀] + (标识符)
pub struct NarseseFormatAtom<F = Box<dyn Fn(char) -> bool + Send + Sync>>
where
    F: Fn(char) -> bool + Send + Sync,
{
    /// 合法的「原子词项前缀」
    /// * 词语
    /// * 独立变量
    /// * 非独变量
    /// * 查询变量
    /// * 间隔
    /// * 操作符
    pub prefixes: PrefixMatchDict,

    /// 用于判断字符是否为「合法原子标识符」的函数
    pub is_identifier: F,
}

/// 复合词项格式
#[derive(Debug, Clone)]
pub struct NarseseFormatCompound {
    /// 合法的「集合复合词项括弧对」
    /// * 外延集
    /// * 内涵集
    pub set_brackets: BiFixMatchDictPair,

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
    pub copulas: BiFixMatchDict,
}

/// 语句格式（含标点、真值、时间戳）
pub struct NarseseFormatSentence<F = Box<dyn Fn(char) -> bool + Send + Sync>>
where
    F: Fn(char) -> bool + Send + Sync,
{
    /// 合法的「标点」
    pub punctuations: SuffixMatchDict,

    /// 真值括弧
    /// * 🚩通过括弧捕获整个「真值」字符串，然后拆分其内部结构
    pub truth_brackets: (String, String),

    /// 真值内部分隔符
    /// * 🎯用于进一步**细分**内部的值
    ///   * 📌【2024-03-22 20:02:15】初次需求见NAVM的「不同格式词法Narsese互转」的情形
    /// * 📍原则：不能包括任何「具体格式固定」的内容
    ///   * 📄如「漢文」格式中存储"$1.0; 0.9$"作为「合法」情况
    /// * 🚩【2024-03-22 20:04:09】目前的实现方法：作为「数值字串数组」的形式
    ///   * 📄形如`vec!["1.0", "0.9"]`目前是最佳实践
    pub truth_separator: String,

    /// 判断是否为「真值内部允许的字符」
    /// * 🎯用于提供信息以更快分割边界（从预算值而来）
    pub is_truth_content: F,

    /// 合法的时间戳「括弧」对
    /// * 🎯适配LaTeX/漢文的「无固定括弧」情况
    /// * 📝对于「时间戳」不能再再像ASCII版本那样假设「一定有固定括弧」了
    ///   * 📄ASCIIの「过去」：`:\:` => `("", ":\:")`
    ///   * 📄LaTeXの「过去」：`\backslash\!\!\!\!\!\Rightarrow` => `("", "\backslash\!\!\!\!\!\Rightarrow")`
    ///   * 📌此处可统一使用「空前缀」兼容「枚举时间戳」：实际使用时进行后缀匹配，空前缀可实现与「特别开一个`enum_stamps`」基本一样的性能
    /// * 🎯一并处理有关「固定时间戳」的问题：内部合法字符判定
    ///   * 📄ASCIIの「固定」：`:!137:` => `(":!", ":")`
    ///   * 📄LaTeXの「固定」：`t=[+-][0-9]+` => `("t=", "")`
    /// * 🚩在「枚举时间戳」之后，以更细分的方式【正确】捕获「固定时间戳」类型
    ///   * 实际上是一种「括弧匹配」
    /// * ✨直接通过「不同类括弧」兼容各类「固定时间戳」类型
    ///   * 📌而无需固定「时间戳括弧」
    pub stamp_brackets: SuffixMatchDictPair<String>,

    /// 合法的「固定时间戳」
    /// * 🎯适配LaTeX/漢文的「无固定括弧」情况
    /// * 📌通过「合法字符序列」兼容「前后缀不固定的『固定』时间戳类型」
    ///   * 📄ASCIIの「固定」：`:!-123:`
    pub is_stamp_content: F,
}

/// 任务格式（含预算值）
pub struct NarseseFormatTask<F = Box<dyn Fn(char) -> bool + Send + Sync>>
where
    F: Fn(char) -> bool + Send + Sync,
{
    /// 预算值括弧
    /// * 🚩通过括弧捕获整个「预算值」字符串，然后拆分其内部结构
    pub budget_brackets: (String, String),

    /// 预算值内部分隔符
    /// * 🎯用于进一步**细分**内部的值
    /// * 📄来由、用法等参考[`NarseseFormatSentence::truth_separator`]
    pub budget_separator: String,

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
    pub is_budget_content: F,
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
/// * 🚩现在将其中的「函数类型」提取为类型参数
///   * 📜默认还是`Box<dyn Fn>`
///   * ✅可兼容其它实现了`Fn`特征的对象（如函数指针）
pub struct NarseseFormat<F = Box<dyn Fn(char) -> bool + Send + Sync>>
where
    F: Fn(char) -> bool + Send + Sync,
{
    /// 空白符格式
    pub space: NarseseFormatSpace<F>,

    /// 原子词项格式
    pub atom: NarseseFormatAtom<F>,

    /// 复合词项格式
    pub compound: NarseseFormatCompound,

    /// 陈述格式
    pub statement: NarseseFormatStatement,

    /// 语句格式（含标点、真值、时间戳）
    pub sentence: NarseseFormatSentence<F>,

    /// 任务格式（含预算值）
    pub task: NarseseFormatTask,
    // ! 相比「枚举Narsese」不再有「关键词截断选项」
    // ! 🚩【2024-03-15 17:48:03】目前`enable_keyword_truncation`强制为`true`
}
