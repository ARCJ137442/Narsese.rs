//! 定义「Narsese格式」的常用实例
//! * 📌最初自「枚举Narsese」拷贝过来
//! * 📌均基于CommonNarsese的语法格式，只是其中的「关键字」不同
//! * 📄部分参考自[JuNarsese](https://github.com/ARCJ137442/JuNarsese.jl)
//!   * ℹ️有少量修改

use super::format::*;
use lazy_static::lazy_static;
use util::{
    prefix_match_dict, prefix_match_dict_pair, PrefixMatch, PrefixMatchDict, PrefixMatchDictPair,
};

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
//     use util::show;
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
    pub static ref FORMAT_ASCII: NarseseFormat<'static> = create_format_ascii();

    /// LaTeX扩展
    /// * 来源：文档 `NARS ASCII Input.pdf`
    /// * 【20230809 10:22:34】注：暂未找到官方格式模板，此仅基于个人观察
    /// * 【20230811 0:26:55】不能很好地兼容「二元运算」表达（需要更专业者优化）
    ///
    /// * 📄使用[`lazy_static`]实现「静态常量」
    ///   * 详请参考[`create_format_ascii`]
    pub static ref FORMAT_LATEX: NarseseFormat<'static> = create_format_latex();

    /// 漢文扩展
    /// * 📌原创
    ///
    /// * 📄使用[`lazy_static`]实现「静态常量」
    ///   * 详请参考[`create_format_ascii`]
    pub static ref FORMAT_HAN: NarseseFormat<'static> = create_format_han();
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
pub fn create_format_ascii<'a>() -> NarseseFormat<'a> {
    NarseseFormat {
        space: NarseseFormatSpace {
            is_for_parse: Box::new(|c: char| c.is_whitespace()), // ! 解析时忽略空格
            format_terms: " ", // 格式化时，词项间需要空格（英文如此）
            format_items: " ", // 格式化时，条目间需要空格（英文如此）
            remove_spaces_before_parse: true, // ASCII版本空格无关
        },
        atom: NarseseFormatAtom {
            // 所有原子词项的前缀
            prefixes: prefix_match_dict!(
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
            is_identifier: Box::new(|c: char| c.is_alphanumeric() || c == '_'),
        },
        compound: NarseseFormatCompound {
            // 外延集/内涵集
            set_brackets: prefix_match_dict_pair!(
                "{" => "}" // 外延集
                "[" => "]" // 内涵集
            ),
            // 普通括号
            brackets: ("(", ")"),
            // 普通分隔符
            separator: ",",
            // 复合词项连接符
            connecters: prefix_match_dict!(
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
            brackets: ("<", ">"),
            // 陈述系词
            copulas: prefix_match_dict!(
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
            punctuations: prefix_match_dict!(
                "." // 判断
                "!" // 目标
                "?" // 问题
                "@" // 请求
            ),
            // 时间戳
            stamp_brackets: (":", ":"),
            // 真值 | 内容已不包含空格
            truth_brackets: ("%", "%"),
            is_truth_content: Box::new(|c: char| matches!(c, '0'..='9' | ';')),
        },
        task: NarseseFormatTask {
            // 预算 | 内容已不包含空格
            budget_brackets: ("$", "$"),
            is_budget_content: Box::new(|c: char| matches!(c, '0'..='9' | ';')),
        },
    }
}

/// LaTeX扩展
/// * 来源：文档 `NARS ASCII Input.pdf`
/// * 【20230809 10:22:34】注：暂未找到官方格式模板，此仅基于个人观察
/// * 【20230811 0:26:55】不能很好地兼容「二元运算」表达（需要更专业者优化）
pub fn create_format_latex<'a>() -> NarseseFormat<'a> {
    NarseseFormat {
        space: NarseseFormatSpace {
            is_for_parse: Box::new(|c| c.is_whitespace()), // ! 解析时可跳过空格
            format_terms: " ", // 格式化时，词项间需要分隔（避免代码粘连）
            format_items: " ", // 格式化时，条目间需要分隔（避免代码粘连）
            remove_spaces_before_parse: true, // LaTeX版本亦可空格无关——通过「后缀空参数」省去空格
        },
        atom: NarseseFormatAtom {
            prefixes: prefix_match_dict!(
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
            is_identifier: Box::new(|c| c.is_alphanumeric() || c == '_'),
        },
        compound: NarseseFormatCompound {
            // 左右括弧
            brackets: (r"\left(", r"\right)"),
            // 以空格作分隔符
            separator: " ",
            // 词项集
            set_brackets: prefix_match_dict_pair!(
                // ! ↓此中`{` `}`需要转义
                r"\left\{" => r"\right\}" // 外延集
                r"\left[" => r"\right]" // 内涵集
            ),
            // 复合词项连接符
            connecters: prefix_match_dict!(
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
            brackets: (r"\left<", r"\right>"),
            copulas: prefix_match_dict!(
                r"\rightarrow{}" // 继承
                r"\leftrightarrow{}" // 相似
                r"\Rightarrow{}" // 蕴含
                r"\Leftrightarrow{}" // 等价
                r"\circ\!\!\!\rightarrow {}" // 实例
                r"\rightarrow\!\!\!\circ {}" // 属性
                r"\circ\!\!\!\rightarrow\!\!\!\circ {}" // 实例属性
                r"/\!\!\!\Rightarrow{}" // 预测性蕴含
                r"|\!\!\!\Rightarrow{}" // 并发性蕴含
                r"\backslash\!\!\!\Rightarrow{}" // 回顾性蕴含
                r"/\!\!\!\Leftrightarrow{}" // 预测性等价
                r"|\!\!\!\Leftrightarrow{}" // 并发性等价
                r"\backslash\!\!\!\Leftrightarrow{}" // 回顾性等价
            ),
        },
        sentence: NarseseFormatSentence {
            // 标点
            punctuations: prefix_match_dict!(
                "." // 判断
                "!" // 目标
                "?" // 问题
                "¿" // 请求
                // ! 💭【20230806 23:46:18】倒问号没有对应的LaTeX。。。
            ),
            // 时间戳
            stamp_brackets: ("", ""), // !【2024-02-25 16:31:38】此处时态没括号。。
            // 真值
            truth_brackets: (r"\langle", r"\rangle"),
            is_truth_content: Box::new(|c: char| matches!(c, '0'..='9' | ';')),
        },
        task: NarseseFormatTask {
            // 预算
            budget_brackets: (r"\$", r"\$"),
            is_budget_content: Box::new(|c: char| matches!(c, '0'..='9' | ';')),
        },
    }
}

/// 漢文扩展
/// * 📌原创
pub fn create_format_han<'a>() -> NarseseFormat<'a> {
    NarseseFormat {
        space: NarseseFormatSpace {
            is_for_parse: Box::new(|c| c.is_whitespace()), // ! 解析时忽略空格
            format_terms: "",  // 格式化时，词项间无需分隔（避免太过松散）
            format_items: " ", // 格式化时，条目间需要分隔（避免太过密集）
            remove_spaces_before_parse: true, // 漢文亦空格无关
        },
        atom: NarseseFormatAtom {
            prefixes: prefix_match_dict!(
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
            is_identifier: Box::new(|c| c.is_alphanumeric() || c == '_'),
        },
        compound: NarseseFormatCompound {
            brackets: ("（", "）"),
            separator: "，",
            set_brackets: prefix_match_dict_pair!(
                "『" => "』" // 外延集
                "【" => "】" // 内涵集
            ),
            // 复合词项连接符
            connecters: prefix_match_dict!(
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
            brackets: ("「", "」"),
            copulas: prefix_match_dict!(
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
            punctuations: prefix_match_dict!(
                "。" // 判断
                "！" // 目标
                "？" // 问题
                "；" // 请求
                // ! 暂且没有更合适、更方便输入的全角标点
            ),
            // 时间戳
            stamp_brackets: ("", ""), // !【2024-02-25 16:31:38】此处时态没括号。。
            // 真值
            truth_brackets: ("真", "值"), // 大改：兼容单真值、空真值
            is_truth_content: Box::new(|c: char| matches!(c, '0'..='9' | '、')), // 此处有特别的分隔符「、」
        },
        task: NarseseFormatTask {
            // 预算
            budget_brackets: ("预", "算"),
            is_budget_content: Box::new(|c: char| matches!(c, '0'..='9' | '、')), // 此处有特别的分隔符「、」
        },
    }
}

/// 单元测试
#[cfg(test)]
mod tests_enum_narsese {

    use super::*;
    use crate::conversion::string::impl_lexical::tests::_sample_task_ascii;

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
