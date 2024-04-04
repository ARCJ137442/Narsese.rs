//! 用于统一「格式化」的特征定义
//! * 🎯最初用于「格式化」函数的多态性
//! * ✨借助Rust的类型推导机制，允许将多种函数统一成一个函数
//! * 📄case：字符串 ==(一个函数)=> 多种类型

/// 格式化（Narsese类型值）到指定类型
/// * 🎯用于「方法+泛型参数=多样化实现」
/// * 📄【2024-04-05 01:49:16】格式化目标目前主要为字符串[`String`]
/// * ✅单一实现方式：先对各Narsese类型实现`FormatTo<Formatter>`，再于`NarseseFormat`中建立泛型函数`format<T: FormatTo<Self, String>>`
pub trait FormatTo<Formatter, Target = String> {
    /// 格式化到目标
    /// * 📌此中「目标」一般是字符串
    /// * 🚩不再限定一定要是一个「Narsese结果」
    ///   * 📄case：允许格式化多个Narsese，并将结果分装到一个Vec中
    fn format_to(&self, formatter: Formatter) -> Target;
}
