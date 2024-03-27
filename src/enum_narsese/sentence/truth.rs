//! 统一定义「真值」
//!
//! 📌分类
//! * 空真值
//! * 单真值
//! * 双真值

use crate::api::hyper_parameters::*;
use util::ZeroOneFloat;

/// 使用枚举表示真值
/// * 📌与目标使用的「预算值」同一
/// * 📌兼容不同的缺省形式
///   * 空真值
///   * 单真值
///   * 双真值
///
/// ! ❌【2024-03-27 20:54:19】浮点数[`f32`]、[`f64`]不支持[`Hash`]特征
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
    ///
    /// # Panics
    /// ! 若其中的值不符合范围，会发生panic
    pub fn new_single(f: FloatPrecision) -> Self {
        Truth::Single(*f.validate_01())
    }

    /// 构造「双真值」
    ///
    /// # Panics
    /// ! 若其中的值不符合范围，会发生panic
    pub fn new_double(f: FloatPrecision, c: FloatPrecision) -> Self {
        Truth::Double(*f.validate_01(), *c.validate_01())
    }

    /// 尝试从「浮点数迭代器」中提取真值
    /// * 🚩多余的值会被忽略
    /// * 🚩无效的值会被上报（作为字符串提示）
    /// * 📌边声明边提取边检验，空间基本最小开销：按需分配浮点数空间
    pub fn try_from_floats(
        mut floats: impl Iterator<Item = FloatPrecision>,
    ) -> Result<Truth, String> {
        // 尝试提取第一个，提取不了⇒空 | 边提取边检查范围
        let f = match floats.next() {
            Some(v) => *v.try_validate_01()?,
            None => return Ok(Self::new_empty()),
        };
        // 尝试提取第二个，提取不了⇒单 | 边提取边检查范围
        let c = match floats.next() {
            Some(v) => *v.try_validate_01()?,
            None => return Ok(Self::new_single(f)),
        };
        // 两个都存在⇒双
        Ok(Self::new_double(f, c))
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
    use super::*;
    use util::fail_tests;

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
