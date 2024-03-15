//! 定义「Narsese格式」的常用实例
//! * 📌最初自「枚举Narsese」拷贝过来
//! * 📌均基于CommonNarsese的语法格式，只是其中的「关键字」不同
//! * 📄部分参考自[JuNarsese](https://github.com/ARCJ137442/JuNarsese.jl)
//!   * ℹ️有少量修改

use util::{
    prefix_match_dict, prefix_match_dict_pair, PrefixMatch, PrefixMatchDict, PrefixMatchDictPair,
};

use super::format::*;

// 📝有关「全局常量」定义，闭包↔死局 //
// 这里不可以：`Box::new`并非常量函数
// pub const CLJ: Box<dyn Fn(char)> = Box::new(|_c: char| {});
// 这里也不可以 | `static`需要线程安全，而**闭包没法线程安全**
// pub static CLJ: Box<dyn Fn(char)> = Box::new(|_c: char| {});
// 即便外边是Arc，里边也不可以 | `static`需要线程安全，而**闭包没法线程安全**
// pub static CLJ: std::sync::Arc<dyn Fn(char)> = std::sync::Arc::new(|_c: char| {});

/// 通用 ASCII格式
/// * 来源：文档 `NARS ASCII Input.pdf`
/// * 另可参考：<https://github.com/opennars/opennars/wiki/Narsese-Grammar-(Input-Output-Format)>
/// * 可用于打印Narsese的默认形式
/// * 🚩【2024-03-15 18:00:32】目前没法使用`const` `static`，只能使用函数【按需创建】格式
///   * ❌无法将其作为一个常量使用，即便其根本不会变化
///   * ❌使用`const`的方法行不通：包裹闭包的智能指针[`Box`]、[`Rc`]均无法作为常量初始化
///   * ❌使用`static`的方法行不通：闭包无法保证线程安全
pub fn format_ascii<'a>() -> NarseseFormat<'a> {
    NarseseFormat {
        space: NarseseFormatSpace {
            parse: Box::new(|c: char| c.is_whitespace()), // ! 解析时忽略空格
            format_terms: " ",                            // 格式化时，词项间需要空格（英文如此）
            format_items: " ",                            // 格式化时，条目间需要空格（英文如此）
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
            brackets: ("<", ">"),
            copulas: prefix_match_dict!(
                "-->" // 继承
                "<->" // 相似
                "==>" // 蕴含
                "<=>" // 等价
                "{--" // 实例
                "--]" // 属性
                "{-]" // 实例属性
                r"=/>" // 预测蕴含
                r"=|>" // 并发蕴含
                r"=\>" // 回顾蕴含
                r"</>" // 预测等价
                r"<|>" // 并发等价
                r"<\>" // 回顾等价
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
            // 真值
            truth_brackets: ("%", "%"),
            // 时间戳
            stamp_brackets: (":", ":"),
        },
        task: NarseseFormatTask {
            // 预算
            budget_brackets: ("$", "$"),
        },
    }
}

/// LaTeX扩展
/// * 来源：文档 `NARS ASCII Input.pdf`
/// * 【20230809 10:22:34】注：暂未找到官方格式模板，此仅基于个人观察
/// * 【20230811 0:26:55】不能很好地兼容「二元运算」表达（需要更专业者优化）
pub fn format_latex<'a>() -> NarseseFormat<'a> {
    NarseseFormat {
        space: NarseseFormatSpace {
            parse: Box::new(|c| c.is_whitespace()), // ! 解析时可跳过空格
            format_terms: " ",                      // 格式化时，词项间需要分隔（避免代码粘连）
            format_items: " ",                      // 格式化时，条目间需要分隔（避免代码粘连）
        },
        atom: NarseseFormatAtom {
            prefixes: prefix_match_dict!(
                ""
                // 占位符
                r"\diamond "
                // 变量
                r"\$" r"\#" "?"
                // 间隔
                "+"
                // 操作符
                r"\Uparrow "
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
                r"\left\{" => r"\right\}" // ! 此中`{` `}`需要转义
                r"\left[" => r"\right]"
            ),
            // 复合词项连接符
            connecters: prefix_match_dict!(
                r"\cap "
                r"\cup "
                r"\minus "
                r"\sim "
                r"\times "
                "/"
                r"\backslash "
                r"\wedge "
                r"\vee "
                r"\neg "
                ","
                ";"
            ),
        },
        statement: NarseseFormatStatement {
            brackets: (r"\left<", r"\right>"),
            copulas: prefix_match_dict!(
                r"\rightarrow "
                r"\leftrightarrow "
                r"\Rightarrow "
                r"\Leftrightarrow "
                r"\circ\!\!\!\rightarrow  "
                r"\rightarrow\!\!\!\circ  "
                r"\circ\!\!\!\rightarrow\!\!\!\circ  "
                r"/\!\!\!\Rightarrow "
                r"|\!\!\!\Rightarrow "
                r"\backslash\!\!\!\Rightarrow "
                r"/\!\!\!\Leftrightarrow "
                r"|\!\!\!\Leftrightarrow "
                r"\backslash\!\!\!\Leftrightarrow "
            ),
        },
        sentence: NarseseFormatSentence {
            punctuations: prefix_match_dict!(
                "."
                "!"
                "?"
                "¿" // 【20230806 23:46:18】倒问号没有对应的LaTeX。。。
            ),
            stamp_brackets: ("", ""), // !【2024-02-25 16:31:38】此处时态没括号。。
            truth_brackets: (r"\langle", r"\rangle"),
        },
        task: NarseseFormatTask {
            budget_brackets: (r"\$", r"\$"),
        },
    }
}

/// 漢文扩展
/// * 📌原创
pub fn format_han<'a>() -> NarseseFormat<'a> {
    NarseseFormat {
        space: NarseseFormatSpace {
            parse: Box::new(|c| c.is_whitespace()), // ! 解析时忽略空格
            format_terms: "",                       // 格式化时，词项间无需分隔（避免太过松散）
            format_items: " ",                      // 格式化时，条目间需要分隔（避免太过密集）
        },
        atom: NarseseFormatAtom {
            prefixes: prefix_match_dict!(
                "" // 置空
                "任一"
                "其一"
                "所问"
                "间隔"
                "操作"
                "某"
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
                "外交"
                "内交"
                "外差"
                "内差"
                "积"
                "外像"
                "内像"
                "与"
                "或"
                "非"
                "接连"
                "同时"
            ),
        },
        statement: NarseseFormatStatement {
            brackets: ("「", "」"),
            copulas: prefix_match_dict!(
                "是"
                "似"
                "得"
                "同"
                "为"
                "有"
                "具有"
                "将得"
                "现得"
                "曾得"
                "将同"
                "现同"
                "曾同"
            ),
        },
        sentence: NarseseFormatSentence {
            punctuations: prefix_match_dict!(
                "，"
                "。"
                "！"
                "？"
                "；"  // 暂且没有更合适、更方便输入的全角标点
            ),
            stamp_brackets: ("", ""), // !【2024-02-25 16:31:38】此处时态没括号。。
            truth_brackets: ("真", "值"), // 大改：兼容单真值、空真值
        },
        task: NarseseFormatTask {
            budget_brackets: ("预", "算"),
        },
    }
}

/// 单元测试
#[cfg(test)]
mod tests_enum_narsese {

    use super::*;
    use crate::conversion::string::impl_lexical::tests::_sample_task_ascii;

    fn test_format(label: &str, format: NarseseFormat) {
        let task = _sample_task_ascii();
        println!("{label} formatted task: {:#?}", format.format_task(&task));
    }

    #[test]
    fn tests() {
        test_format("ASCII", format_ascii());
        test_format("LaTeX", format_latex());
        test_format("漢文", format_han());
    }
}
