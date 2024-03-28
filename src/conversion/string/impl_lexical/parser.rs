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
//! * 🚩【2024-03-19 20:28:45】初步完成解析功能
//!   * 📌从「陈述环境特殊匹配」到「类似『枚举Narsese』的『前缀匹配解析』」
//!   * 📝许多波折：有关「空前缀原子词项（词语）」「原子词项字符集与陈述系词重复，吃掉陈述系词」的问题，
//!     * ❌在陈述中使用后缀匹配谓词，然后匹配系词：对「空前缀原子词项」无法（不依靠陈述系词数据）判断终止条件
//!     * ❌对「原子词项作为陈述主词」特殊处理：接近重写「词项解析」逻辑
//!   * 💫即便使用「字符数组切片」，「截取子环境→子环境解析」的作用仍然有限
//!     * 许多时候仍然是在模拟「枚举Narsese」的「头索引递进」机制

use super::NarseseFormat;
use crate::{
    api::UIntPrecision,
    lexical::{Budget, Narsese, Sentence, Task, Term, Truth},
};
use std::{error::Error, fmt::Display};
use util::{PrefixMatch, StartsWithStr, SuffixMatch};

/// 词法解析 辅助结构对象
/// * 🚩放在一个独立的模块内，以便折叠
pub mod structs {
    use super::*;
    use crate::lexical::{Budget, Punctuation, Stamp, Truth};

    /// 定义「解析环境」：字符数组切片
    pub type ParseEnv<'a> = &'a [char];

    /// 定义具备所有权的「解析环境」：字符数组
    pub type ParseEnvOwned = Vec<char>;

    /// 定义「解析索引」
    /// * 🎯用于区分「长度」与「位置」：与直接使用的`UIntPrecision`区分开
    pub type ParseIndex = UIntPrecision;

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
        pub budget: Option<Budget>,
        /// 词项
        pub term: Option<Term>,
        /// 标点
        pub punctuation: Option<Punctuation>,
        /// 时间戳
        pub stamp: Option<Stamp>,
        /// 真值
        pub truth: Option<Truth>,
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
                        stamp: stamp.unwrap_or(Stamp::new()),
                        truth: truth.unwrap_or(Truth::new()),
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
                    stamp: stamp.unwrap_or(Stamp::new()),
                    truth: truth.unwrap_or(Truth::new()),
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
        pub format: &'a NarseseFormat,
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

        /// 快速构造`ParseError`
        pub fn parse_error(&self, env: ParseEnv<'a>, message: &str) -> ParseError {
            ParseError::new(message, env)
        }

        /// 快速构造`Err`
        pub fn err<T>(&self, env: ParseEnv<'a>, message: &str) -> ParseResult<T> {
            Err(self.parse_error(env, message))
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
        let (truth, right_border) = truth.right_unwrap_or(env.len());

        // 时间戳
        let stamp = self.segment_stamp(&env[..right_border]);
        // 默认值 "" | 标点的索引上界（不含）
        let (stamp, right_border) = stamp.right_unwrap_or(right_border);

        // 标点
        let punctuation = self.segment_punctuation(&env[..right_border]);
        // 默认值 "" | 词项的索引上界（不含）
        let (punctuation, right_border) = punctuation.right_unwrap_or(right_border);

        // 前后缀切割完毕，最后解析出词项 //
        // 获得「词项」的「字符数组切片」
        let env_term = &env[begin_index..right_border];

        // 开始解析词项
        let term = match begin_index < right_border {
            // 在此提取词项
            // ! 解析过程出错，仍然上报错误
            true => Some(self.segment_term(env_term)?.0),
            // ! 🚩不再上抛错误，而是诚实反馈「解析失败」
            false => None,
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
    ///   * 非法字符⇒`Err(非法字符所在索引)`
    ///   * 环境边界⇒`Err(环境长度即索引右边界)`
    /// * 🎯对应PEG中的Any/Some逻辑
    /// * 🚩【2024-03-18 08:47:12】现在基本确立「延迟截取字符串」原则
    ///   * 不到需要的时候，一律以「起止索引」表示「字符串」
    ///   * 后续一律从[`String::from_iter`]转换
    /// * 📌「在指定位置开始」的情形，的确可以通过「预先对环境切片」解决
    ///   * 📄例如：`("abc", start = 1)` ⇒ `(&"abc"[1..])`
    ///   * ⚠️但需要面对「切片之后索引不一致」以及「切片本身有性能开销」的问题
    ///     * 特别是在「前缀截取」之后，索引应该随即改变
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

    /// 🛠️工具函数/在环境中从某处前缀截取字符序列
    /// * 🎯用于词项的「非贪婪有条件前缀匹配」
    /// * 持续【从左到右】匹配，直到非法字符/环境边界为止
    ///   * 非法字符⇒`非法字符所在索引`
    ///   * 环境边界⇒`环境长度即索引右边界`
    /// * 📌相比[`Self::segment_some_prefix`]不再有（固定的）右括号
    /// * 🎯对应PEG中的Any/Some逻辑
    /// * 🚩【2024-03-18 08:47:12】现在基本确立「延迟截取字符串」原则
    /// * 📄参考：[`Self::segment_some_prefix`]
    /// * 🚩【2024-03-28 14:08:31】现在恢复「系词前缀匹配」规则
    #[inline(always)]
    fn collect_some_prefix(
        &self,
        env: ParseEnv<'a>,
        start: ParseIndex,
        verify: impl Fn(ParseIndex, char) -> bool,
    ) -> ParseIndex {
        // 从起始索引处开始
        // ! 🚩此处不能用迭代器：`env[start..].iter().position`索引是【相对切片】而非【相对开头】
        let mut i = start;
        let len_env = env.len();
        while i < len_env {
            // 检测字符是否合法
            match verify(i, env[i]) {
                true => i += 1,
                false => return i,
            }
        }
        // 若没找到，以环境长度为右边界
        len_env
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
    /// * 📌「在指定位置开始」的情形，的确可以通过「预先对环境切片」解决
    ///   * 📄例如：`("abc", start = 1)` ⇒ `(&"abc"[..2])`
    ///   * ⚠️但需要面对「切片之后索引不一致」以及「切片本身有性能开销」的问题
    ///     * 特别是在「前缀截取」之后，索引应该随即改变
    #[inline(always)]
    fn segment_some_suffix(
        &self,
        env: ParseEnv<'a>,
        left_chars: ParseEnv,
        verify_char: impl Fn(char) -> bool,
    ) -> Result<ParseIndex, ParseIndex> {
        // 自动计算长度，然后从末尾开始
        let mut right_border = env.len();
        loop {
            // 左括弧⇒预先返回
            // * 兼任「零长字串检测」的作用
            if env[..right_border].ends_with(left_chars) {
                // 计算边界索引
                let left_border = right_border - left_chars.len();
                // 返回`Ok(左括弧起始索引)`
                break Ok(left_border);
            }
            // 检查边界 | 找不到左括弧 ⇒ 返回`Err(环境长度即索引左边界)`
            if right_border == 0 {
                break Err(0);
            }
            // 检测「边界内要检验的字符」是否合法 | 环境是否终止
            let char_will_pass = env[right_border - 1];
            match verify_char(char_will_pass) {
                // 合法 ⇒ 索引步进
                true => right_border -= 1,
                // 非法 ⇒ 返回 `Err(非法字符所在索引)`
                false => break Err(right_border),
            }
        }
    }

    /// 工具函数/依照「前缀匹配」与「内部合法字符」选取区间
    /// * 🎯【2024-03-18 09:15:24】再度抽象复用「前缀截取预算」
    /// * 📌「在指定位置开始」的情形，完全可以通过「预先对环境切片」解决
    ///   * 📄例如：`("abc", start = 1)` ⇒ `(&"abc"[1..])`
    /// * ❌【2024-03-18 22:16:16】尝试兼容`String`与`&str`失败
    ///   * 兼容对象：
    ///     * `PrefixMatch<(String, String)>`
    ///     * `PrefixMatch<(&'a str, &'a str)>`
    ///
    /// ! ❌【2024-03-18 22:15:48】通过「`S: Deref<Target = str>`」的方法行不通
    ///
    /// ❌旧签名：
    /// ```no-test
    /// fn segment_brackets_prefix<S: Deref<Target = str>>(
    ///    &self,
    ///    env: ParseEnv<'a>,
    ///    brackets: impl PrefixMatch<(S, S)>,
    ///    verify_char: impl Fn(char) -> bool,
    ///) -> Option<(String, ParseIndex)>
    /// ```
    ///
    /// ⚠️【2024-03-18 22:18:40】无论是`brackets`中的元组参数填`(S, S)`还是`(&'s S, &'s S)`均不通过编译
    /// * 📌最接近的一次报错：`cannot move out of `self.format.task.budget_brackets` which is behind a shared reference`
    ///   * ❌但很可惜，不能拿掉格式对象中字段数据的所有权
    /// * 📌若为`&'s S`（引入新的生命周期参数），则特征不兼容
    /// * 📝【2024-03-19 00:15:02】似乎`rust,no-test`在此又失效了
    fn segment_brackets_prefix(
        &self,
        env: ParseEnv<'a>,
        brackets: &impl PrefixMatch<(String, String)>,
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
        brackets: &impl SuffixMatch<(String, String)>,
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
    fn segment_budget(&self, env: ParseEnv<'a>) -> Option<(Budget, ParseIndex)> {
        // * 📌至于「解析出『vec![".9"]』和『vec!["0.9"]』之后，如何能判等」的问题：不应该以这里的「词法Narsese」作为判等依据
        // 尝试前缀匹配
        let (budget_string, right_border) = self.segment_brackets_prefix(
            env,
            &self.format.task.budget_brackets,
            &self.format.task.is_budget_content,
        )?;
        // 截去头尾俩括弧
        let budget_string = budget_string
            .trim_start_matches(&self.format.task.budget_brackets.0)
            .trim_end_matches(&self.format.task.budget_brackets.1);
        // 然后使用「预算分隔符」进行分割
        // * 🚩【2024-03-22 20:13:04】目前专注上层，不再细写字串分割逻辑了
        // * 🚩【2024-03-24 02:57:17】此处的空字串必须被过滤掉，以便让`$$`等价于`[]`而非`[""]`
        Some((
            budget_string
                .split(&self.format.task.budget_separator)
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
                .collect::<Budget>(),
            right_border,
        ))
    }

    /// 后缀截取真值
    /// * 🚩直接在整个环境中进行「后缀截取」
    /// * ⚙️返回一个可空值
    ///   * 📌要么「没匹配到合法的真值（[`None`]）」
    ///   * 📌要么返回「匹配到的完整真值，以及其在『解析环境』中的开头位置（用于切分时间戳）」
    /// * 📄匹配的环境如：`$0.5;0.5;0.5$<A-->B>.%1.0;0.9%`
    /// * 📄匹配的结果如：`Some(("$0.5;0.5;0.5$", 21))` | `21` 对应第一个`%`
    fn segment_truth(&self, env: ParseEnv<'a>) -> Option<(Truth, ParseIndex)> {
        // 尝试后缀匹配
        let (truth_string, right_border) = self.segment_brackets_suffix(
            env,
            &self.format.sentence.truth_brackets,
            &self.format.sentence.is_truth_content,
        )?;
        // 截去头尾俩括弧
        let truth_string = truth_string
            .trim_start_matches(&self.format.sentence.truth_brackets.0)
            .trim_end_matches(&self.format.sentence.truth_brackets.1);
        // 然后直接使用「预算分隔符」进行分割
        // * 🚩【2024-03-22 20:13:04】目前专注上层，不再细写字串分割逻辑了
        // * 🚩【2024-03-24 02:57:17】此处的空字串必须被过滤掉，以便让`$$`等价于`[]`而非`[""]`
        Some((
            // 不要括弧！
            truth_string
                // 拆分
                .split(&self.format.sentence.truth_separator)
                .map(str::to_owned)
                .filter(|s| !s.is_empty())
                .collect::<Truth>(),
            right_border,
        ))
    }

    /// 向前截取时间戳
    /// * 🚩在「分割真值」[`segment_truth`]后，继续向前「后缀匹配」分割「时间戳」
    ///   *  💭大体还是使用「括弧匹配」的思路
    ///   * ❓如何解决「固定时间戳」与「枚举时间戳」的问题
    ///     * 💫漢文中不设固定「括弧」怎么解决？
    /// * ⚙️返回一个可空值
    ///   * 📌要么「没匹配到合法的时间戳（[`None`]）」
    ///   * 📌要么返回「匹配到的完整时间戳，以及其在『解析环境』中的开头位置（用于切分标点）」
    /// * 📄匹配的环境如：`G!:|:`
    ///   * ⚠️此时应该已经截去了真值
    /// * 📄匹配的结果如：`Some((":|:", 2))` | `2` 对应第一个`:`
    fn segment_stamp(&self, env: ParseEnv<'a>) -> Option<(String, ParseIndex)> {
        // 尝试后缀匹配
        self.segment_brackets_suffix(
            env,
            &self.format.sentence.stamp_brackets,
            &self.format.sentence.is_stamp_content,
        )
    }

    /// 向前截取标点
    /// * 🚩在「分割时间戳」[`segment_stamp`]后，继续向后「前缀匹配」分割「标点」
    ///   * 直接使用「后缀匹配」的思路
    ///   * 匹配不到就返回空
    /// * ⚙️返回一个可空值
    ///   * 📌要么「没匹配到合法的标点（[`None`]）」
    ///   * 📌要么返回「匹配到的完整标点，以及其在『解析环境』中的开头位置（用于切分出词项）」
    /// * 📄匹配的环境如：`<A-->B>!`
    /// * 📄匹配的结果如：`Some(("!", 7))` | `7` 对应`!`
    fn segment_punctuation(&self, env: ParseEnv<'a>) -> Option<(String, ParseIndex)> {
        // 尝试解析出标点
        let punctuation = self
            .format
            .sentence
            .punctuations
            .match_suffix_char_slice(env)?
            .clone();
        // 跳过标点
        let var_name = env.len() - punctuation.chars().count();
        Some((punctuation, var_name))
    }

    /// 递归解析词项
    /// * 内部函数[`Self::segment_term`]的独立对外接口
    /// * 🚩返回一个包含「词项」或「解析错误」的结果
    pub fn parse_term(&self, input: &str) -> ParseResult<Term> {
        let idealized = idealize_env(self.format, input);
        Ok(self.segment_term(&idealized)?.0)
    }

    /// 递归分隔词项
    /// * 🚩分「集合」「复合」「陈述」「原子」四类
    ///   * 💭层层递归深入
    /// * ⚙️返回一个可空值
    ///   * 📌要么「词项解析失败」
    ///   * 📌要么返回「解析成功」：词项及其右边界（即长度）
    /// * 🚩因为「递归解析」需要传递信息，故需要额外传递索引
    /// * 📌不传递额外信息、直接传递字符串的才能叫「parse」
    fn segment_term(&self, env: ParseEnv<'a>) -> ParseResult<(Term, ParseIndex)> {
        // 先解析「集合词项」
        if let Ok(result) = self.segment_term_set(env) {
            return Ok(result);
        }
        // 然后解析「复合词项」
        if let Ok(result) = self.segment_compound(env) {
            return Ok(result);
        }
        // 再解析「陈述」
        if let Ok(result) = self.segment_statement(env) {
            return Ok(result);
        }
        // 最后解析「原子」 | 此时不会附加「停止条件」（只会在陈述上下文中开启）
        self.segment_atom(env)
    }

    /// 前缀解析原子词项（贪婪匹配）
    /// * 🎯正常情况下的原子词项：纯原子词项、复合词项中、陈述主词
    /// * ❗遇到陈述系词总会停下
    /// * ⚙️返回一个结果
    ///   * 📌要么返回解析错误
    ///   * 📌要么返回「匹配到的完整词项，以及其在『解析环境』中的右边界（用于切分出其它词项）」
    /// * 📄匹配的环境如：
    ///   * 单纯环境：`word` `^op` `+123` `$i_var`
    ///   * 复合环境：`{subject,predicate}` => `subject`
    ///   * 陈述环境：`subject-->predicate` => `subject`
    /// * 🚩现在不再辅以对应的「后缀匹配」方案
    ///   * 📌核心原因：「后缀匹配」的需求仅在「原子词项作陈述主词」时出现
    ///   * 📍解决方案：直接作为「陈述解析」的特殊情况对待
    /// * 🚩【2024-03-19 19:02:38】现在添加「额外停止条件」用以应对「吃掉系词」的情况
    fn segment_atom(&self, env: ParseEnv<'a>) -> ParseResult<(Term, ParseIndex)> {
        // 尝试解析出前缀
        let prefix = self
            // 匹配前缀
            .format
            .atom
            .prefixes
            .match_prefix_char_slice(env)
            // 从Option打包成Result，然后尝试解包
            .ok_or(self.parse_error(env, "未匹配到原子词项前缀"))?
            .to_owned();
        // 计算出所有系词的首字符 // ! 用于【统一】应对「分割陈述」时「原子词项做主词」的情况
        let copulas = &self.format.statement.copulas;
        // 计算出起始索引
        let content_start = prefix.chars().count();
        // 朝后贪婪扫描字符
        let right_border = self.collect_some_prefix(
            env,
            content_start,
            // 检验
            |i, c| {
                // 首先是合法字符
                (self.format.atom.is_identifier)(c) &&
                // 其次是「不能以系词作为开头」（遇到系词⇒截止）
                copulas.match_prefix_char_slice(&env[i..]).is_none()
            },
        );
        // 检查非空
        // ! 不允许名称为空的原子词项
        if content_start >= right_border && prefix.is_empty() {
            return self.err(env, "原子词项名称与前缀不能同时为空");
        }
        // 获取名称
        let name = String::from_iter(&env[content_start..right_border]);
        // 构造
        let term = Term::Atom { prefix, name };
        // 返回
        Ok((term, right_border))
    }

    /// 解析集合词项
    fn segment_term_set(&self, env: ParseEnv<'a>) -> ParseResult<(Term, ParseIndex)> {
        // 前缀匹配并跳过左括弧
        let (left, right) = self
            .format
            .compound
            .set_brackets
            .match_prefix_char_slice(env)
            .ok_or(self.parse_error(env, "缺少陈述左括弧"))?;

        // 前缀切片最需要注意的是长度
        let mut term_begin = left.chars().count();

        // 开始解析其中的元素
        let mut terms = Vec::new();
        let right_border;
        // 第一个元素
        let (term, term_len) = self.segment_term(&env[term_begin..])?;
        terms.push(term);
        term_begin += term_len;
        loop {
            // 右括弧⇒跳过，结束
            if env[term_begin..].starts_with_str(right) {
                right_border = term_begin + right.chars().count();
                break;
            }
            // 分隔符⇒跳过
            if env[term_begin..].starts_with_str(&self.format.compound.separator) {
                term_begin += self.format.compound.separator.chars().count();
            }
            // 解析一个词项
            let (term, term_len) = self.segment_term(&env[term_begin..])?;
            terms.push(term);
            term_begin += term_len;
        }

        // 解包 & 构造 //
        let term = Term::Set {
            left_bracket: left.clone(),
            terms,
            right_bracket: right.clone(),
        };
        // 返回
        Ok((term, right_border))
    }

    /// 解析复合词项
    fn segment_compound(&self, env: ParseEnv<'a>) -> ParseResult<(Term, ParseIndex)> {
        // 前缀匹配并跳过左括弧
        let (left, right) = self
            .format
            .compound
            .brackets
            .match_prefix_char_slice(env)
            .ok_or(self.parse_error(env, "缺少陈述左括弧"))?;

        // 前缀切片最需要注意的是长度
        let connecter_start = left.chars().count();

        // 解析连接符 //
        let connecter = self
            .format
            .compound
            .connecters
            .match_prefix_char_slice(&env[connecter_start..])
            .ok_or(self.parse_error(env, "缺少陈述左括弧"))?
            .clone();

        // 不断解析「分隔符-词项-分隔符-词项……」
        let mut terms = Vec::new();
        let mut term_begin = connecter_start + connecter.chars().count();
        let right_border;
        loop {
            // 右括弧⇒跳过，结束
            if env[term_begin..].starts_with_str(right) {
                right_border = term_begin + right.chars().count();
                break;
            }
            // 分隔符⇒跳过
            if env[term_begin..].starts_with_str(&self.format.compound.separator) {
                term_begin += self.format.compound.separator.chars().count();
            }
            // 解析一个词项
            let (term, term_len) = self.segment_term(&env[term_begin..])?;
            terms.push(term);
            term_begin += term_len;
        }

        // 解包 & 构造 //
        let term = Term::Compound { connecter, terms };
        // 返回
        Ok((term, right_border))
    }

    /// 解析陈述
    /// * 🎯基础、统一的陈述解析支持
    /// * ⚙️返回一个结果
    ///   * 📌要么返回解析错误
    ///   * 📌要么返回「匹配到的完整词项，以及其在『解析环境』中的右边界（用于切分出其它词项）」
    ///   * 📌为【原子词项作为主词】的特殊情况作适配
    /// * 📄匹配的环境如：
    ///   * 原子词项作为主词：`<A-->B>`
    ///   * 其它常规情况：`<(*,{SELF})-->yes>` `<<A-->B>==><B-->C>>`
    ///
    /// * ❌【2024-03-19 19:14:08】放弃对「原子词项作为主词」的适配：宁愿一刀切，也不要让代码变复杂
    /// * ❌【2024-03-19 19:10:28】不要过于复杂化：解析主词最好跟其它情况一样
    /// * ❌【2024-03-19 16:29:22】弃用「后缀匹配谓词，再以此定位系词」的方案：后缀匹配还得分开「无前缀原子词项」的情况
    /// * 🚩方案：使用「原子词项前缀」结合「原子词项内容（首个字符）」作为判断依据
    /// ! ⚠️不能直接使用「原子词项前缀」作为判断依据：必须考虑**空前缀**情况
    fn segment_statement(&self, env: ParseEnv<'a>) -> ParseResult<(Term, ParseIndex)> {
        // 前缀匹配并跳过左括弧
        let (left, right) = self
            .format
            .statement
            .brackets
            .match_prefix_char_slice(env)
            .ok_or(self.parse_error(env, "缺少陈述左括弧"))?;
        // 前缀切片最需要注意的是长度
        let subject_start = left.chars().count();

        // 解析主词 //
        // ! 【2024-03-19 19:26:16】现在不再特别区分对待「原子词项作为主词，贪婪解析内容吃掉系词」的情况了
        // * 🚩解决方案：「一刀切」拒绝系词开头作为原子词项内容
        let (subject, subject_len) = self.segment_term(&env[subject_start..])?;
        let copula_start = subject_start + subject_len;

        // 解析系词 //
        let copula = self
            .format
            .statement
            .copulas
            .match_prefix_char_slice(&env[copula_start..])
            .ok_or(self.parse_error(env, "未解析出系词"))?
            .clone();
        let predicate_start = copula_start + copula.chars().count();

        // 解析谓词 //
        let (predicate, relative_len) = self.segment_term(&env[predicate_start..])?;

        // 跳过右括弧 //
        let right_bracket_start = predicate_start + relative_len;
        let right_border = match env[right_bracket_start..].starts_with_str(right) {
            true => right_bracket_start + right.chars().count(),
            false => return self.err(env, "未匹配到右括弧"),
        };

        // 解包 & 构造 //
        let subject = Box::new(subject);
        let predicate = Box::new(predicate);
        let term = Term::Statement {
            subject,
            copula,
            predicate,
        };

        // 返回
        Ok((term, right_border))
    }
}

/// 侧门 [`NarseseFormat::parse(format, input)`]
/// * 💭为何一定要绑在「Narsese格式」中呢？
///   * 🚩【2024-03-16 22:12:01】随即独立
impl NarseseFormat {
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
    #![allow(unused)]

    use super::{super::format_instances::*, *};
    use crate::lexical::shortcuts::*;
    use util::*;

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
            // 从字面量构建「理想化环境」
            let env = idealize_env($format, $env_str);
            // 解析并返回结果
            $state.$f(&env)
        }};
        { $format:expr, $state:expr, $f:ident; $( $env_str:expr => ($item:expr, $index:expr $(,)?) $(,)? )+ } => {
            $(
                // 构造环境并解析出结果
                let result = test_segment!(@PARSE $format, $state, $f; $env_str);
                // 解构（成功的）结果
                let (result, last_index) = result.expect(&format!("「{:?}」解析失败！", $env_str));
                // 断言
                asserts! {
                    result => $item,
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
        let expected = budget!["0.5" "0.5" "0.5"];
        let idealized = "$0.5;0.5;0.5$"; // 去掉空格
        test_budget! {
            "$0.5; 0.5; 0.5$" => (expected, idealized.chars().count())
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
        let expected = vec!["1.0", "0.9"];
        let idealized = "%1.0;0.9%";
        test_truth! {
            "%1.0; 0.9%" => (
                expected, // 过滤掉了空格
                0, // 是「潜在的时间戳」的右边界
            )
            "<A --> B>.\n:|:\t%1.0; 0.9%" => (
                expected, // 过滤掉了空格
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

    /// 测试/后缀截取时间戳
    #[test]
    fn test_segment_stamp() {
        let format = &FORMAT_ASCII;
        let state = ParseState::new(format);

        // case统一定义
        macro_rules! test_stamp {
            { $( $content:tt )+ } => {
                test_segment! {
                    format, state, segment_stamp;
                    $($content)+
                }
            };
        }

        // 成功cases
        test_stamp! {
            ":|:" => (
                ":|:", // 过滤掉了空格
                0, // 是「潜在的时间戳」的右边界
            )
            " :!\t-123: " => (
                ":!-123:", // 过滤掉了空格
                0, // 是「潜在的时间戳」的右边界
            )
            "<A --> B>.\n:|:\t" => (
                ":|:", // 过滤掉了空格
                // ! 理想化之后变成 "<A-->B>.:|:
                // * 时间戳的右边界 第一个':'
                "<A-->B>.:|:".find(':').unwrap(),
            )
        };

        // 所有的失败case
        test_stamp! {
            // 失败case 1 | 没找到左括弧
            "+123:"
            // 失败case 2 | 前后缀不匹配
            "(+123)"
            // 失败case 3 | 后缀不匹配
            ":!+123"
            // 失败case 4 | 非法字符
            ":!_+123:"
            // 失败case 5 | 只有右括弧
            ":"
            // 失败case 6 | 不是末尾后缀
            ":!+123:❌"
        };
    }

    /// 测试/后缀截取时间戳
    #[test]
    fn test_segment_punctuation() {
        let format = &FORMAT_ASCII;
        let state = ParseState::new(format);

        // case统一定义
        macro_rules! test_segment_punctuation {
            { $( $content:tt )+ } => {
                test_segment! {
                    format, state, segment_punctuation;
                    $($content)+
                }
            };
        }

        // 成功cases
        test_segment_punctuation! {
            "! " => (
                "!", // 过滤掉了空格
                0, // 是「潜在的词项」的右边界
            )
            "<A --> B>." => (
                ".", // 过滤掉了空格
                // ! 理想化之后变成 "<A-->B>.:|:
                "<A-->B>".chars().count(), // 是「潜在的词项」的右边界
            )
        };

        // 所有的失败case
        test_segment_punctuation! {
            // 原子词项 //
            // 非法前缀
            ";" "#" r"$" "%"
            "^" "&" r"*" "-"
            "_" "+" r"=" "/"
            ":" "|" r"\" "0"
        };
    }

    // case统一定义
    macro_rules! test_parse_term {
        // 成功case
        {
            $state:expr;
            $( $narsese:expr => $expected:expr )*
        } => {
            asserts! {
                $(
                    $state
                        .parse_term($narsese)
                        .expect(&format!("词项「{}」解析失败！", $narsese))
                    => $expected
                )*
            }
        };
        // 成功case
        {
            $state:expr;
            $( $narsese:expr )*
        } => {
            asserts! {
                $(
                    {
                        let parsed = $state.parse_term($narsese);
                        if parsed.is_ok() {dbg!(&parsed);}
                        parsed.is_err()
                    }
                )*
            }
        };
    }

    /// 测试/解析词项
    #[test]
    fn test_parse_term() {
        let format = &FORMAT_ASCII;
        let state = ParseState::new(format);

        // 成功cases
        test_parse_term! {
            state;
            // 原子词项 //
            // 正常完整形式 | 会去掉空格
            "\n\tA" => atom!("A")
            "#A" => atom!("#" "A")
            "真の词项" => atom!("真の词项")
            "_" => atom!("_" "") // * 占位符
            "_占位符" => atom!("_" "占位符") // * 占位符
            // 舍去无效后缀
            "$A❗" => atom!("$" "A")
            "+123%%%" => atom!("+" "123")
            "^op --> あ" => atom!("^" "op")
            // 陈述 //
            "<^op --> あ>" => statement!(atom!("^" "op") "-->" atom!("あ"))
            "<<A --> B> ==> <B --> C>>" => statement!(
                statement!(atom!("A") "-->" atom!("B"))
                "==>"
                statement!(atom!("B") "-->" atom!("C"))
            )
            // 复合词项 //
            "(*, A, B, C)" => compound!("*"; atom!("A") atom!("B") atom!("C"))
            "(* A, B, C)" => compound!("*";
                // 此处允许没有分隔符
                atom!("A")
                atom!("B")
                atom!("C")
            )
            "(* A #B #C)" => compound!("*";
                // 此处允许没有分隔符
                atom!("" "A")
                atom!("#" "B")
                atom!("#" "C")
            )
            "(*, A  B, C)" => compound!(
                "*";
                atom!("AB") // * ←理想化去掉空格之后，这俩粘在一起
                atom!("C")
            )
            "(*, A)" => compound!("*"; atom!("A"))
            "(*, _)" => compound!("*"; atom!("_" ""))
            "(&&, <A --> B>, <B --> C>, <C --> D>)" => compound!(
                "&&";
                statement!(atom!("A") "-->" atom!("B"))
                statement!(atom!("B") "-->" atom!("C"))
                statement!(atom!("C") "-->" atom!("D"))
            )
            // 集合词项
            "{SELF}" => set!("{"; "SELF"; "}")
        }

        // 失败cases
        test_parse_term! {
            state;
            // 原子词项 //
            // 空内容
            ""
            // 非法前缀
            "@A"
            "&A"
            "*A"
            "%A"
            "!A"
            // "-A" // ! ❌【2024-03-28 14:09:31】现在已被兼容
            // 非法字符 | ⚠️不允许名称为空
            "❗"
            "!"
            "!因为前面这个非法前缀_这玩意儿无法被解析成原子词项"
            "~不会被解析到"
            // 复合词项/集合词项 //
            // 非法连接符
            "(A, B, C)"
            "(@, A, B, C)"
            "(;, A, B, C)"
            "(%, A, B, C)"
            "($, A, B, C)"
            "(#, A, B, C)"
            "(!, A, B, C)"
            "(^, A, B, C)"
            "(_, A, B, C)"
            // 缺少括弧
            "(*, A, B, C"
            "[A, B, C"
            "{A, B, C"
            // 多余括弧
            "((*, A, B, C)"
            "[[A, B, C]"
            "{{A, B, C}"
            // "(A, B, C))" // ! ←这些会只认前缀
            // "[A, B, C]]" // ! ←这些会只认前缀
            // "{A, B, C}}" // ! ←这些会只认前缀
            // 多余分隔符 | 分隔符可缺省，但不可多余
            "(*,, A,  B,  C )"
            "(*,  A,, B,  C )"
            "(*,  A,  B,, C )"
            "(*,  A,  B,  C,)"
            // 陈述 //
            // 缺少括弧
            "<A --> B"
            // 多余括弧
            "<<A ==> B>"
            // 非法系词
            "<A --> B ==> C>" // 连续系词不受支持
            "<A -|> B>"
            "<A -?> B>"
            "<A -#> B>"
            "<A ==< B>"
            "<A =>> B>"
            "<A -=> B>"
            "<A <-- B>"
            "<A <== B>"
            "<A <:> B>"
            "<A <#> B>"
            "<A ==@ B>"
            "<A --} B>"
            "<A [-- B>"
        }
    }

    /// 测试/所有条目
    #[test]
    fn test_parse_items() {
        // 背景
        fn test(format: &NarseseFormat, narsese: &str) {
            // 构建状态
            let mut state = ParseState::new(format);

            // 解析出条目（中间结果）
            let result = state
                .parse_items(&idealize_env(format, narsese))
                .expect("条目解析失败！");

            // 断言
            // * 📌【2024-03-18 22:50:58】此处至少要包括除了词项在内的所有数据
            asserts! {
                result.budget => @ Some(..)
                result.truth => @ Some(..)
                result.stamp => @ Some(..)
                result.punctuation => @ Some(..)
            }
        }

        // 批量生成的宏
        macro_rules! tests {
            {
                $format:expr;
                $($narsese:expr)*
            } => {
                $(
                    test($format, $narsese);
                )*
            };
        }

        // 测试 @ ASCII
        tests! {
            &FORMAT_ASCII;
            // 正常陈述
            "$0.5; 0.5; 0.5$ <A --> B>. :|: %1.0; 0.9%"
            // 原子词项 | 查询变量🆚问题
            "$0.5; 0.5; 0.5$ ?v? :|: %1.0; 0.9%"
            // 原子词项 | 独立变量🆚预算
            "$0.5; 0.5; 0.5$ $i_var@ :|: %1.0; 0.9%"
        }
    }

    /// 集中测试/鲁棒性
    #[test]
    fn test_parse_robust() {
        let format = &FORMAT_ASCII;
        let parse = |input| format.parse(input).expect("解析失败");
        let results = f_parallel![
            parse;
            "<(&&, <<$x-->A>==><$x-->B>>, <<$y-->C>==><$y-->D>>) ==> E>.";
            "<{tim} --> (/,livingIn,_,{graz})>. %0%";
            "<<(*,$1,sunglasses) --> own> ==> <$1 --> [aggressive]>>.";
            "<(*,{tom},sunglasses) --> own>.";
            "<<$1 --> [aggressive]> ==> <$1 --> murder>>.";
            "<<$1 --> (/,livingIn,_,{graz})> ==> <$1 --> murder>>.";
            "<{?who} --> murder>?";
            "<{tim} --> (/,livingIn,_,{graz})>.";
            "<{tim} --> (/,livingIn,_,{graz})>. %0%";
            "<<(*,$1,sunglasses) --> own> ==> <$1 --> [aggressive]>>.";
            "<(*,{tom},(&,[black],glasses)) --> own>.";
            "<<$1 --> [aggressive]> ==> <$1 --> murder>>.";
            "<<$1 --> (/,livingIn,_,{graz})> ==> <$1 --> murder>>.";
            "<sunglasses --> (&,[black],glasses)>.";
            "<{?who} --> murder>?";

            "<(&&,<(*,{$1},{$2},$d) --> 方向>, <(*,{$1},$c) --> 格点状态>, <(*,{$2},无缺陷) --> 格点状态>) ==> <(*,$d,$c,{$1},{$2}) --> [同色连空]>>. %1.00;0.999%";
            "<(*,{格点-4-5},缺陷1) --> 格点状态>. %1.00;0.999%";
        ];
        show!(&results);
        // for result in &results {
        //     assert!(result.is_ok());
        // }
    }
}
