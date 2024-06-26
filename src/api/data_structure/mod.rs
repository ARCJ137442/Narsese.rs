//! 用于在API层面定义与Narsese（或亦有NAL一部分）的数据结构
//! * 🎯与「词项」有关的共用API
//!   * 📄词项的「类别」「容量」等概念
//! * 🎯与「真值/预算值」有关的共用API
//!   * 📄其通用的「证据值」特征
//!     * 🚩【2024-04-16 18:46:06】后续可以用来泛化「真值函数」

nar_dev_utils::pub_mod_and_pub_use! {
    // Narsese值
    narsese_value
    // 部分初始化的Narsese值
    narsese_options
    // 词项
    term
    // 证据值
    evidence_value
}
