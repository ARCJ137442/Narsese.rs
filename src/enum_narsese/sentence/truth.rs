//! 统一定义「真值」
//!
//! 📌分类
//! * 空真值
//! * 单真值
//! * 双真值

use crate::enum_narsese::hyper_parameters::*;
use util::ZeroOneFloat;

/// 使用枚举表示真值
/// * 📌与目标使用的「预算值」同一
/// * 📌兼容不同的缺省形式
///   * 空真值
///   * 单真值
///   * 双真值
#[derive(Debug, Clone, PartialEq)]
pub enum Truth {
    /// 空真值（默认）
    Empty,
    /// 单真值
    Single(FloatPrecision),
    /// 双真值
    Double(FloatPrecision, FloatPrecision),
}

/// 实现/构造
impl Truth {
    /// 构造「空真值」
    pub fn new_empty() -> Self {
        Truth::Empty
    }

    /// 构造「单真值」
    pub fn new_single(f: FloatPrecision) -> Self {
        Truth::Single(f.validate_01())
    }

    /// 构造「双真值」
    pub fn new_double(f: FloatPrecision, c: FloatPrecision) -> Self {
        Truth::Double(f.validate_01(), c.validate_01())
    }
}

/// 实现/属性
impl Truth {
    /// 获取「频率」
    pub fn frequency(&self) -> FloatPrecision {
        match self {
            Truth::Single(frequency) | Truth::Double(frequency, _) => *frequency,
            _ => panic!("尝试获取缺省的值"),
        }
    }

    /// 获取「信度」
    pub fn confidence(&self) -> FloatPrecision {
        match self {
            Truth::Double(_, confidence) => *confidence,
            _ => panic!("尝试获取缺省的值"),
        }
    }

    /// 【辅助】用`f`快速获取「频率」
    pub fn f(&self) -> FloatPrecision {
        self.frequency()
    }

    /// 【辅助】用`c`快速获取「信度」
    pub fn c(&self) -> FloatPrecision {
        self.confidence()
    }
}

/// 单元测试/真值
#[cfg(test)]
mod tests_truth {
    use util::fail_tests;

    use super::*;

    /// 辅助构造示例
    #[inline(always)]
    fn new_examples(f: FloatPrecision, c: FloatPrecision) -> (Truth, Truth, Truth) {
        let empty = Truth::new_empty();
        let single = Truth::new_single(f);
        let double = Truth::new_double(f, c);
        (empty, single, double)
    }

    /// valid - new
    #[test]
    fn test_new_valid() {
        let (f, c) = (0.5, 0.5);
        let (empty, single, double) = new_examples(f, c);
        println!("empty: {empty:?}");
        println!("single: {single:?}");
        println!("double: {double:?}");
    }

    /// valid - get
    #[test]
    fn test_valid_get() {
        let (f, c) = (0.5, 0.5);
        let (_, single, double) = new_examples(f, c);

        // f
        // assert_eq!(empty.f(), f);
        assert_eq!(single.f(), f);
        assert_eq!(double.f(), f);

        // c
        assert_eq!(double.c(), c);
    }

    // invalid //
    fail_tests! {
        /// invalid - new | f | >1
        test_new_invalid_f_up Truth::new_single(1.5);

        /// invalid - new | f | <0
        test_new_invalid_f_down Truth::new_single(-0.5);

        /// invalid - new | c | >1
        test_new_invalid_c_up Truth::new_double(0.5, 1.5);

        /// invalid - new | c | <0
        test_new_invalid_c_down Truth::new_double(0.5, -0.5);

        /// invalid - get | f | empty
        test_get_invalid_f_empty Truth::new_empty().f();

        /// invalid - get | c | empty
        test_get_invalid_c_empty Truth::new_empty().c();

        /// invalid - get | c | single
        test_get_invalid_c_single Truth::new_single(0.5).c();
    }
}
