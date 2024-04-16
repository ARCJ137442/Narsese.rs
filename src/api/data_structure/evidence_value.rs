//! 「证据值」
//! * 📌基于NARS中「证据与真值」的系统
//! * 🎯在「抽象特征」的层面统一「真值」与「欲望值」
//!   * 📄实现其接口的，一律支持「真值函数」，不论内部附加的数据多么复杂

use std::ops::{Add, Div, Mul, Sub};

/// 抽象API「证据值」
/// * 🚩【2024-04-16 18:59:46】目前所内含的类型**必须实现四则运算**
///   * 💭【2024-04-16 19:11:58】后续有可能为此要添加更多特征约束
///   * 📌「频率」「信度」必须是一种类型：实际真值函数中会包含「频率×信度」等情况
pub trait EvidentValue<V>
where
    V: Add<Output = V> + Sub<Output = V> + Mul<Output = V> + Div<Output = V>,
{
    /// 获取「频率」
    /// * 📌对应「真值」和「欲望值」中的「频率」
    fn get_frequency(&self) -> V;

    /// 获取「信度」
    /// * 📌对应「真值」和「欲望值」中的「信度」
    fn get_confidence(&self) -> V;

    /// （获取）「频率」
    /// * 🎯[`Self::get_frequency`]方法的短别名
    #[inline(always)]
    fn frequency(&self) -> V {
        self.get_frequency()
    }

    /// （获取）「信度」
    /// * 🎯[`Self::get_confidence`]方法的短别名
    #[inline(always)]
    fn confidence(&self) -> V {
        self.get_confidence()
    }
}

/// 「可变证据值」
/// * 在「[证据值](EvidentValue)」的基础上，允许改变其频率和信度
pub trait EvidentValueMut<V>: EvidentValue<V>
where
    V: Add<Output = V> + Sub<Output = V> + Mul<Output = V> + Div<Output = V>,
{
    /// 设置「频率」
    /// * 📌对应「真值」和「欲望值」中的「频率」
    /// * 🚩【2024-04-16 19:09:41】此处使用不可变引用，旨在显式提示「从复杂类型中拷贝的成本」
    fn set_frequency(&mut self, new_f: &V);

    /// 设置「信度」
    /// * 📌对应「真值」和「欲望值」中的「信度」
    /// * 🚩【2024-04-16 19:09:41】此处使用不可变引用，旨在显式提示「从复杂类型中拷贝的成本」
    fn set_confidence(&mut self, new_c: &V);
}

// /// 从「频率」「信度」来
// /// * 🎯统一解决「从『频率』『信度』中构造『真值/预算值』，但可能某些实现不支持」的问题
// pub trait FromEvidentFC<V>: EvidentValue<V>
// where
//     F: Add + Sub + Mul + Div,
//     C: Add + Sub + Mul + Div,
// {
//     /// 从「频率」「信度」构造自身
//     /// * 🎯用于实现「挪用所有权的真值计算」
//     fn from_fc(f: F, c: C) -> Self;
// }
// ! 🚩【2024-04-16 19:20:44】目前不使用：实际上可以「先创建真值，再对其修改」

/// 单元测试
#[cfg(test)]
mod test {
    use super::*;
    use util::{asserts, manipulate};

    /// 统一的浮点数类型
    type V = f64;

    /// 测试用真值
    #[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
    struct TruthV {
        f: V,
        c: V,
    }

    impl EvidentValue<V> for TruthV {
        fn get_frequency(&self) -> V {
            self.f
        }

        fn get_confidence(&self) -> V {
            self.c
        }
    }

    impl EvidentValueMut<V> for TruthV {
        fn set_frequency(&mut self, new_f: &V) {
            self.f = *new_f;
        }

        fn set_confidence(&mut self, new_c: &V) {
            self.c = *new_c;
        }
    }

    /// 测试/真值数据结构
    #[test]
    fn test_truth_v() {
        let mut t = TruthV { f: 0.5, c: 0.5 };
        // 获取真值
        asserts! {
            t.get_frequency() => 0.5,
            t.get_confidence() => 0.5,
        }
        // 设置真值
        t.set_frequency(&1.0);
        t.set_confidence(&0.9);
        asserts! {
            t.get_frequency() => 1.0,
            t.get_confidence() => 0.9,
        }
    }

    /// 测试/真值函数
    /// * 🚩仅用于「原地计算」不在其中创建任何新对象
    ///   * 要使用「创建新对象的函数」可以「先[`Default`]，再修改」
    /// * 📝【2024-04-16 19:21:41】目前有两个逻辑
    ///   * 纯不可变逻辑：每次推理即创建一个新的值，基于「从频率、信度来」
    ///     * 💭性能问题：创建新对象需要分配内存
    ///   * 可变逻辑：每次推理都基于现有的值，即便不可避免会有「模板值」的问题
    ///     * 💭特征要求问题：需要都实现「可变证据值」
    ///   * 🚧TODO：亟待统一的「最终方案」
    trait TruthWithFunctions<V>
    where
        Self: EvidentValueMut<V>,
        V: Add<Output = V> + Sub<Output = V> + Mul<Output = V> + Div<Output = V>,
    {
        /// 测试/一般演绎
        /// * ❓【2024-04-16 19:22:54】是否有可能用宏来实现「自动产生多种版本」
        fn deduction(tv1: &Self, tv2: &Self, target: &mut Self) {
            let prod_f = tv1.get_frequency() * tv2.get_frequency();
            let prod_c = tv1.get_confidence() * tv2.get_confidence();
            target.set_frequency(&prod_f);
            target.set_confidence(&(prod_f * prod_c));
        }
    }

    impl TruthWithFunctions<V> for TruthV {}

    /// 测试/真值函数
    #[test]
    fn test_deduction() {
        let t1 = TruthV { f: 1.0, c: 0.9 };
        let t2 = TruthV { f: 1.0, c: 0.9 };

        // 演绎推理の结果
        let deducted = manipulate!(TruthV::default() => TruthV::deduction(&t1, &t2, _));

        // 测试演绎推理
        assert_eq!(dbg!(deducted), TruthV { f: 1.0, c: 0.81 })
    }
}
