use super::format::*;

/// 通用 ASCII格式
/// * 来源：文档 `NARS ASCII Input.pdf`
/// * 另可参考：<https://github.com/opennars/opennars/wiki/Narsese-Grammar-(Input-Output-Format)>
/// * 可用于打印Narsese的默认形式
pub const FORMAT_ASCII: NarseseFormat<&str> = NarseseFormat {
    space : NarseseFormatSpace {
        parse: " ", // ! 解析时忽略空格
        format_terms: " ", // 格式化时，词项间需要空格（英文如此）
        format_items: " ", // 格式化时，条目间需要空格（英文如此）
    },
    atom: NarseseFormatAtom {
        prefix_word: "",
        prefix_variable_independent: "$",
        prefix_variable_dependent: "#",
        prefix_variable_query: "?",
        prefix_interval: "+",
        prefix_operator: "^",
        prefix_placeholder: "_",
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
        copula_implication_retrospective: "=\\>",
        copula_equivalence_predictive: "</>",
        copula_equivalence_concurrent: "<|>",
        copula_equivalence_retrospective: "<\\>",
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
    // * ASCII格式中，系词等符号并非「标识符字符」，故无需启用「关键字」
    enable_keyword_truncation: false,
};

/// LaTeX扩展
/// * 来源：文档 `NARS ASCII Input.pdf`
/// * 【20230809 10:22:34】注：暂未找到官方格式模板，此仅基于个人观察
/// * 【20230811 0:26:55】不能很好地兼容「二元运算」表达（需要更专业者优化）
pub const FORMAT_LATEX: NarseseFormat<&str> = NarseseFormat {
    space: NarseseFormatSpace {
        parse: " ",        // ! 解析时可跳过空格
        format_terms: " ", // 格式化时，词项间需要分隔（避免代码粘连）
        format_items: " ", // 格式化时，条目间需要分隔（避免代码粘连）
    },
    atom: NarseseFormatAtom {
        prefix_word: "",
        prefix_variable_independent: r"\$",
        prefix_variable_dependent: r"\#",
        prefix_variable_query: "?",
        prefix_interval: "+",
        prefix_operator: r"\Uparrow ",
        prefix_placeholder: r"\diamond ",
    },
    compound: NarseseFormatCompound {
        brackets: (r"\left(", r"\right)"),
        separator: " ",
        brackets_set_extension: (r"\left\{", r"\right\}"), // ! 此中`{` `}`需要转义
        brackets_set_intension: (r"\left[", r"\right]"),
        connecter_intersection_extension: r"\cap ",
        connecter_intersection_intension: r"\cup ",
        connecter_difference_extension: r"\minus ",
        connecter_difference_intension: r"\sim ",
        connecter_product: r"\times ",
        connecter_image_extension: "/",
        connecter_image_intension: r"\backslash ",
        connecter_conjunction: r"\wedge ",
        connecter_disjunction: r"\vee ",
        connecter_negation: r"\neg ",
        connecter_conjunction_sequential: ",",
        connecter_conjunction_parallel: ";",
    },
    statement: NarseseFormatStatement {
        brackets: (r"\left<", r"\right>"),
        copula_inheritance: r"\rightarrow ",
        copula_similarity: r"\leftrightarrow ",
        copula_implication: r"\Rightarrow ",
        copula_equivalence: r"\Leftrightarrow ",
        copula_instance: r"\circ\!\!\!\rightarrow  ",
        copula_property: r"\rightarrow\!\!\!\circ  ",
        copula_instance_property: r"\circ\!\!\!\rightarrow\!\!\!\circ  ",
        copula_implication_predictive: r"/\!\!\!\Rightarrow ",
        copula_implication_concurrent: r"|\!\!\!\Rightarrow ",
        copula_implication_retrospective: r"\backslash\!\!\!\Rightarrow ",
        copula_equivalence_predictive: r"/\!\!\!\Leftrightarrow ",
        copula_equivalence_concurrent: r"|\!\!\!\Leftrightarrow ",
        copula_equivalence_retrospective: r"\backslash\!\!\!\Leftrightarrow ",
    },
    sentence: NarseseFormatSentence {
        punctuation_judgement: ".",
        punctuation_goal: "!",
        punctuation_question: "?",
        punctuation_quest: "¿", // 【20230806 23:46:18】倒问号没有对应的LaTeX。。。
        stamp_brackets: ("", ""), // !【2024-02-25 16:31:38】此处时态没括号。。
        stamp_past: r"\backslash\!\!\!\Rightarrow",
        stamp_present: r"|\!\!\!\Rightarrow",
        stamp_future: r"/\!\!\!\Rightarrow",
        stamp_fixed: "t=", // ? LaTeX语法未知
        truth_brackets: (r"\langle", r"\rangle"),
        truth_separator: ",",
    },
    task: NarseseFormatTask {
        budget_brackets: (r"\$", r"\$"),
        budget_separator: ";",
    },
    // * LaTeX格式中，系词等符号的开头均非「标识符字符」，故无需启用「关键字」
    enable_keyword_truncation: false,
};

/// 漢文扩展
/// * 📌原创
pub const FORMAT_HAN: NarseseFormat<&str> = NarseseFormat {
    space: NarseseFormatSpace {
        parse: " ",       // ! 解析时忽略空格
        format_terms: "", // 格式化时，词项间无需分隔（避免太过松散）
        format_items: " ", // 格式化时，条目间需要分隔（避免太过密集）
    },
    atom: NarseseFormatAtom {
        prefix_word: "", // 置空
        prefix_variable_independent: "任一",
        prefix_variable_dependent: "其一",
        prefix_variable_query: "所问",
        prefix_interval: "间隔",
        prefix_operator: "操作",
        prefix_placeholder: "某",
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
        punctuation_quest: "；", // 暂且没有更合适、更方便输入的全角标点
        stamp_brackets: ("", ""), // !【2024-02-25 16:31:38】此处时态没括号。。
        stamp_past: "过去",
        stamp_present: "现在",
        stamp_future: "将来",
        stamp_fixed: "发生在", // 另一个候选是「时为」，但欠缺可读性
        truth_brackets: ("真", "值"), // 大改：兼容单真值、空真值
        truth_separator: "、",
    },
    task: NarseseFormatTask {
        budget_brackets: ("预", "算"),
        budget_separator: "、",
    },
    // * 漢文则需要：由此可省略空格分隔
    enable_keyword_truncation: true,
};

/// 单元测试
#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;

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
