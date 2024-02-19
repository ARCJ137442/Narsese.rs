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
        stamp_predictive: "/",
        stamp_concurrent: "|",
        stamp_retrospective: "\\",
        stamp_fixed: "!",
        truth_brackets: ("$", "$"),
        truth_separator: ";",
    },
    task: NarseseFormatTask {
        truth_brackets: ("$", "$"),
        truth_separator: ";",
    },
};

/// 单元测试
#[cfg(test)]
#[test]
fn test_ascii() {
    use crate::term::*;
    let format = &FORMAT_ASCII;
    println!("ASCII format: {format:#?}");
    let term = Term::new_inheritance(Term::new_word("A"), Term::new_word("B"));
    let term2 = Term::new_implication(
        Term::new_inheritance(
            Term::new_product(vec![
                Term::new_set_extension(vec![Term::new_word("SELF")]),
                Term::new_variable_independent("any"),
                Term::new_variable_dependent("some"),
            ]),
            Term::new_operator("do"),
        ),
        Term::new_inheritance(
            Term::new_set_extension(vec![Term::new_word("SELF")]),
            Term::new_set_intension(vec![Term::new_word("good")]),
        ),
    );
    println!("ASCII formatted term: {:#?}", format.format_term(&term));
    println!("ASCII formatted term: {:#?}", format.format_term(&term2));
}
