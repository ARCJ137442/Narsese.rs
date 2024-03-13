//! Narsese格式化中的「字符串格式模板」函数
//! * 📌重在**纯字符串**处理：与「具体实现的NarseseFormat」格式无关
//! * 🎯统一提取出通用的「字符串格式化模板」模块

use util::{join_lest_multiple_separators, push_str};

/// 模板/原子词项：前缀+名称
/// * 🎯所有Narsese原子词项类型
/// * 📝仅使用`pub(super)`即可在mod内共用，但为后续复用扩展，仍然使用`pub`对crate外开放
pub fn template_atom(out: &mut String, prefix: &str, name: &str) {
    push_str!(out; prefix, name);
}

/// 模板/系列词项
/// * 🎯一般复合词项，词项集（外延集/内涵集）
/// * 📝对于「字符串自面量数组」，`Vec<&str>`的引用类型对应`&[str]`而非`&[&str]`
///   * ❓亦或两者皆可
pub fn template_components(
    out: &mut String,
    components: impl Iterator<Item = String>,
    separator: &str,
    space: &str,
) {
    for (i, term_str) in components.enumerate() {
        // 逗号
        if i != 0 {
            push_str!(out; separator, space);
        }
        // 词项
        out.push_str(&term_str);
    }
}

/// 模板/一般复合词项
/// * 🎯使用「连接符」区分「复合类型」的词项
/// * 📝对于「字符串自面量数组」，`Vec<&str>`的引用类型对应`&[&str]`而非`&[str]`
///   * ⚠️后者的`str`是大小不定的：the size for values of type `str` cannot be known at compilation time
pub fn template_compound(
    out: &mut String,
    left_bracket: &str,
    connecter: &str,
    components: impl Iterator<Item = String>,
    separator: &str,
    space: &str,
    right_bracket: &str,
) {
    // 左括号&连接符
    push_str!(out;
        // 左括号 `(`
        left_bracket,
        // 连接符 | `&&, `
        connecter, separator, space,
    );
    // 组分 | `A, B, C`
    template_components(out, components, separator, space);
    // 右括号 | `)`
    out.push_str(right_bracket);
}

/// 模板/集合复合词项
/// * 🎯「外延集/内涵集」这样【无需特定连接符，只需特殊括弧区分】的词项
pub fn template_compound_set(
    out: &mut String,
    left_bracket: &str,
    components: impl Iterator<Item = String>,
    separator: &str,
    space: &str,
    right_bracket: &str,
) {
    // 左括号 | `{`
    out.push_str(left_bracket);
    // 组分 | `A, B, C`
    template_components(out, components, separator, space);
    // 右括号 | `}`
    out.push_str(right_bracket);
}

/// 模板/陈述
/// * 🎯各类作为陈述的词项
pub fn template_statement(
    out: &mut String,
    left_bracket: &str,
    subject: &str,
    copula: &str,
    predicate: &str,
    space: &str,
    right_bracket: &str,
) {
    push_str!(out;
        left_bracket, // `<`
        subject, // `S`
        space, copula, space, // ` --> `
        predicate, // `P`
        right_bracket, // `>`
    );
}

/// 模板/语句
/// * 🎯词项+标点+时间戳+真值
pub fn template_sentence(
    out: &mut String,
    term: &str,
    punctuation: &str,
    stamp: &str,
    truth: &str,
    separator: &str,
) {
    // 词项直接输入，后续紧跟标点
    out.push_str(term);
    // 后续顺序拼接，并避免多余分隔符
    join_lest_multiple_separators(out, [punctuation, stamp, truth].into_iter(), separator)
}
