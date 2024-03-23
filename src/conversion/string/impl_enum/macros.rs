/// 快捷构造「枚举Narsese」
/// * 🎯用于快捷（从解析器中）构造Narsese
/// * ⚠️一般用于Narsese字面量
///   * **强制`unwrap`解析结果**
///
/// ! 已知问题
/// * ❌输入必须遵循Rust词法：不能出现未配对的括弧
///   * 📄无法输入的语法元素：`{--` `--]` `{-]`
#[macro_export]
macro_rules! enum_nse {
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
        $crate::enum_nse!(
            // 「解析」子函数
            @PARSE
            // 解析所用的格式
            [$crate::conversion::string::impl_enum::format_instances::FORMAT_ASCII]
            // 解析的目标类型
            [$crate::enum_narsese::Narsese]
            // 被解析的表达式（实际上是字面量）
            $narsese
        )
    };
    // 主解析规则
    (@PARSE [$format:expr] [$target:ty] $narsese:expr) => {
        {
            // 去掉空格的字符数组
            let narsese_chars = $narsese
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<_>>();
            // 直接作为「解析环境」构建「解析状态」
            // * ⚡无需再构造`String`对象，享受性能提升
            let mut state =
                $crate
                ::conversion::string::impl_enum::ParseState
                ::from_env(
                    &$format,
                    narsese_chars,
                    0
                );
            // 向指定目标进行解析
            state.parse::<$target>().unwrap()
        }
    };
    // * 兜底总入口
    // * ❌【2024-03-23 16:35:59】不再尝试兼容其它语法，专精兼容ASCII版本
    // * 📌↑此举亦有可能造成解析歧义
    (/* [$($variant:tt)*]  */$($tail:tt)*) => {
        $crate::enum_nse!(
            // 「解析」子函数
            @PARSE
            // 解析的参数
            $crate::enum_nse!(@ARG $($tail)*)
        )
    };
}

/// 专用/内联的Narsese词项
/// * 🚩在调用[`enum_nse`]解析后，调用`try_into_term`并随即`unwrap`
/// * ⚠️若解析或转换失败，将发生运行时panic
#[macro_export]
macro_rules! enum_nse_term {
    ($($t:tt)*) => {
        $crate::enum_nse!($($t)*).try_into_term().unwrap()
    };
}

/// 专用/内联的Narsese语句
/// * 🚩在调用[`enum_nse`]解析后，调用`try_into_sentence`并随即`unwrap`
/// * ⚠️若解析或转换失败，将发生运行时panic
#[macro_export]
macro_rules! enum_nse_sentence {
    ($($t:tt)*) => {
        $crate::enum_nse!($($t)*).try_into_sentence().unwrap()
    };
}

/// 专用/内联的Narsese任务
/// * 🚩在调用[`enum_nse`]解析后，调用`try_into_task_compatible`并随即`unwrap`
///   * ✨即便解析出来的是「语句」类型，也会进行自动转换
/// * ⚠️若解析或转换失败，将发生运行时panic
#[macro_export]
macro_rules! enum_nse_task {
    ($($t:tt)*) => {
        $crate::enum_nse!($($t)*).try_into_task_compatible().unwrap()
    };
}

/// 单元测试
#[cfg(test)]
mod tests {
    use crate::{
        conversion::string::impl_enum::format_instances::*,
        enum_narsese::{Narsese, Sentence, Task, Term},
        enum_nse as nse, enum_nse_sentence as nse_sentence, enum_nse_task as nse_task,
        enum_nse_term as nse_term,
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
            nse => @ Narsese::Sentence(Sentence::Judgement(..)),
            nse => @ Narsese::Sentence(Sentence::Judgement(Term::Inheritance(..), ..)),
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

    /// 测试/专用化
    #[test]
    fn test_specialize() {
        asserts! {
            // 词项
            nse_term!(<A --> B>) => @ Term::Inheritance(..),
            // 语句
            nse_sentence!(<A --> B>.) => @ Sentence::Judgement(..),
            // 任务
            nse_task!(<A --> B>. :!-1: %1.0;0.9%) => @ Task(..),
        }

        // 兼容模式
        asserts! {
            // 语句→任务的隐式转换
            nse_task!(<A --> B>.) => nse_task!($$ <A --> B>.),
            nse_task!(<A --> B>.) => @ Task(..),
        }
    }
}
