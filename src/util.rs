//! 一些实用工具、定义、函数
//! * 📌宏定义专门放在[`macros.rs`]中
//!   * 📄参考标准库与其它包（如`winnow`）

use crate::push_str;

/// 统一指定「精度」类型
/// * 🎯真值
/// * 🎯预算值
/// * 💫难点：无法通过泛型处理形如 `f32 | f64` 的类型标注
///   * 🕚时间：【2024-02-19 22:42:18】
///   * ❌无法处理「类型比对」的操作（f32无法和f64比对，反之亦然，不兼容）
///   * ❌无法使用「统一特征」的方式
///     * 🚩方法代码：`trait Float{}; impl Float for f32 {}; impl Float for f64 {};`
///   * ❌无法处理「构造传参」中有关的「常量转换操作」
///     * ❗类似`new_single(1.0)`，此中之常量无法转换为「精度」对象
///     * ❌无法使用`as`：无法限制`Precision`为基础类型
///     * ❌无法使用`From<f64>`的方法：[`f32`]未实现[`From<f64>`]特征，反之亦然
pub type FloatPrecision = f64;
/// 默认的整数精度
/// * 🎯时间戳/固定时间 | OpenNARS/PyNARS均支持「负整数时间」
pub type IntPrecision = isize;

/// 「0-1」实数
/// 📌通过特征为浮点数添加「0-1 限制」方法
///   * 📝而非直接`impl FloatPrecision`：孤儿规则
pub trait ZeroOneFloat {
    /// 判断是否在范围内
    fn is_in_01(&self) -> bool;
    /// 验证「0-1」合法性
    /// * 📌短暂借走所有权，比对后归还
    /// * ⚠️若不在范围内，则产生panic
    fn validate_01(self) -> Self;
}

/// 实现
impl ZeroOneFloat for FloatPrecision {
    fn is_in_01(&self) -> bool {
        *self >= 0.0 && *self <= 1.0
    }
    fn validate_01(self) -> Self {
        if self < 0.0 || self > 1.0 {
            panic!("「0-1」区间外的值（建议：`0<x<1`）");
        }
        self
    }
}

/// 单元测试/「0-1」实数
#[cfg(test)]
mod tests_01_float {
    use crate::{fail_tests, show};

    use super::*;
    /// 辅助用测试宏/成功测试
    /// ! 📝`, $(,)?`这里的「,」代表的不是「分隔表达式」，而是「模式中的`,`」
    /// * 故应去除这前边的「,」
    macro_rules! test_valid {
        // 📝实际上这里边啥括号都可以
        [$($num:expr),* $(,)?] => {
            $(
                let v = $num.validate_01();
                assert_eq!(v, $num);
                show!($num.validate_01());
            )*
        };
    }

    #[test]
    fn test_01_float_valid() {
        test_valid![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    }

    /// 辅助用测试宏/失败测试
    /// ! 📝`, $(,)?`这里的「,」代表的不是「分隔表达式」，而是「模式中的`,`」
    /// * 故应去除这前边的「,」
    macro_rules! test_all_invalid {
        ($($name:ident => $num:expr),* $(,)?) => {
            // 直接用`fail_tests!`生成失败测试
            fail_tests!{
                $(
                    $name {
                        $num.validate_01();
                    }
                )*
            }
        };
    }

    test_all_invalid! {
        // 大数
        fail_1_1 => 2.0,
        fail_3_0 => 3.0,
        fail_10_0 => 10.0,
        // 负数
        fail_n_0_1 => -0.1,
        fail_n_0_2 => -0.2,
        fail_n_2_0 => -2.0,
    }
}

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
