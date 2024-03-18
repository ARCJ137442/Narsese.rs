//! 实现/词法解析器
//! * 🎯字符串→词法Narsese
//!
//! 🆕【2024-03-16 20:50:39】新的解析方案
//! * 🏷️核心：拆解⇒分派⇒组装
//!   * ✨拆解：对每个「子解析器」，都会按自身结构将环境分块成系列「子环境」
//!   * ✨分派：「子解析器」将环境分块后，把这些分块的「子环境」分派给其它「子解析器」解析
//!   * ✨组装：在「被分派的解析器」全部解析完毕后，「子解析器」将解析结果进行组装
//! * 📌有界字符环境：不依赖所谓「字符迭代器」，直接使用**字符数组**
//!   * 📍确定「解析环境」为「字符数组切片」`&[char]`
//!   * 📍解析环境总是「长度有限、双端已知」的：总是可以进行前后匹配
//!   * 🎯解决先前「字符缓冲区迭代器方案」的「缺乏条件，处处受限」的问题
//! * 📌组合式解析器：解析器间不共享除「解析环境」外的状态
//!   * 📍解析器之间即便会相互调用，也不会共享除「解析环境」外的任何状态
//!   * 🎯解决先前「头索引递进方案」的「总需关注头索引位置，生怕越界还要回溯」的麻烦
//! * 📌充足环境假设：对所有五种条目均做足「预设环境」假设
//!   * 📄五种条目类型：「预算值」「词项」「标点」「时间戳」「真值」
//!   * 📍预设环境：总能从理想的「解析环境」中开始解析
//!     * 如：`parse_statement`总是能以`"<A --> B>"`的原子化形式传入
//!     * 如：`parse_compound`总是能以`"(*, A, B)"`的原子化形式传入
//!   * 🎯利用这些条件，牺牲一定时间复杂度，拯救更多空间复杂度
//!     * 至少`O(n)`不可避免：总是需要扫描整个「解析环境」
//!   * ⚠️因此，其中的「子解析器」可能仍需「理想化」才能转为公开接口
//!     * 如：` <(*, A, B) --> ^op >` ⇒ `<(*,A,B)-->^op>`
//!     * 🎯由此可以引入「预筛除空白符」机制，简化先前「处处判断空白符」的问题
//!
//! ❓在「解析复合词项」「解析陈述」这类【词项无法简单通过「前后搜寻」分割出来】的情况
//!   * 💡预先交给一个基于「嵌套括号匹配」的「界定函数」
//!   * ❗但要避免「系词里含有『括号』」的干扰情况
//!     * 📄源自CommonNarsese case `<A-->B>`中的`-->`
//!     * 📌目前假设「只有『陈述系词』才需要特别对待」：连接词可以使用「前缀匹配」随着左括弧一起排除
//!       * 📄如：漢文版本`（外像，我，某，是，似）`中的两个「系词」（「是」「似」）在「复合词项上下文」中不会被考虑为「复合词项连接词」
//!       * 💭只要别把括号改得「过于变态」，就可以通过
//!     * ❌这基本否决了通过「括号树」进行匹配的方案——不然就要时刻提防「系词/连接符冒充括号」的情况

use util::{first, PrefixMatch, SuffixMatch};

use super::NarseseFormat;
use crate::{
    lexical::{Narsese, Sentence, Task, Term},
    util::{BufferIterator, IntoChars},
};
use std::{error::Error, fmt::Display, result};

/// 词法解析 辅助结构对象
/// * 🚩放在一个独立的模块内，以便折叠
pub mod structs {

    use super::*;

    /// 定义「解析环境」：字符数组切片
    pub type ParseEnv<'a> = &'a [char];

    /// 定义具备所有权的「解析环境」：字符数组
    pub type ParseEnvOwned = Vec<char>;

    /// 定义「解析索引」
    /// * 🎯用于区分「长度」与「位置」：与直接使用的`usize`区分开
    pub type ParseIndex = usize;

    /// 定义「解析结果」
    /// * 🚩实际就是「错误类型已指定的[`Result`]」
    /// * 返回的「结果」默认为[`Narsese`]（词项/语句/任务）
    pub type ParseResult<T = Narsese> = Result<T, ParseError>;

    /// 定义「中间结果」
    /// * 🎯用于表征「可有可无」的各种Narsese条目
    ///   * 🏷️预算、词项、标点、时间戳、真值
    /// * 📌其内字段均具有所有权
    ///   * ✅均可以被直接拿取，并解析为Narsese值
    #[derive(Debug, Clone)]
    pub struct MidParseResult {
        /// 预算值
        pub budget: Option<String>,
        /// 词项
        pub term: Option<Term>,
        /// 标点
        pub punctuation: Option<String>,
        /// 时间戳
        pub stamp: Option<String>,
        /// 真值
        pub truth: Option<String>,
    }

    impl MidParseResult {
        /// 从「中间解析结果」到「Narsese值」
        /// * 🎯实现最终的「词项/语句/任务」限制
        /// * ⚠️会直接递交所有权：需要取出其中的值
        /// * 🚩暂且最纯粹地实现为[`Option`]，[`Err`]生成交给调用者
        pub fn fold(self) -> Option<Narsese> {
            match self {
                // 任务：词项+标点+预算值
                MidParseResult {
                    term: Some(term),
                    punctuation: Some(punctuation),
                    budget: Some(budget),
                    stamp,
                    truth,
                    ..
                } => Some(Narsese::Task(Task {
                    budget,
                    sentence: Sentence {
                        term,
                        punctuation,
                        stamp: stamp.unwrap_or("".into()),
                        truth: truth.unwrap_or("".into()),
                    },
                })),
                // 语句：词项+标点
                MidParseResult {
                    term: Some(term),
                    punctuation: Some(punctuation),
                    stamp,
                    truth,
                    ..
                } => Some(Narsese::Sentence(Sentence {
                    term,
                    punctuation,
                    stamp: stamp.unwrap_or("".into()),
                    truth: truth.unwrap_or("".into()),
                })),
                // 词项
                MidParseResult {
                    term: Some(term), ..
                } => Some(Narsese::Term(term)),
                // 缺省情况
                _ => None,
            }
        }
    }

    /// 用于表征「解析错误」
    /// * 📝不要依赖于任何外部引用：后续需要【脱离】解析环境
    /// * 🚩【2024-03-16 21:24:22】自「枚举Narsese」迁移而来
    ///   * 因「解析环境」（字符数组（切片））的共通性，此处可以无缝迁移
    #[derive(Debug, Clone)]
    pub struct ParseError {
        /// 错误消息 | 一般不含冒号
        /// * 🎯用于描述出错原因
        message: String,
        /// 裁剪出的「解析环境」切片（具有所有权）
        /// * 🎯用于展示出错范围
        /// * 🚩【2024-03-17 01:59:26】现在直接一步到位变成字符串
        env_scope: String,
        // /// 出错所在的「解析索引」
        // /// * 🎯用于指示出错位置
        // ! ⚠️【2024-03-17 01:55:44】现在不再需要「解析索引」
        //   * 📌解析的方法本身已经和「头索引」无关
        //   * 【无法也没必要】给错误定位
        // index: ParseIndex,
    }
    impl ParseError {
        /// 工具函数/生成「环境切片」
        /// * 🚩【2024-03-17 01:58:27】现在因为「与『头索引』概念解绑」无需再选取范围
        fn generate_env_scope(env: ParseEnv) -> String {
            // 直接获取所有权即可
            String::from_iter(env.iter())
        }

        /// 构造函数
        /// * ⚠️【2024-03-17 01:57:33】现在不再需要
        pub fn new(message: &str, env: ParseEnv) -> ParseError {
            ParseError {
                message: message.into(),
                env_scope: ParseError::generate_env_scope(env),
            }
        }
    }
    /// 呈现报错文本
    impl Display for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            // 输出
            write!(
                f,
                "Narsese解析错误：{} in {:?}",
                self.message, self.env_scope
            )
        }
    }
    impl Error for ParseError {}

    /// 词法解析状态
    /// * 🚩只持有引用，相当于一个局部变量上下文
    /// * 📌这种结构一定是**轻量级**的
    ///   * 🚩后续预计会大量递归调用（至少会出现在「递归解析词项」中）
    #[derive(Clone)]
    pub struct ParseState<'a> {
        /// 词法格式
        /// * 📌用于指定解析所用的关键字
        pub format: &'a NarseseFormat<'a>,
        // /// 解析环境：字符数组切片
        // /// * 📌基本是唯一共享的状态
        // pub env: ParseEnv<'a>,
        // ! 🚩【2024-03-17 01:29:17】现在不再内置于「解析状态」中
        // * 📌原因：实际推断中发现「解析状态」的「解析环境」经常会变
        //   * 💭更适合作为函数参数传递，而非
    }
    /// 通用实现 / 非「词法解析」的方法
    impl<'a> ParseState<'a> {
        /// 构造函数
        /// * ⚠️【2024-03-17 01:30:50】不再与「解析环境」绑定
        ///   * 📌后者改为「在方法中动态传入」
        pub fn new(format: &'a NarseseFormat) -> Self {
            Self { format }
        }

        /// 快速构造`Err`
        pub fn err<T>(&self, env: ParseEnv<'a>, message: &str) -> ParseResult<T> {
            Err(ParseError::new(message, env))
        }
    }
}
use structs::*;

// 词法解析 正式逻辑开始 //

/// 用于把「自由函数」封装成「实例方法」
pub trait RightUnwrapOr<T, U> {
    /// 工具函数
    /// * 🎯用于可选元组「(解析结果，索引)」的部分默认值化
    ///   * 在「真值」「预算值」等「可选条目」中，「没有值」与「值为空字串」是不一样的
    /// * 🚩`Option<(T, U)>`⇒`(Option<T>, U)` | U取默认值
    fn right_unwrap_or(self, default_u: U) -> (Option<T>, U);
}
impl<T, U> RightUnwrapOr<T, U> for Option<(T, U)> {
    fn right_unwrap_or(self, default_u: U) -> (Option<T>, U) {
        match self {
            // 若有⇒部分取值
            Some((t, u)) => (Some(t), u),
            // 若无⇒部分设置默认值
            None => (None, default_u),
        }
    }
}

/// 总入口
/// * 🚩构造「解析状态」然后转发到「解析状态的实例方法」中去
pub fn parse(format: &NarseseFormat, input: &str) -> ParseResult {
    // 「理想化」构造解析状态
    // ! 📌此处「理想化」必须在构造之前，否则很难修改
    let chars = idealize_env(format, input);
    let mut state = ParseState::new(format);
    // 用状态进行解析
    state.parse(&chars)
    // ! 随后丢弃状态
}

/// 预处理/理想化
/// * 📌将一个「字符串」进行「理想化」以便后续解析
/// * 🎯用于「预处理删去空格」这一类情况
///   * ❗每个`&str`字符串在被解析之前，都要经过此处解析
pub fn idealize_env(format: &NarseseFormat, input: &str) -> ParseEnvOwned {
    // 获取字符迭代器
    let chars = input.chars();
    // 对「字符迭代器」进行处理 | 不能提取`.collect::<ParseEnvOwned>()`，因为其所应用的类型不一致
    match format.space.remove_spaces_before_parse {
        // 预删去空格
        true => chars
            .filter(|&c| !(format.space.is_for_parse)(c))
            .collect::<ParseEnvOwned>(),
        // 不删去空格
        false => chars.collect::<ParseEnvOwned>(),
    }
}

/// 开始在「解析状态」的基础上进行解析
impl<'a> ParseState<'a> {
    /// 主解析入口
    /// * 📌【2024-03-17 01:34:10】现在总是从外部传入「解析环境」
    /// * 🚩先解析出各个条目组成「中间结果」，再进行拼接
    ///   * 其中「中间结果」不作为自身字段
    pub fn parse(&mut self, env: ParseEnv<'a>) -> ParseResult {
        // 先解析出「中间结果」
        let mid_result = self.parse_items(env)?;
        // 再折叠「中间结果」得到最终情况
        match mid_result.fold() {
            // 解析出了结果⇒返回最终结果
            Some(result) => Ok(result),
            // 没有解析出结果⇒返回错误
            None => self.err(env, "缺省条目，无法解析成词项/语句/任务"),
        }
    }

    /// 主解析过程
    /// * 🎯返回相比「Narsese值」[`Narsese`]更**灵活**的「中间结果」
    /// * 🚩前缀截取预算，后缀截取真值、时间戳、标点⇒最后就只剩下词项
    ///   * 📌重点在「递归解析词项」获得「词法结构」
    /// * 📄从「中间结果」到「Narsese值」参见
    /// * ⚠️注意：「没解析到」和「解析时出错」是不一样的
    ///   * 比如「没解析到预算值」也可以是如`$A.`的情况
    pub fn parse_items(&mut self, env: ParseEnv<'a>) -> ParseResult<MidParseResult> {
        // 前缀切割出预算值 //
        let budget = self.segment_budget(env);
        // 默认值 "" | 词项的起始索引（含）
        let (budget, begin_index) = budget.right_unwrap_or(0);

        // 后缀连续切割出真值、时间戳、标点 //
        let truth = self.segment_truth(env);
        // 默认值 "" | 时间戳的索引上界（不含）
        let (truth, end_index) = truth.right_unwrap_or(env.len());
        // 时间戳
        let stamp = self.segment_stamp(env);
        // 默认值 "" | 标点的索引上界（不含）
        let (stamp, end_index) = stamp.right_unwrap_or(end_index);
        // 标点
        let punctuation = self.segment_punctuation(env);
        // 默认值 "" | 词项的索引上界（不含）
        let (punctuation, end_index) = punctuation.right_unwrap_or(end_index);

        // 前后缀切割完毕，最后解析出词项 //
        // 获得「词项」的「字符数组切片」
        let term = match begin_index < end_index {
            true => self.parse_term(&env[begin_index..end_index])?,
            false => {
                return self.err(
                    env,
                    &format!("无法在索引[{begin_index}..{end_index}]解析出词项"),
                )
            }
        };

        // 构造「中间结果」 //
        Ok(MidParseResult {
            term,
            truth,
            stamp,
            punctuation,
            budget,
        })
    }

    /// 🛠️工具函数/在环境中从某处索引截取字符序列
    /// * 持续【从左到右】匹配，直到右边界/非法字符/环境边界为止
    ///   * 右边界⇒`Ok(右边界起始索引)`
    ///   * 非法字符⇒`Ok(非法字符所在索引)`
    ///   * 环境边界⇒`Ok(环境长度即索引右边界)`
    /// * 🎯对应PEG中的Any/Some逻辑
    /// * 🚩【2024-03-18 08:47:12】现在基本确立「延迟截取字符串」原则
    ///   * 不到需要的时候，一律以「起止索引」表示「字符串」
    ///   * 后续一律从[`String::from_iter`]转换
    /// * 📌「在指定位置开始」的情形，完全可以通过「预先对环境切片」解决
    ///   * 📄例如：`("abc", start = 1)` ⇒ `(&"abc"[1..])`
    #[inline(always)]
    fn segment_some_prefix(
        &self,
        env: ParseEnv<'a>,
        start: ParseIndex,
        right_chars: ParseEnv,
        verify_char: impl Fn(char) -> bool,
    ) -> Result<ParseIndex, ParseIndex> {
        // 自动计算长度
        let right_len_chars = right_chars.len();
        // 然后从起始索引处开始
        let mut i = start;
        while i < env.len() {
            // 右括弧⇒预先返回
            if env[i..].starts_with(right_chars) {
                // 计算边界索引
                let right_border = i + right_len_chars;
                // 返回`Ok(右边界起始索引)`
                return Ok(right_border);
            }
            // 检测字符是否合法
            match verify_char(env[i]) {
                // 合法⇒索引步进
                true => i += 1,
                // 非法⇒解析失败⇒返回`Err(非法字符所在索引)`
                false => return Err(i),
            }
        }
        // 未找到终止括弧 ⇒ `Err(环境长度即索引右边界)`
        Err(i)
    }

    /// 🛠️工具函数/在环境中从某处索引截取字符序列
    /// * 持续【从右到左】匹配，直到左边界/非法字符/环境边界为止
    ///   * 左边界⇒`Ok(左边界起始索引)`
    ///   * 非法字符⇒`Ok(非法字符所在索引)`
    ///   * 环境边界⇒`Ok(环境长度即索引左边界)`
    /// * 🎯对应PEG中的Any/Some逻辑
    /// * 🚩【2024-03-18 08:47:12】现在基本确立「延迟截取字符串」原则
    ///   * 不到需要的时候，一律以「起止索引」表示「字符串」
    ///   * 后续一律从[`String::from_iter`]转换
    /// * 📌「在指定位置开始」的情形，完全可以通过「预先对环境切片」解决
    ///   * 📄例如：`("abc", start = 1)` ⇒ `(&"abc"[..2])`
    #[inline(always)]
    fn segment_some_suffix(
        &self,
        env: ParseEnv<'a>,
        left_chars: ParseEnv,
        verify_char: impl Fn(char) -> bool,
    ) -> Result<ParseIndex, ParseIndex> {
        // 自动计算长度，然后从末尾开始
        let mut right_border = env.len();
        while right_border > 0 {
            // 左括弧⇒预先返回
            if env[..right_border].ends_with(left_chars) {
                // 计算边界索引
                let left_border = right_border - left_chars.len();
                // 返回`Ok(左括弧起始索引)`
                return Ok(left_border);
            }
            // 检测「边界内要检验的字符」是否合法 | 环境是否终止
            let char_will_pass = env[right_border - 1];
            match verify_char(char_will_pass) {
                // 合法 ⇒ 索引步进
                true => right_border -= 1,
                // 非法 ⇒ 返回 `Err(非法字符所在索引)`
                false => return Err(right_border),
            }
        }
        // 找不到左括弧 ⇒ 返回`Err(环境长度即索引左边界)`
        Err(0)
    }

    /// 工具函数/依照「前缀匹配」与「内部合法字符」选取区间
    /// * 🎯【2024-03-18 09:15:24】再度抽象复用「前缀截取预算」
    /// * 📌「在指定位置开始」的情形，完全可以通过「预先对环境切片」解决
    ///   * 📄例如：`("abc", start = 1)` ⇒ `(&"abc"[1..])`
    fn segment_brackets_prefix(
        &self,
        env: ParseEnv<'a>,
        brackets: impl PrefixMatch<(&'a str, &'a str)>,
        verify_char: impl Fn(char) -> bool,
    ) -> Option<(String, ParseIndex)> {
        // 尝试前缀匹配
        let (left, right) = brackets.match_prefix_char_slice(env)?;

        // 匹配成功⇒将右括弧变成字符数组 | 字符数组不能直接与「静态字串」比对
        let right_chars = right.chars().collect::<Vec<_>>();

        // 然后从左括弧尾部开始尝试截取
        let result = self.segment_some_prefix(env, left.chars().count(), &right_chars, verify_char);

        // 从返回结果计算左右边界，并尝试返回结果字符串
        match result {
            Ok(right_border) => {
                // 从给定的左边界从头开始截取
                let result = String::from_iter(&env[..right_border]);
                // 返回
                Some((result, right_border))
            }
            // 中间字符非法 || 未找到右括弧 ⇒ 解析失败
            Err(..) => None,
        }
    }

    /// 工具函数/依照「后缀匹配」与「内部合法字符」选取区间
    /// * 🎯【2024-03-18 09:15:24】再度抽象复用「后缀截取预算」
    /// * 📌「在指定位置开始」的情形，完全可以通过「预先对环境切片」解决
    ///   * 📄例如：`("abc", start = 1)` ⇒ `(&"abc"[..2])`
    fn segment_brackets_suffix(
        &self,
        env: ParseEnv<'a>,
        brackets: impl SuffixMatch<(&'a str, &'a str)>,
        verify_char: impl Fn(char) -> bool,
    ) -> Option<(String, ParseIndex)> {
        // 尝试后缀匹配
        let (left, right) = brackets.match_suffix_char_slice(env)?;

        // 匹配成功⇒将左括弧变成字符数组 | 字符数组不能直接与「静态字串」比对
        let left_chars = left.chars().collect::<Vec<_>>();

        // 然后从右括弧头部开始，尝试截取
        let env_content = &env[..env.len() - right.chars().count()];
        let result = self.segment_some_suffix(
            env_content,
            // * 减去右括弧长度 | 语义：右边界而非位置（相比「后缀」而言）
            &left_chars,
            verify_char,
        );

        // 从返回结果计算左右边界，并尝试返回结果字符串
        match result {
            Ok(left_border) => {
                // 从给定的右边界从头开始截取
                let result = String::from_iter(&env[left_border..]);
                // 返回
                Some((result, left_border))
            }
            // 中间字符非法 || 未找到左括弧 ⇒ 解析失败
            Err(..) => None,
        }
    }

    /// 前缀截取预算
    /// * 🚩直接在整个环境中进行「前缀截取」
    /// * ⚙️返回一个可空值
    ///   * 📌要么「没匹配到合法的预算值（[`None`]）」
    ///   * 📌要么返回「匹配到的完整预算值，以及其在『解析环境』中的**右边界**（用于切分词项）」
    ///     * 🎯返回并直接使用「词项部分」的开头索引，同时也无需做「-1」偏移
    /// * 📄匹配的环境如：`$0.5;0.5;0.5$<A-->B>.%1.0;0.9%`
    /// * 📄匹配的结果如：`Some(("$0.5;0.5;0.5$", 12))` | `12` 对应第二个`$`
    fn segment_budget(&self, env: ParseEnv<'a>) -> Option<(String, ParseIndex)> {
        // 尝试前缀匹配
        self.segment_brackets_prefix(
            env,
            self.format.task.budget_brackets,
            &self.format.task.is_budget_content,
        )
    }

    /// 后缀截取真值
    /// * 🚩直接在整个环境中进行「后缀截取」
    /// * ⚙️返回一个可空值
    ///   * 📌要么「没匹配到合法的真值（[`None`]）」
    ///   * 📌要么返回「匹配到的完整真值，以及其在『解析环境』中的开头位置（用于切分时间戳）」
    /// * 📄匹配的环境如：`$0.5;0.5;0.5$<A-->B>.%1.0;0.9%`
    /// * 📄匹配的结果如：`Some(("$0.5;0.5;0.5$", 21))` | `21` 对应第一个`%`
    fn segment_truth(&self, env: ParseEnv<'a>) -> Option<(String, ParseIndex)> {
        // 尝试后缀匹配
        self.segment_brackets_suffix(
            env,
            self.format.sentence.truth_brackets,
            &self.format.sentence.is_truth_content,
        )
    }

    /// 向前截取时间戳
    /// * 🚩在「分割真值」[`segment_truth`]后，继续向前「后缀匹配」分割「时间戳」
    ///   *  💭大体还是使用「括弧匹配」的思路
    ///   * ❓如何解决「固定时间戳」与「枚举时间戳」的问题
    ///     * 💫漢文中不设固定「括弧」怎么解决？
    /// * ⚙️返回一个可空值
    ///   * 📌要么「没匹配到合法的时间戳（[`None`]）」
    ///   * 📌要么返回「匹配到的完整时间戳，以及其在『解析环境』中的开头位置（用于切分标点）」
    /// * 📄匹配的环境如：`G!:|:%1.0;0.9%`
    /// * 📄匹配的结果如：`Some((":|:", 2))` | `2` 对应第一个`:`
    fn segment_stamp(&self, env: ParseEnv<'a>) -> Option<(String, ParseIndex)> {
        // TODO: 有待完成
        todo!("有待完成")
    }

    /// 向前截取标点
    /// * 🚩在「分割时间戳」[`segment_stamp`]后，继续向后「前缀匹配」分割「标点」
    ///   * 直接使用「后缀匹配」的思路
    ///   * 匹配不到就返回空
    /// * ⚙️返回一个可空值
    ///   * 📌要么「没匹配到合法的标点（[`None`]）」
    ///   * 📌要么返回「匹配到的完整标点，以及其在『解析环境』中的开头位置（用于切分出词项）」
    /// * 📄匹配的环境如：`G!:|:%1.0;0.9%`
    /// * 📄匹配的结果如：`Some(("!", 1))` | `1` 对应`!`
    fn segment_punctuation(&self, env: ParseEnv<'a>) -> Option<(String, ParseIndex)> {
        // TODO: 有待完成
        todo!("有待完成")
    }

    /// 递归解析词项
    /// * 🚩分「复合」「陈述」「原子」三类
    ///   * 💭层层递归深入
    /// * ⚙️返回一个可空值
    ///   * 📌要么「词项解析失败」
    ///   * 📌要么返回「词项解析成功（仅词项）」
    /// * 💭至于「返回位置标识」可能需要在专门的「分割词项」方法中
    ///   * 🎯复合词项/陈述中的「词项分割」
    ///   *
    fn parse_term(&self, env: ParseEnv<'a>) -> ParseResult<Option<Term>> {
        // TODO: 有待完成
        todo!("有待完成")
    }
}

/// 侧门 [`NarseseFormat::parse(format, input)`]
/// * 💭为何一定要绑在「Narsese格式」中呢？
///   * 🚩【2024-03-16 22:12:01】随即独立
impl<'a> NarseseFormat<'a> {
    /// 主解析函数@字符串
    /// * 🚩【2024-03-16 21:30:25】放弃使用「字符迭代器」的方案
    ///   * ❗本身并没多少实际的「应用场景」
    pub fn parse(&self, input: &str) -> ParseResult {
        parse(self, input)
    }
}

/// 单元测试
#[cfg(test)]
mod test {

    use util::asserts;

    use super::super::format_instances::*;
    use super::*;

    /// 通通用测试/尝试解析并返回错误
    fn __test_parse(format: &NarseseFormat, input: &str) -> Narsese {
        // 解析
        let result = format.parse(input);
        // 检验
        match result {
            // 词项⇒解析出词项
            Ok(result) => result,
            // 错误
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    // 测试case统一定义
    macro_rules! test_segment {
        (@PARSE $format:expr, $state:expr, $f:ident; $env_str:expr) => {{
            // 从自面量构建「理想化环境」
            let env = idealize_env($format, $env_str);
            // 解析并返回结果
            $state.$f(&env)
        }};
        { $format:expr, $state:expr, $f:ident; $( $env_str:expr => ($item:expr, $index:expr $(,)?) $(,)? )+ } => {
            $(
                // 构造环境并解析出结果
                let result = test_segment!(@PARSE $format, $state, $f; $env_str);
                // 解构（成功的）结果
                let (truth, last_index) = result.expect("解析失败！");
                // 断言
                asserts! {
                    truth => $item,
                    last_index => $index
                }
            )+
        };
        { $format:expr, $state:expr, $f:ident; $( $env_str:expr $(,)? )+ } => {
            $(
                // 构造环境并解析出结果
                let result = test_segment!(@PARSE $format, $state, $f; $env_str);
                // 断言
                asserts! {
                    result => None // 解析失败
                }
            )+
        };
    }

    /// 测试/前缀截取预算
    #[test]
    fn test_segment_budget() {
        let format = &FORMAT_ASCII;
        let state = ParseState::new(format);

        // case统一定义
        macro_rules! test_budget {
            { $( $content:tt )+ } => {
                test_segment! {
                    format, state, segment_budget;
                    $($content)+
                }
            };
        }

        // 成功case
        let expected_str = "$0.5;0.5;0.5$";
        test_budget! {
            "$0.5; 0.5; 0.5$" => (expected_str, expected_str.len())
        }

        // 所有的失败case
        test_budget! {
            // 失败case 1 | 没找到右括弧
            "$0.5; 0.5; 0.5"
            // 失败case 2 | 前后缀不匹配
            "(0.5; 0.5; 0.5)"
            // 失败case 3 | 前缀不匹配
            "0.5; 0.5; 0.5$"
            // 失败case 4 | 非法字符
            "$0.5; 0.5; +0.5$"
            // 失败case 5 | 只有左括弧
            "$"
            // 失败case 6 | 不是开头前缀
            "❌$0.5; 0.5; 0.5$"
        };
    }

    /// 测试/后缀截取真值
    #[test]
    fn test_segment_truth() {
        let format = &FORMAT_ASCII;
        let state = ParseState::new(format);

        // case统一定义
        macro_rules! test_truth {
            { $( $content:tt )+ } => {
                test_segment! {
                    format, state, segment_truth;
                    $($content)+
                }
            };
        }

        // 成功cases
        test_truth! {
            "%1.0; 0.9%" => (
                "%1.0;0.9%", // 过滤掉了空格
                0, // 是「潜在的时间戳」的右边界
            )
            "<A --> B>.\n:|:\t%1.0; 0.9%" => (
                "%1.0;0.9%", // 过滤掉了空格
                // ! 理想化之后变成 "<A-->B>.:|:%1.0;0.9%"
                // * 时间戳的右边界 第一个'%'
                "<A-->B>.:|:%1.0;0.9%".find('%').unwrap(),
            )
        };

        // 所有的失败case
        test_truth! {
            // 失败case 1 | 没找到左括弧
            "1.0; 0.9%"
            // 失败case 2 | 前后缀不匹配
            "(1.0; 0.9)"
            // 失败case 3 | 后缀不匹配
            "%1.0; 0.9"
            // 失败case 4 | 非法字符
            "%1.0; +0.9%"
            // 失败case 5 | 只有右括弧
            "%"
            // 失败case 6 | 不是末尾后缀
            "%1.0; 0.9%❌"
        };
    }
}
