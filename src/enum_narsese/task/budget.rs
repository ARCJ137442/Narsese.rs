//! 统一定义「预算值」

use crate::api::hyper_parameters::*;
use util::*;

/// 使用枚举表示预算值
/// * 📌兼容不同的缺省形式
///   * 空预算
///   * 单预算
///   * 双预算
///   * 三预算
#[derive(Debug, Clone, PartialEq)]
pub enum Budget {
    /// 空预算
    Empty,
    /// 单预算
    Single(FloatPrecision),
    /// 双预算
    Double(FloatPrecision, FloatPrecision),
    /// 三预算
    Triple(FloatPrecision, FloatPrecision, FloatPrecision),
}
/// 实现/构造
impl Budget {
    /// 构造「空预算」
    pub fn new_empty() -> Self {
        Budget::Empty
    }

    /// 构造「单预算」
    pub fn new_single(p: FloatPrecision) -> Self {
        Budget::Single(*p.validate_01())
    }

    /// 构造「双预算」
    pub fn new_double(p: FloatPrecision, d: FloatPrecision) -> Self {
        Budget::Double(*p.validate_01(), *d.validate_01())
    }

    /// 构造「三预算」
    pub fn new_triple(p: FloatPrecision, d: FloatPrecision, q: FloatPrecision) -> Self {
        Budget::Triple(*p.validate_01(), *d.validate_01(), *q.validate_01())
    }

    /// 尝试从「浮点数迭代器」中提取预算值
    /// * 🚩多余的值会被忽略
    /// * 🚩无效的值会被上报（作为字符串提示）
    /// * 📌边声明边提取边检验，空间基本最小开销：按需分配浮点数空间
    pub fn try_from_floats(
        mut floats: impl Iterator<Item = FloatPrecision>,
    ) -> Result<Budget, String> {
        // 尝试提取第一个，提取不了⇒空 | 边提取边检查范围
        let p = match floats.next() {
            Some(v) => *v.try_validate_01()?,
            None => return Ok(Self::new_empty()),
        };
        // 尝试提取第二个，提取不了⇒单 | 边提取边检查范围
        let d = match floats.next() {
            Some(v) => *v.try_validate_01()?,
            None => return Ok(Self::new_single(p)),
        };
        // 尝试提取第三个，提取不了⇒双 | 边提取边检查范围
        let q = match floats.next() {
            Some(v) => *v.try_validate_01()?,
            None => return Ok(Self::new_double(p, d)),
        };
        // 三个都存在⇒三
        Ok(Self::new_triple(p, d, q))
    }
}

/// 实现/属性
impl Budget {
    /// 获取「优先级」
    pub fn priority(&self) -> FloatPrecision {
        match self {
            Budget::Single(priority)
            | Budget::Double(priority, _)
            | Budget::Triple(priority, _, _) => *priority,
            _ => panic!("尝试获取缺省的值"),
        }
    }

    /// 获取「耐久度」
    pub fn duality(&self) -> FloatPrecision {
        match self {
            Budget::Double(_, duality) | Budget::Triple(_, duality, _) => *duality,
            _ => panic!("尝试获取缺省的值"),
        }
    }

    /// 获取「质量」
    pub fn quality(&self) -> FloatPrecision {
        match self {
            Budget::Triple(_, _, quality) => *quality,
            _ => panic!("尝试获取缺省的值"),
        }
    }

    /// 【辅助】用`p`快速获取「优先级」
    pub fn p(&self) -> FloatPrecision {
        self.priority()
    }

    /// 【辅助】用`d`快速获取「耐久度」
    pub fn d(&self) -> FloatPrecision {
        self.duality()
    }

    /// 【辅助】用`q`快速获取「质量」
    pub fn q(&self) -> FloatPrecision {
        self.quality()
    }
}

/// 单元测试/预算值
#[cfg(test)]
mod tests_budget {
    use util::fail_tests;

    use super::*;

    /// 辅助构造示例
    #[inline(always)]
    fn new_examples(
        p: FloatPrecision,
        d: FloatPrecision,
        q: FloatPrecision,
    ) -> (Budget, Budget, Budget, Budget) {
        let empty = Budget::new_empty();
        let single = Budget::new_single(p);
        let double = Budget::new_double(p, d);
        let triple = Budget::new_triple(p, d, q);
        (empty, single, double, triple)
    }

    /// valid - new
    #[test]
    fn test_new_valid() {
        let (p, d, q) = (0.5, 0.5, 0.5);
        let (empty, single, double, triple) = new_examples(p, d, q);
        println!("empty: {empty:?}");
        println!("single: {single:?}");
        println!("double: {double:?}");
        println!("triple: {triple:?}");
    }

    /// valid - get
    #[test]
    fn test_valid_get() {
        let (p, d, q) = (0.5, 0.5, 0.5);
        let (_, single, double, triple) = new_examples(p, d, q);

        // p
        // assert_eq!(empty.p(), p);
        assert_eq!(single.p(), p);
        assert_eq!(double.p(), p);
        assert_eq!(triple.p(), p);

        // d
        // assert_eq!(empty.d(), d);
        // assert_eq!(single.d(), d);
        assert_eq!(double.d(), d);
        assert_eq!(triple.d(), d);

        // q
        // assert_eq!(empty.q(), q);
        // assert_eq!(single.q(), q);
        // assert_eq!(double.q(), q);
        assert_eq!(triple.q(), q);
    }

    /// from_floats
    #[test]
    fn test_from_floats() {
        let (p, d, q) = (0.5, 0.5, 0.5);
        let (empty, single, double, triple) = new_examples(p, d, q);
        asserts! {
            empty => Budget::try_from_floats([].into_iter()).unwrap()
            single => Budget::try_from_floats([p].into_iter()).unwrap()
            double => Budget::try_from_floats([p,d].into_iter()).unwrap()
            triple => Budget::try_from_floats([p,d,q].into_iter()).unwrap()
        }
    }

    // invalid //

    fail_tests! {
        /// invalid - new | p | >1
        test_new_invalid_p_up Budget::new_single(1.5);

        /// invalid - new | p | <0
        test_new_invalid_p_down Budget::new_single(-0.5);

        /// invalid - new | d | >1
        test_new_invalid_d_up Budget::new_double(0.5, 1.5);

        /// invalid - new | d | <0
        test_new_invalid_d_down Budget::new_double(0.5, -0.5);

        /// invalid - new | q | >1
        test_new_invalid_q_up Budget::new_triple(0.5, 0.5, 1.5);

        /// invalid - new | q | <0
        test_new_invalid_q_down Budget::new_triple(0.5, 0.5, -0.5);

        /// invalid - get | p | empty
        test_get_invalid_p_empty Budget::new_empty().p();

        /// invalid - get | d | empty
        test_get_invalid_d_empty Budget::new_empty().d();

        /// invalid - get | q | empty
        test_get_invalid_q_empty Budget::new_empty().q();

        /// invalid - get | d | single
        test_get_invalid_d_single Budget::new_single(0.5).d();

        /// invalid - get | q | single
        test_get_invalid_q_single Budget::new_single(0.5).q();

        /// invalid - get | q | double
        test_get_invalid_q_double Budget::new_double(0.5, 0.5).q();
    }
}
