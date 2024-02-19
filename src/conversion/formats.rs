use super::formatter::*;

/// ASCII格式
pub const FORMAT_ASCII: NarseseFormat<&str> = NarseseFormat {
    space: " ",

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
        connector_intersection_intension: "|",
        connecter_difference_extension: "-",
        connecter_difference_intension: "~",
        connecter_product: "*",
        connecter_image_extension: "/",
        connecter_image_intension: "\\",
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
        copula_predictive_implication: "=/>",
        copula_concurrent_implication: "=|>",
        copula_retrospective_implication: "=\\>",
        copula_predictive_equivalence: "</>",
        copula_concurrent_equivalence: "<|>",
        copula_retrospective_equivalence: "<\\>",
    },
    sentence: NarseseFormatSentence {
        punctuation_judgement: ".",
        punctuation_goal: "!",
        punctuation_question: "?",
        punctuation_quest: "@",
        stamp_brackets: (":", ":"),
        stamp_future: "/",
        stamp_present: "|",
        stamp_past: "\\",
        stamp_fixed: "!",
        truth_brackets: ("$", "$"),
        truth_separator: ";",
    },
    task: NarseseFormatTask {
        budget_brackets: ("$", "$"),
        budget_separator: ";",
    },
};

/// 单元测试
#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;

    fn test_format(format: NarseseFormat<&str>) {
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
            "ASCII formatted term: {:#?}",
            format.format_term(&self_good)
        );
        println!(
            "ASCII formatted sentence: {:#?}",
            format.format_sentence(&sentence)
        );
        println!("ASCII formatted task: {:#?}", format.format_task(&task));
    }

    #[test]
    fn test_ascii() {
        let format = FORMAT_ASCII;
        println!("ASCII format: {format:#?}");
        test_format(format);
    }
}
