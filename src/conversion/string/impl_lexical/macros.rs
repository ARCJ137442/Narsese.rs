/// 快捷构造「词法Narsese」
/// * 🎯用于快捷（从解析器中）构造Narsese
///   * ✨可直接输入Narsese，并享受Rust的语法高亮
/// * ⚠️一般用于Narsese字面量
///   * **强制`unwrap`解析结果**
///
/// ! 已知问题
/// * ❌输入必须遵循Rust词法：
///   * 📄不能出现非法token：`\` `=\>` `<\>`
///   * 📄不能出现未配对的括弧：`{--` `--]` `{-]`
///
/// # Panics
///
/// ⚠️当所传入的Narsese非法（解析失败）时，将在运行中panic
#[macro_export]
macro_rules! lexical_nse {
    // 对字符串字面量的支持
    (@ARG $narsese:literal) => {
        $narsese
    };
    // 直接内联表达式
    (@ARG $($inlined:tt)*) => {
        stringify!($($inlined)*)
    };
    // 预备解析表达式
    (@PARSE $narsese:expr) => {
        $crate::lexical_nse!(
            // 「解析」子函数
            @PARSE
            // 解析所用的格式
            [$crate::conversion::string::impl_lexical::format_instances::FORMAT_ASCII]
            // 解析的目标类型
            [$crate::lexical::Narsese]
            // 被解析的表达式（实际上是字面量）
            $narsese
        )
    };
    // 主解析规则
    (@PARSE [$format:expr] [$target:ty] $narsese:expr) => {
        {
            // 直接调用模块内部的解析方法
            // 🚩【2024-03-23 17:25:58】没有性能trick
            // ✅无需指定目标类型：根目录已经指定了`ParseResult`
            $crate::conversion::string::impl_lexical::parser::parse(
                // 格式的引用
                &$format,
                // 要解析的Narsese
                $narsese
            ).unwrap()
        }
    };
    // * 兜底总入口
    // * ❌【2024-03-23 16:35:59】不再尝试兼容其它语法，专精兼容ASCII版本
    // * 📌↑此举亦有可能造成解析歧义
    (/* [$($variant:tt)*]  */$($tail:tt)*) => {
        $crate::lexical_nse!(
            // 「解析」子函数
            @PARSE
            // 解析的参数
            $crate::lexical_nse!(@ARG $($tail)*)
        )
    };
}

/// 单元测试
#[cfg(test)]
mod tests {
    use crate::{
        conversion::string::impl_lexical::format_instances::*,
        lexical::{Narsese, Sentence, Term},
        lexical_nse as nse,
    };
    use util::*;

    /// 测试：快捷构造
    #[test]
    fn test_construct() {
        // 简单case
        let nse_str = "<A --> B>.";
        let nse = nse!(<A --> B>.);
        asserts! {
            // 测试是否等效
            dbg!(&nse) => &FORMAT_ASCII.parse(nse_str).unwrap(),
            // 匹配内部结构
            nse => @ Narsese::Sentence(..),
            nse => @ Narsese::Sentence(Sentence{..}),
            nse => @ Narsese::Sentence(Sentence{term: Term::Statement { .. }, ..}),
        };

        // 复杂case
        let nse_str = "$0.5;0.75;0.4$ <(&/, <{ball} --> [left]>, <(*, {SELF}, $any, #some) --> ^do>) ==> <{SELF} --> [good]>>. :!-1: %1.0;0.9%";
        let nse_s = nse!("$0.5;0.75;0.4$ <(&/, <{ball} --> [left]>, <(*, {SELF}, $any, #some) --> ^do>) ==> <{SELF} --> [good]>>. :!-1: %1.0;0.9%");
        let nse = nse!($0.5;0.75;0.4$ <(&/, <{ball} --> [left]>, <(*, {SELF}, $any, #some) --> ^do>) ==> <{SELF} --> [good]>>. :!-1: %1.0;0.9%);
        asserts! {
            // 测试是否等效
            dbg!(&nse) => &FORMAT_ASCII.parse(nse_str).unwrap(),
            dbg!(&nse_s) => &nse,
        }
    }
}
