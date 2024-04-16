//! 统一放置全部有关「词项」的抽象接口
//! * 🎯抽象Narsese的内部结构
//!   * 📄功能「提取内部词项」
//! * 🎯抽象Narsese的属性、内容、含义
//!   * 📄概念「词项类别」「词项容量」
//! * ⚠️不一定要求所有版本Narsese都实现
//!   * 📌不同版Narsese的实现不一样，要具体考虑其中的抽象程度

util::pub_mod_and_pub_use! {
    // API「提取词项中的元素」
    extract_terms
    // API「词项类别」
    term_category
    // API「词项容量」
    term_capacity
}
