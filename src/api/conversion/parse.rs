//! 用于统一「解析」的特征定义
//! * 🎯最初用于「语法解析」函数的多态性
//! * ✨借助Rust的类型推导机制，允许将多种函数统一成一个函数
//! * 📄case：字符串 ==(一个函数)=> 多种类型

/// 将[`Self::From`]值解析到指定类型
/// * 🎯用于「方法+泛型参数=多样化实现」
/// * 📌模仿[`str::parse`]
/// * 📌生命周期参数源自对`&str`的支持
/// * 📄【2024-03-20 14:25:05】解析来源目前主要是字符串
/// * 📝【2024-03-20 14:41:43】目前的实现仍有缺点
///   * ❗无法使用`parser.parse::<To>(from)`的形式解析
///   * 🚩目前只能通过显式标注类型`let result: To = parser.parse(from)`
/// * ✅单一实现方式：仅实现`FromParse<From, Parser>`，再于`NarseseFormat`中建立泛型函数`parse<T: FromParse<&str, Self>>`
///   * 🚩【2024-03-20 15:33:28】现在放开生命周期限制
///   * 🎯解开对`Parser`的「不可变引用」限制，允许`ParseState`通过可变引用实现此特征
/// * ❌【2024-03-20 14:55:52】尝试同时搭建`Parse<From, To>`与`FromParse<From, Parser>`特征失败
///   * 📌失败原因1：复杂的生命周期标注（语法冗余）
///   * 📌失败原因2：特征实现兼容性差（批量自动实现影响字符串到数值的正常解析）
pub trait FromParse<From, Parser> {
    /// 解析
    /// * 🚩不再限定一定要是一个「Narsese结果」
    ///   * 📄case：解析多个Narsese，并将结果分装到一个Vec中
    fn from_parse(from: From, parser: Parser) -> Self;
}
