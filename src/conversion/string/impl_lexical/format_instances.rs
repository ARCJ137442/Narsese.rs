//! 定义「Narsese格式」的常用实例
//! * 📌最初自「枚举Narsese」拷贝过来
//! * 📌均基于CommonNarsese的语法格式，只是其中的「关键字」不同
//! * 📄部分参考自[JuNarsese](https://github.com/ARCJ137442/JuNarsese.jl)
//!   * ℹ️有少量修改
//! * 🚩【2024-03-18 22:23:20】现在全面采用具备所有权的[`String`]，放弃在此场合使用`&str`
//!   * 🎯避免后续解析器中「前后缀匹配」的无谓兼容
//!   * 🎯加快开发，牺牲一定性能，规避一系列的生命周期标注与复杂的生命周期问题

use super::format::*;
use lazy_static::lazy_static;
use nar_dev_utils::{
    bi_fix_match_dict_pair, suffix_match_dict_pair, x_fix_match_dict, PrefixMatchDict,
};

/// 工具宏：减少一些`into`
/// * 🎯元组⇒[`String`]，&str⇒[`String`]
macro_rules! s {
    ($l:literal) => {
        $l.to_string()
    };
    ( $($l:literal $(,)?)+ ) => {
        ($(s!($l)),+)
    };
}

// * 📝有关「全局常量」定义，闭包↔死局？ * //
// 这里不可以：`Box::new`并非常量函数
// pub const CLJ: Box<dyn Fn(char)> = Box::new(|_c: char| {});
// 这里也不可以 | `static`需要线程安全，而**闭包没法线程安全**
// pub static CLJ: Box<dyn Fn(char)> = Box::new(|_c: char| {});
// 即便外边是Arc，里边也不可以 | `static`需要线程安全，而**闭包没法线程安全**
// pub static CLJ: std::sync::Arc<dyn Fn(char)> = std::sync::Arc::new(|_c: char| {});
// ? 📄似乎有个使用`OnceCell`的方案：https://stackoverflow.com/questions/73260997/rust-boxed-closure-in-global-variable
//   ! ❌↑但上边这个方法报错：`dyn Fn`未实现`sync`，无法被装进Cell中
// ✅↑使用`lazy_static` + `Send + Sync`已解决
//
// * 📝使用`once_cell`的笔记
// use once_cell::sync::OnceCell;
// use std::collections::HashSet;
// lazy_static! {
//     static ref S: HashSet<i32> = {
//         let mut s = HashSet::new();
//         s.insert(0);
//         s
//     };
//     static ref B: Box<i32> = Box::new(0);
//     // ❌Mutex无法确保线程安全
//     // static ref C: Mutex<dyn Fn()> = Mutex::new(|| println!("I'm a closure!"));
//     // ❌函数指针无法确保线程安全
//     // static ref PF: &dyn Fn() = &imc;
//     // 🤣最后发现：直接在「闭包类型」中加上约束`Send + Sync`就可以了
//     //  * 📌因为实际上要加进去的俩闭包
//     static ref CLJ: OnceCell<Box<dyn Fn() + Send + Sync>> = {
//         let c = OnceCell::new();
//         c.get_or_try_init(||Result::<_,()>::Ok(create_function())).expect("无法初始化！");
//         c
//     };
// }
// fn create_function() -> Box<dyn Fn() + Send + Sync> {
//     Box::new(|| println!("I'm a closure!"))
// }
// #[test]
// fn t() {
//     use nar_dev_utils::show;
//     show!(S.clone());
//     let v = CLJ.get().unwrap();
//     v();

//     let is_space = &(FORMAT_ASCII.space.is_for_parse);
//     for c in ['1', 'c', ' ', '　', '\t', '\n'] {
//         show!(c, is_space(c));
//     }
// }
// ! 📝可以使用[`once_cell::Lazy`]实现`const`，但不采用
//   ! 📌这在Clippy看来更不安全：内部可变性安放在了常量之中

lazy_static! {
    /// 通用 ASCII格式
    /// * 来源：文档 `NARS ASCII Input.pdf`
    /// * 另可参考：<https://github.com/opennars/opennars/wiki/Narsese-Grammar-(Input-Output-Format)>
    /// * 可用于打印Narsese的默认形式
    ///
    /// * 📄使用[`lazy_static`]实现「静态常量」
    ///   * 详请参考[`create_format_ascii`]
    pub static ref FORMAT_ASCII: NarseseFormat = create_format_ascii();

    /// LaTeX扩展
    /// * 来源：文档 `NARS ASCII Input.pdf`
    /// * 【20230809 10:22:34】注：暂未找到官方格式模板，此仅基于个人观察
    /// * 【20230811 0:26:55】不能很好地兼容「二元运算」表达（需要更专业者优化）
    ///
    /// * 📄使用[`lazy_static`]实现「静态常量」
    ///   * 详请参考[`create_format_ascii`]
    pub static ref FORMAT_LATEX: NarseseFormat = create_format_latex();

    /// 漢文扩展
    /// * 📌原创
    ///
    /// * 📄使用[`lazy_static`]实现「静态常量」
    ///   * 详请参考[`create_format_ascii`]
    pub static ref FORMAT_HAN: NarseseFormat = create_format_han();
}

/// 简单判断是否为原子词项（标识符）
/// * 🚩仅使用一个有限的范围
/// * ⚠️若使用否定性匹配，一是影响性能，二是过于模糊（像是"wer#-12395%^#$"都会被匹配到）
/// * 🚩【2024-06-11 20:39:43】对emoji只进行有限度的支持（常见表情符号）
///   * 🔗参考：https://www.reddit.com/r/rust/comments/kohitu/how_to_check_if_a_char_is_emoji/
///   * 💭部分表情如"❗"等不受支持；范围不明，可能还会继续扩大
///   * 🔗另见：https://unicode.org/reports/tr51/index.html#emoji_data
/// * ⚠️目前[`char::is_alphanumeric`]还不是常量函数
fn is_identifier(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-' || c > '\u{1f2ff}' // 常见emoji兼容
}

/// 通用 ASCII格式
/// * 来源：文档 `NARS ASCII Input.pdf`
/// * 另可参考：<https://github.com/opennars/opennars/wiki/Narsese-Grammar-(Input-Output-Format)>
/// * 可用于打印Narsese的默认形式
/// * 🚩【2024-03-15 18:00:32】目前没法使用`const` `static`，只能使用函数【按需创建】格式
///   * ❌无法将其作为一个常量使用，即便其根本不会变化
///   * ❌使用`const`的方法行不通：包裹闭包的智能指针[`Box`]、[`Rc`]均无法作为常量初始化
///   * ❌使用`static`的方法行不通：闭包无法保证线程安全
///   * ✅使用[`lazy_static`]实现了一定的「静态常量」定义
///     * 🚩【2024-03-15 19:58:20】但目前仍然保留该工厂函数
/// * 🚩【2024-06-13 19:11:50】部分删除堆分配
///   * ✅成功通过「函数指针类型」去除了其中的Box堆分配，
///   * ⚠️但涉及「前后缀匹配字典」的堆分配，仍然需要使用[`lazy_static`]
pub fn create_format_ascii() -> NarseseFormat {
    const fn is_stamp_content(c: char) -> bool {
        matches!(c, '0'..='9' | '+' | '-') // regex:`[0-9+\-]`
    }
    const fn is_truth_content(c: char) -> bool {
        matches!(c, '0'..='9' | '.' | ';')
    }
    const fn is_budget_content(c: char) -> bool {
        matches!(c, '0'..='9' | '.' | ';')
    }
    NarseseFormat {
        space: NarseseFormatSpace {
            is_for_parse: char::is_whitespace, // ! 解析时忽略空格
            format_terms: s!(" "),             // 格式化时，词项间需要空格（英文如此）
            format_items: s!(" "),             // 格式化时，条目间需要空格（英文如此）
            remove_spaces_before_parse: true,  // ASCII版本空格无关
        },
        atom: NarseseFormatAtom {
            // 所有原子词项的前缀
            prefixes: x_fix_match_dict!(
                // 词语
                ""
                // 占位符
                "_"
                // 变量
                "$" "#" "?"
                // 间隔
                "+"
                // 操作符
                "^"
            ),
            // 一般文字、数字、连带`-`均算入在内
            is_identifier,
        },
        compound: NarseseFormatCompound {
            // 外延集/内涵集
            set_brackets: bi_fix_match_dict_pair!(
                "{" => "}" // 外延集
                "[" => "]" // 内涵集
            ),
            // 普通括号
            brackets: s!("(", ")"),
            // 普通分隔符
            separator: s!(","),
            // 复合词项连接符
            connecters: x_fix_match_dict!(
                "&"  // 外延交
                "|"  // 内涵交
                "-"  // 外延差
                "~"  // 内涵差
                "*"  // 乘积
                r"/" // 外延像
                r"\" // 内涵像
                "&&" // 合取
                "||" // 析取
                "--" // 否定
                "&/" // 顺序合取
                "&|" // 平行合取
            ),
        },
        statement: NarseseFormatStatement {
            // 陈述括弧
            brackets: s!("<", ">"),
            // 陈述系词
            copulas: x_fix_match_dict!(
                "-->" // 继承
                "<->" // 相似
                "==>" // 蕴含
                "<=>" // 等价
                "{--" // 实例
                "--]" // 属性
                "{-]" // 实例属性
                r"=/>" // 预测性蕴含
                r"=|>" // 并发性蕴含
                r"=\>" // 回顾性蕴含
                r"</>" // 预测性等价
                r"<|>" // 并发性等价
                r"<\>" // 回顾性等价
            ),
        },
        sentence: NarseseFormatSentence {
            // 所有标点
            punctuations: x_fix_match_dict!(
                "." // 判断
                "!" // 目标
                "?" // 问题
                "@" // 请求
            ),
            // 时间戳
            stamp_brackets: suffix_match_dict_pair!(
                // * 🚩空前缀匹配
                "" => r":\:" // 过去
                "" => r":|:" // 现在
                "" => r":/:" // 将来
                // * 📌ASCII版本经典使用双边括弧
                ":!" => r":" // 固定
            ),
            is_stamp_content,
            // 真值 | 内容已不包含空格
            truth_brackets: s!("%", "%"),
            truth_separator: s!(";"),
            // ! 【2024-03-22 20:23:39】↓虽说此时使用分隔符，但在「截取」阶段仍然需要将分隔符作为「内容」
            is_truth_content,
        },
        task: NarseseFormatTask {
            // 预算 | 内容已不包含空格
            budget_brackets: s!("$", "$"),
            budget_separator: s!(";"),
            is_budget_content,
        },
    }
}

/// LaTeX扩展
/// * 来源：文档 `NARS ASCII Input.pdf`
/// * 【20230809 10:22:34】注：暂未找到官方格式模板，此仅基于个人观察
/// * 【20230811 0:26:55】不能很好地兼容「二元运算」表达（需要更专业者优化）
/// * 📌【2024-03-17 11:00:17】现在对「\【字母串】」形式的LaTeX文本**强制要求后缀**`{}`以便实现「空格无关」
///   * ⚠️这可能会影响到「LaTeX→Narsese」的语法，但**LaTeX Narsese语法本身就是【面向输出】而非【面向解析】的**
///   * ℹ️LaTeX扩展本身不会有多少「需要由此转换成Narsese」的场景
/// * 🆕更新@2024-04-05：时序系词与时态由「前缀竖杠」变为「中缀竖杠」
pub fn create_format_latex() -> NarseseFormat {
    const fn is_stamp_content(c: char) -> bool {
        matches!(c, '0'..='9' | '+' | '-') // regex:`[0-9+\-]`
    }
    const fn is_truth_content(c: char) -> bool {
        matches!(c, '0'..='9' | '.' | ',') // ! LaTeX使用逗号而非分号
    }
    const fn is_budget_content(c: char) -> bool {
        matches!(c, '0'..='9' | '.' | ';')
    }
    NarseseFormat {
        space: NarseseFormatSpace {
            is_for_parse: char::is_whitespace, // ! 解析时可跳过空格
            format_terms: s!(" "),             // 格式化时，词项间需要分隔（避免代码粘连）
            format_items: s!(" "),             // 格式化时，条目间需要分隔（避免代码粘连）
            remove_spaces_before_parse: true,  // LaTeX版本亦可空格无关——通过「后缀空参数」省去空格
        },
        atom: NarseseFormatAtom {
            prefixes: x_fix_match_dict!(
                // 词语
                ""
                // 占位符
                r"\diamond{}" // ! 此处即「后缀空参数」
                // 变量
                r"\$" r"\#" "?"
                // 间隔
                "+"
                // 操作符
                r"\Uparrow{}" // ! 此处即「后缀空参数」
            ),
            is_identifier,
        },
        compound: NarseseFormatCompound {
            // 左右括弧
            // * 📌【2024-03-17 14:07:31】目前暂且不对`\left` `\right`做【括号封装】
            brackets: s!(r"\left(", r"\right)"),
            // 以（显式）空格作分隔符
            separator: s!(r"\;"), // ! LaTeX使用`\space{}`也可使用`\;` | ✅兼容MathJax
            // 词项集
            set_brackets: bi_fix_match_dict_pair!(
                // ! ↓此中`{` `}`需要转义
                r"\left\{" => r"\right\}" // 外延集
                r"\left[" => r"\right]" // 内涵集
            ),
            // 复合词项连接符
            connecters: x_fix_match_dict!(
                r"\cap{}" // 外延交
                r"\cup{}" // 内涵交
                r"\minus{}" // 外延差
                r"\sim{}" // 内涵差
                r"\times{}" // 乘积
                "/" // 外延像
                r"\backslash{}" // 内涵像
                r"\wedge{}" // 合取
                r"\vee{}" // 析取
                r"\neg{}" // 否定
                "," // 顺序合取
                ";" // 平行合取
            ),
        },
        statement: NarseseFormatStatement {
            brackets: s!(r"\left<", r"\right>"),
            copulas: x_fix_match_dict!(
                r"\rightarrow{}" // 继承
                r"\leftrightarrow{}" // 相似
                r"\Rightarrow{}" // 蕴含
                r"\Leftrightarrow{}" // 等价
                r"\circ\!\!\!\rightarrow{}" // 实例
                r"\rightarrow\!\!\!\circ{}" // 属性
                r"\circ\!\!\!\rightarrow\!\!\!\circ{}" // 实例属性
                r"/\!\!\!\!\!\Rightarrow{}" // 预测性蕴含
                r"|\!\!\!\!\!\Rightarrow{}" // 并发性蕴含
                r"\backslash\!\!\!\!\!\Rightarrow{}" // 回顾性蕴含
                r"/\!\!\!\Leftrightarrow{}" // 预测性等价
                r"|\!\!\!\Leftrightarrow{}" // 并发性等价
                r"\backslash\!\!\!\Leftrightarrow{}" // 回顾性等价
            ),
        },
        sentence: NarseseFormatSentence {
            // 标点
            punctuations: x_fix_match_dict!(
                "." // 判断
                "!" // 目标
                "?" // 问题
                "¿" // 请求
                // ! 💭【20230806 23:46:18】倒问号没有对应的LaTeX。。。
            ),
            // 时间戳
            stamp_brackets: suffix_match_dict_pair!(
                // * 🚩空前缀匹配
                "" => r"\backslash\!\!\!\!\!\Rightarrow{}" // 过去
                "" => r"|\!\!\!\!\!\Rightarrow{}" // 现在
                "" => r"/\!\!\!\!\!\Rightarrow{}" // 将来
                // !【2024-03-17 10:07:16】没有后缀，只以前缀区分
                "t=" => "", // ? LaTeX语法未知
            ),
            is_stamp_content,
            // 真值
            truth_brackets: s!(r"\langle{}", r"\rangle{}"),
            truth_separator: s!(","), // ! LaTeX格式使用`,`作为真值分隔符
            is_truth_content,
        },
        task: NarseseFormatTask {
            // 预算
            budget_brackets: s!(r"\$", r"\$"),
            budget_separator: s!(";"),
            is_budget_content,
        },
    }
}

/// 漢文扩展
/// * 📌原创
pub fn create_format_han() -> NarseseFormat {
    const fn is_stamp_content(c: char) -> bool {
        matches!(c, '0'..='9' | '+' | '-') // regex:`[0-9+\-]`
    }
    const fn is_truth_content(c: char) -> bool {
        matches!(c, '0'..='9' | '.' | '、') // 此处有特别的分隔符「、」
    }
    const fn is_budget_content(c: char) -> bool {
        matches!(c, '0'..='9' | '.' | '、') // 此处有特别的分隔符「、」
    }
    NarseseFormat {
        space: NarseseFormatSpace {
            is_for_parse: char::is_whitespace, // ! 解析时忽略空格
            format_terms: s!(""),              // 格式化时，词项间无需分隔（避免太过松散）
            format_items: s!(" "),             // 格式化时，条目间需要分隔（避免太过密集）
            // ! ❌【2024-03-22 23:25:40】暂时不能支持全角空格：枚举Narsese处只能有一种空格
            remove_spaces_before_parse: true, // 漢文亦空格无关
        },
        atom: NarseseFormatAtom {
            prefixes: x_fix_match_dict!(
                // 词语
                ""
                // 占位符
                "某"
                // 变量
                "任一" "其一" "所问"
                // 间隔
                "间隔"
                // 操作符
                "操作"
            ),
            is_identifier,
        },
        compound: NarseseFormatCompound {
            brackets: s!("（", "）"),
            separator: s!("，"),
            set_brackets: bi_fix_match_dict_pair!(
                "『" => "』" // 外延集
                "【" => "】" // 内涵集
            ),
            // 复合词项连接符
            connecters: x_fix_match_dict!(
                "外交" // 外延交
                "内交" // 内涵交
                "外差" // 外延差
                "内差" // 内涵差
                "积" // 乘积
                "外像" // 外延像
                "内像" // 内涵像
                "与" // 合取
                "或" // 析取
                "非" // 否定
                "接连" // 顺序合取
                "同时" // 平行合取
            ),
        },
        statement: NarseseFormatStatement {
            brackets: s!("「", "」"),
            copulas: x_fix_match_dict!(
                "是" // 继承
                "似" // 相似
                "得" // 蕴含
                "同" // 等价
                "为" // 实例
                "有" // 属性
                "具有" // 实例属性
                "将得" // 预测性蕴含
                "现得" // 并发性蕴含
                "曾得" // 回顾性蕴含
                "将同" // 预测性等价
                "现同" // 并发性等价
                "曾同" // 回顾性等价
            ),
        },
        sentence: NarseseFormatSentence {
            // 标点
            punctuations: x_fix_match_dict!(
                "。" // 判断
                "！" // 目标
                "？" // 问题
                "；" // 请求
                // ! 暂且没有更合适、更方便输入的全角标点
            ),
            // 时间戳
            stamp_brackets: suffix_match_dict_pair!(
                // * 🚩空前缀匹配
                "" => "过去" // 过去
                "" => "现在" // 现在
                "" => "将来" // 将来
                // !【2024-03-17 10:07:16】没有后缀，只以前缀区分
                "发生在" => "",
            ),
            is_stamp_content,
            // 真值
            truth_brackets: s!("真", "值"), // 大改：兼容单真值、空真值
            truth_separator: s!("、"),
            is_truth_content,
        },
        task: NarseseFormatTask {
            // 预算
            budget_brackets: s!("预", "算"),
            budget_separator: s!("、"),
            is_budget_content,
        },
    }
}

/// 单元测试
#[cfg(test)]
mod tests_enum_narsese {
    use super::*;
    use crate::lexical::tests::_sample_task_ascii;

    /// 测试/原子词项标识符
    #[test]
    fn test_is_atom_identifier() {
        use nar_dev_utils::show;
        show!(is_identifier('a'));
        show!(&FORMAT_ASCII.sentence.stamp_brackets);
    }

    fn test_format(label: &str, format: &NarseseFormat) {
        let task = _sample_task_ascii();
        println!("{label} formatted task: {:#?}", format.format_task(&task));
    }

    #[test]
    fn tests() {
        // * ↓此处必须传入引用而非所有权：使用`lazy_static`定义的就是**常量**，不可能交付所有权
        // * 📝另外，这里使用`lazy_static`定义的常量都实现了[`Deref`]，可以自动「解引用」到需要的类型
        test_format("ASCII", &FORMAT_ASCII);
        test_format("LaTeX", &FORMAT_LATEX);
        test_format("漢文", &FORMAT_HAN);
    }
}
