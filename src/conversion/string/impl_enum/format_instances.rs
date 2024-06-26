//! 定义「Narsese格式」的常用实例
//! * 📌均基于CommonNarsese的语法格式，只是其中的「关键字」不同
//! * 📄部分参考自[JuNarsese](https://github.com/ARCJ137442/JuNarsese.jl)
//!   * ℹ️有少量修改

use super::format::*;

/// 工具函数/判断字符是否能作为「词项名」
/// * 🎯用于判断「合法词项名」
/// * ⚠️【2024-06-13 19:25:15】[`char::is_alphanumeric`]目前还不是常量函数
fn is_valid_atom_name(c: char) -> bool {
    //  先判断是否为「字母/数字」
    c.is_alphanumeric()
    // 特殊：横杠/下划线
    // //! ↓【2024-02-22 14:46:16】现因需兼顾`<主词-->谓词>`的结构（防止系词中的`-`被消耗），故不再兼容`-`
    // * 🚩【2024-03-28 14:18:08】现在重新启用对`-`的「原子词项字符兼容」：使用新的「前缀failing匹配」方法
    || c == '_' || c == '-'
    // 常见emoji兼容
    || c > '\u{1f2ff}'
}

/// 通用 ASCII格式
/// * 来源：文档 `NARS ASCII Input.pdf`
/// * 另可参考：<https://github.com/opennars/opennars/wiki/Narsese-Grammar-(Input-Output-Format)>
/// * 可用于打印Narsese的默认形式
pub const FORMAT_ASCII: NarseseFormat<&str> = NarseseFormat {
    is_valid_atom_name,
    space: NarseseFormatSpace {
        parse: " ",        // ! 解析时忽略空格
        format_terms: " ", // 格式化时，词项间需要空格（英文如此）
        format_items: " ", // 格式化时，条目间需要空格（英文如此）
    },
    atom: NarseseFormatAtom {
        prefix_word: "",
        prefix_placeholder: "_",
        prefix_variable_independent: "$",
        prefix_variable_dependent: "#",
        prefix_variable_query: "?",
        prefix_interval: "+",
        prefix_operator: "^",
    },
    compound: NarseseFormatCompound {
        brackets: ("(", ")"),
        separator: ",",
        brackets_set_extension: ("{", "}"),
        brackets_set_intension: ("[", "]"),
        connecter_intersection_extension: "&",
        connecter_intersection_intension: "|",
        connecter_difference_extension: "-",
        connecter_difference_intension: "~",
        connecter_product: "*",
        connecter_image_extension: "/",
        connecter_image_intension: r"\",
        connecter_conjunction: "&&",
        connecter_disjunction: "||",
        connecter_negation: "--",
        connecter_conjunction_sequential: "&/",
        connecter_conjunction_parallel: "&|",
    },
    statement: NarseseFormatStatement {
        brackets: ("<", ">"),
        copula_inheritance: "-->",
        copula_similarity: "<->",
        copula_implication: "==>",
        copula_equivalence: "<=>",
        copula_instance: "{--",
        copula_property: "--]",
        copula_instance_property: "{-]",
        copula_implication_predictive: "=/>",
        copula_implication_concurrent: "=|>",
        copula_implication_retrospective: r"=\>",
        copula_equivalence_predictive: "</>",
        copula_equivalence_concurrent: "<|>",
        copula_equivalence_retrospective: r"<\>",
    },
    sentence: NarseseFormatSentence {
        punctuation_judgement: ".",
        punctuation_goal: "!",
        punctuation_question: "?",
        punctuation_quest: "@",
        stamp_brackets: (":", ":"),
        stamp_past: r"\",
        stamp_present: "|",
        stamp_future: "/",
        stamp_fixed: "!",
        truth_brackets: ("%", "%"),
        truth_separator: ";",
    },
    task: NarseseFormatTask {
        budget_brackets: ("$", "$"),
        budget_separator: ";",
    },
    // * 🚩【2024-03-28 14:33:47】现弃用「关键字截断」机制，直接使用「系词前缀匹配」判断
};

/// LaTeX扩展
/// * 来源：文档 `NARS ASCII Input.pdf`
/// * 【20230809 10:22:34】注：暂未找到官方格式模板，此仅基于个人观察
/// * 【20230811 0:26:55】不能很好地兼容「二元运算」表达（需要更专业者优化）
/// * 🆕更新@2024-04-05：时序系词与时态由「前缀竖杠」变为「中缀竖杠」
pub const FORMAT_LATEX: NarseseFormat<&str> = NarseseFormat {
    is_valid_atom_name,
    space: NarseseFormatSpace {
        parse: " ",        // ! 解析时可跳过空格
        format_terms: " ", // 格式化时，词项间需要分隔（避免代码粘连）
        format_items: " ", // 格式化时，条目间需要分隔（避免代码粘连）
    },
    atom: NarseseFormatAtom {
        prefix_word: "",
        prefix_placeholder: r"\diamond{}",
        prefix_variable_independent: r"\$",
        prefix_variable_dependent: r"\#",
        prefix_variable_query: "?",
        prefix_interval: "+",
        prefix_operator: r"\Uparrow{}",
    },
    compound: NarseseFormatCompound {
        brackets: (r"\left(", r"\right)"),
        separator: r"\;", // ! 【2024-03-18 23:55:17】LaTeX使用`\space{}`也可使用`\;` | ✅兼容MathJax
        brackets_set_extension: (r"\left\{", r"\right\}"), // ! 此中`{` `}`需要转义
        brackets_set_intension: (r"\left[", r"\right]"),
        connecter_intersection_extension: r"\cap{}",
        connecter_intersection_intension: r"\cup{}",
        connecter_difference_extension: r"\minus{}",
        connecter_difference_intension: r"\sim{}",
        connecter_product: r"\times{}",
        connecter_image_extension: "/",
        connecter_image_intension: r"\backslash{}",
        connecter_conjunction: r"\wedge{}",
        connecter_disjunction: r"\vee{}",
        connecter_negation: r"\neg{}",
        connecter_conjunction_sequential: ",",
        connecter_conjunction_parallel: ";",
    },
    statement: NarseseFormatStatement {
        brackets: (r"\left<", r"\right>"),
        // ! 【2024-03-18 23:53:37】↓现在由于格式化时自动添加的空格，故此处不尾缀空格也能进入MathJax
        // * 🚩同步自「词法Narsese」
        copula_inheritance: r"\rightarrow{}",
        copula_similarity: r"\leftrightarrow{}",
        copula_implication: r"\Rightarrow{}",
        copula_equivalence: r"\Leftrightarrow{}",
        copula_instance: r"\circ\!\!\!\rightarrow{}",
        copula_property: r"\rightarrow\!\!\!\circ{}",
        copula_instance_property: r"\circ\!\!\!\rightarrow\!\!\!\circ{}",
        copula_implication_predictive: r"/\!\!\!\!\!\Rightarrow{}",
        copula_implication_concurrent: r"|\!\!\!\!\!\Rightarrow{}",
        copula_implication_retrospective: r"\backslash\!\!\!\!\!\Rightarrow{}",
        copula_equivalence_predictive: r"/\!\!\!\Leftrightarrow{}",
        copula_equivalence_concurrent: r"|\!\!\!\Leftrightarrow{}",
        copula_equivalence_retrospective: r"\backslash\!\!\!\Leftrightarrow{}",
    },
    sentence: NarseseFormatSentence {
        punctuation_judgement: ".",
        punctuation_goal: "!",
        punctuation_question: "?",
        punctuation_quest: "¿", // 【20230806 23:46:18】倒问号没有对应的LaTeX。。。
        stamp_brackets: ("", ""), // !【2024-02-25 16:31:38】此处时态没括号。。
        stamp_past: r"\backslash\!\!\!\!\!\Rightarrow{}",
        stamp_present: r"|\!\!\!\!\!\Rightarrow{}",
        stamp_future: r"/\!\!\!\!\!\Rightarrow{}",
        stamp_fixed: "t=",                            // ? LaTeX语法未知
        truth_brackets: (r"\langle{}", r"\rangle{}"), // ! 【2024-03-18 23:58:02】末尾使用空参数集分隔
        truth_separator: ",",
    },
    task: NarseseFormatTask {
        budget_brackets: (r"\$", r"\$"),
        budget_separator: ";",
    },
    // * 🚩【2024-03-28 14:33:47】现弃用「关键字截断」机制，直接使用「系词前缀匹配」判断
};

/// 漢文扩展
/// * 📌原创
pub const FORMAT_HAN: NarseseFormat<&str> = NarseseFormat {
    is_valid_atom_name,
    space: NarseseFormatSpace {
        parse: " ",        // ! 解析时忽略空格
        format_terms: "",  // 格式化时，词项间无需分隔（避免太过松散）
        format_items: " ", // 格式化时，条目间需要分隔（避免太过密集）
    },
    atom: NarseseFormatAtom {
        prefix_word: "", // 置空
        prefix_placeholder: "某",
        prefix_variable_independent: "任一",
        prefix_variable_dependent: "其一",
        prefix_variable_query: "所问",
        prefix_interval: "间隔",
        prefix_operator: "操作",
    },
    compound: NarseseFormatCompound {
        brackets: ("（", "）"),
        separator: "，",
        brackets_set_extension: ("『", "』"),
        brackets_set_intension: ("【", "】"),
        connecter_intersection_extension: "外交",
        connecter_intersection_intension: "内交",
        connecter_difference_extension: "外差",
        connecter_difference_intension: "内差",
        connecter_product: "积",
        connecter_image_extension: "外像",
        connecter_image_intension: "内像",
        connecter_conjunction: "与",
        connecter_disjunction: "或",
        connecter_negation: "非",
        connecter_conjunction_sequential: "接连",
        connecter_conjunction_parallel: "同时",
    },
    statement: NarseseFormatStatement {
        brackets: ("「", "」"),
        copula_inheritance: "是",
        copula_similarity: "似",
        copula_implication: "得",
        copula_equivalence: "同",
        copula_instance: "为",
        copula_property: "有",
        copula_instance_property: "具有",
        copula_implication_predictive: "将得",
        copula_implication_concurrent: "现得",
        copula_implication_retrospective: "曾得",
        copula_equivalence_predictive: "将同",
        copula_equivalence_concurrent: "现同",
        copula_equivalence_retrospective: "曾同",
    },
    sentence: NarseseFormatSentence {
        punctuation_judgement: "。",
        punctuation_goal: "！",
        punctuation_question: "？",
        punctuation_quest: "；",  // 暂且没有更合适、更方便输入的全角标点
        stamp_brackets: ("", ""), // !【2024-02-25 16:31:38】此处时态没括号。。
        stamp_past: "过去",
        stamp_present: "现在",
        stamp_future: "将来",
        stamp_fixed: "发生在",        // 另一个候选是「时为」，但欠缺可读性
        truth_brackets: ("真", "值"), // 大改：兼容单真值、空真值
        truth_separator: "、",
    },
    task: NarseseFormatTask {
        budget_brackets: ("预", "算"),
        budget_separator: "、",
    },
    // * 🚩【2024-03-28 14:33:47】现弃用「关键字截断」机制，直接使用「系词前缀匹配」判断
};

// ! ❌有关Typst的尝试失败：其原子词项需要包括引号，但目前「词项前缀」的模型无法满足此要求

/// 单元测试
#[cfg(test)]
#[cfg(feature = "enum_narsese")]
mod tests_enum_narsese {

    use super::*;
    use crate::enum_narsese::*;

    fn test_format(label: &str, format: NarseseFormat<&str>) {
        // 展示格式
        println!("{label} format: {format:#?}");
        // 构造词项
        let ball_left = Term::new_instance_property(Term::new_word("ball"), Term::new_word("left"));
        let conditional_operation = Term::new_conjunction_sequential(vec![
            ball_left.clone(),
            Term::new_inheritance(
                Term::new_product(vec![
                    Term::new_set_extension(vec![Term::new_word("SELF")]),
                    Term::new_variable_independent("any"),
                    Term::new_variable_dependent("some"),
                ]),
                Term::new_operator("do"),
            ),
        ]);
        let self_good = Term::new_instance_property(Term::new_word("SELF"), Term::new_word("good"));
        let term = Term::new_implication(conditional_operation.clone(), self_good.clone());
        // 构造语句
        let truth = Truth::Double(1.0, 0.9);
        let stamp = Stamp::Fixed(-1);
        let sentence = Sentence::new_judgement(term.clone(), truth, stamp);
        // 构造任务
        let budget = Budget::Triple(0.5, 0.75, 0.4);
        let task = Task::new(sentence.clone(), budget);
        // 展示
        println!(
            "{label} formatted term: {:#?}",
            format.format_term(&self_good)
        );
        println!(
            "{label} formatted sentence: {:#?}",
            format.format_sentence(&sentence)
        );
        println!("{label} formatted task: {:#?}", format.format_task(&task));
    }

    #[test]
    fn tests() {
        test_format("ASCII", FORMAT_ASCII);
        test_format("LaTeX", FORMAT_LATEX);
        test_format("漢文", FORMAT_HAN);
    }
}
