use crate::push_str;

/// 工具函数/有内容时前缀分隔符
/// * 🎯最初用于「多个用空格分隔的条目」中「若其中有空字串，就无需连续空格」的情况
/// * 关键在「避免无用分隔符」
pub fn add_space_if_necessary_and_flush_buffer(
    out: &mut String,
    buffer: &mut String,
    separator: &str,
) {
    match buffer.is_empty() {
        // 空⇒不做动作
        true => {}
        // 非空⇒预置分隔符，推送并清空
        false => {
            push_str!(out; separator, buffer);
            buffer.clear();
        }
    }
}

/// 工具函数/用分隔符拼接字符串，且当元素为空时避免连续分隔符
/// * 🎯最初用于「多个用空格分隔的条目」中「若其中有空字串，就无需连续空格」的情况
/// * 📌实际上是[`add_space_if_necessary_and_flush_buffer`]的另一种形式
///
/// # Example
/// ```rust
/// use enum_narsese::util::join_lest_multiple_separators;
/// let mut s = String::new();
/// join_lest_multiple_separators(&mut s, vec!["a", "", "b", "c", "", "d"].into_iter(), ",");
/// assert_eq!(s, "a,b,c,d");
/// ```
pub fn join_lest_multiple_separators<'a, I>(out: &mut String, mut elements: I, separator: &str)
where
    I: Iterator<Item = &'a str>,
{
    // 先加入第一个元素
    match elements.next() {
        // 有元素⇒直接加入
        Some(s) => out.push_str(s),
        // 无元素⇒直接返回
        None => return,
    };
    // 其后「先考虑分隔，再添加元素」
    for element in elements {
        match element.is_empty() {
            // 空字串⇒没必要添加
            true => continue,
            // 非空字串⇒连同分隔符一并添加
            false => push_str!(out; separator, element),
        }
    }
}
