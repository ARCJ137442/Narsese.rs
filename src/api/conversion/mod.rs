//! 统一放置有关「转换」的API
//! * 🎯抽象Narsese的转换相关功能
//!   * 📌转换的结构、过程、结果
//!   * 📌字符串互转（解析、格式化）
//!   * 📌数据结构之间的互转
//!     * 📄词项⇄语句⇄任务

util::pub_mod_and_pub_use! {
    // API「解析」
    parse
    // API「格式化」
    format
    // API「获取内部元素」
    get_inner
    // 语句转换 | 语句-任务 兼容
    sentence_cast
    // 对「Narsese值」的实现
    impl_narsese_value
}
