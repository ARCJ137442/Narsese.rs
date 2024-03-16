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

use util::{first, PrefixMatch};

use super::NarseseFormat;
use crate::{
    lexical::{Narsese, Sentence, Task, Term},
    util::{BufferIterator, IntoChars},
};
use std::{error::Error, fmt::Display};

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
        env_slice: ParseEnvOwned,
        /// 出错所在的「解析索引」
        /// * 🎯用于指示出错位置
        index: ParseIndex,
    }
    impl ParseError {
        /// 工具函数/生成「环境切片」
        fn generate_env_slice(env: ParseEnv, index: ParseIndex) -> ParseEnvOwned {
            // 字符范围下限 | 后续截取包含
            let char_range_left = match index > ERR_CHAR_VIEW_RANGE {
                true => index - ERR_CHAR_VIEW_RANGE,
                false => 0,
            };
            // 字符范围上限 | 后续截取不包含
            let char_range_right = match index + ERR_CHAR_VIEW_RANGE + 1 < env.len() {
                true => index + ERR_CHAR_VIEW_RANGE + 1,
                false => env.len(),
            };
            // 截取字符，生成环境
            env[char_range_left..char_range_right].into()
        }

        /// 构造函数
        pub fn new(message: &str, env: ParseEnv, index: ParseIndex) -> ParseError {
            ParseError {
                message: message.to_string(),
                env_slice: ParseError::generate_env_slice(env, index),
                index,
            }
        }
    }
    /// 用于在报错时展示周边文本
    const ERR_CHAR_VIEW_RANGE: usize = 4;
    /// 呈现报错文本
    impl Display for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            // 输出
            write!(
                f,
                "Narsese解析错误：{} @ {} in {:?}",
                self.message,
                self.index,
                String::from_iter(self.env_slice.iter())
            )
        }
    }
    impl Error for ParseError {}

    /// 词法解析状态
    /// * 🚩只持有引用，相当于一个局部变量上下文
    #[derive(Clone)]
    pub struct ParseState<'a> {
        /// 词法格式
        /// * 📌用于指定解析所用的关键字
        pub format: &'a NarseseFormat<'a>,

        /// 解析环境：字符数组切片
        /// * 📌基本是唯一共享的状态
        pub env: &'a [char],
    }
    /// 通用实现 / 非「词法解析」的方法
    impl<'a> ParseState<'a> {
        /// 构造函数
        /// * ⚠️不从其它
        pub fn new(format: &'a NarseseFormat, env: ParseEnv<'a>) -> Self {
            Self { format, env }
        }
        pub fn from_owned_env(format: &'a NarseseFormat, env: &ParseEnvOwned) -> Self {
            Self::new(format, env)
        }
    }
}
use structs::*;

// 词法解析 正式逻辑开始 //

/// 总入口
/// * 🚩构造「解析状态」然后转发到「解析状态的实例方法」中去
pub fn parse<'a>(format: &NarseseFormat<'a>, input: &str) -> ParseResult {
    // 构造解析状态
    let chars = input.chars().collect::<Vec<_>>();
    let mut state = ParseState::new(format, &chars);
    // 用状态进行解析
    state.parse()
    // ! 随后丢弃状态
}

/// 开始在「解析状态」的基础上进行解析
impl<'a> ParseState<'a> {
    pub fn parse(&mut self) -> ParseResult {
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
}
