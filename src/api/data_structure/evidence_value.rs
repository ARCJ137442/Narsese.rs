//! 「证据值」
//! * 📌基于NARS中「证据与真值」的系统
//! * 🎯在「抽象特征」的层面统一「真值」与「欲望值」
//!   * 📄实现其接口的，一律支持「真值函数」，不论内部附加的数据多么复杂

use std::ops::{Add, Div, Mul, Sub};

/// 证据数值
/// * 📌抽象API「证据值」的「数值」类型
/// * 🎯统一其作为「0-1值」的特征
pub trait EvidentNumber:
    Sized
    // ! ❌【2024-05-02 17:25:42】无法将`Rhs`类参定为`&Self`：引用生命周期问题
    // * 🚩因此暂且直接使用值类型
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialEq // 实现判等，但不是「完全相等」（兼容「浮点数」本身）
    + Copy // * 🚩对上述四则运算的妥协：需要频繁采取「移动语义」并伴随着值赋值 | 这亦要求「尽可能让拷贝成本低」
    /*  + TryFrom<Float, Error = Self::TryFromError> */ // ! ←↓❌【2024-05-02 17:31:34】无法统一精度，故不使用
{
    // /// * 📌此处对[`Error`](std::fmt::Error)的需求仅仅在于[`Result::unwrap`]需要`Error: Debug`
    // /// * 🎯【2024-05-02 12:17:19】引入以兼容[`TryFrom`]的[`try_from`](TryFrom::try_from)
    // type TryFromError: std::error::Error;

    // 基础：合法性检查相关 //

    /// 判断其是否合法
    /// * 🎯用于验证是否具有「合法性」
    ///   * 📄一般的「频率」「信度」均处在 0≤x≤1 的范围
    /// * 📜默认实现：总是合法
    #[inline(always)]
    fn is_valid(&self) -> bool {
        true
    }

    /// 尝试验证值是否合法
    /// * 📜默认实现：基于[`Self::try_validate`]
    ///   * ⚠️需要保证与[`is_valid`](Self::is_valid)的一致性
    ///   * 🚩不合法⇒字符串形式的[`Err`]值
    #[inline]
    fn try_validate(&self) -> Result<&Self, &str> {
        match self.is_valid() {
            true => Ok(self),
            false => Err("证据数值不合法"),
        }
    }

    /// （强制）验证其是否合法
    /// * 🎯验证其是否合法
    ///   * ⚠️不合法⇒panic
    /// * 📜默认实现：基于[`Self::try_validate`]
    ///   * ⚠️需要保证与[`try_validate`](Self::try_validate)的一致性
    ///
    /// # Panics
    /// ! ⚠️当其经过[`Self::try_validate`]检验为[`Err`]时，会导致panic
    #[inline(always)]
    fn validate(&self) -> &Self {
        // * 📝这里直接使用`unwrap`即可：报错信息会写「called `Result::unwrap()` on an `Err` value: ...」
        self.try_validate().unwrap()
    }

    // 基础：数值相关 //

    /// 常数「0」
    /// * 🎯用于各种「逻辑计算」的常量
    ///   * 📄逻辑或「多项加和」的起始量
    fn zero() -> Self;

    /// 常数「1」
    /// * 🎯用于各种「逻辑计算」的常量
    ///   * 📄逻辑或「多项加和」的起始量
    fn one() -> Self;

    // /// 常数「0」
    // /// * 🎯用于各种「逻辑计算」的常量
    // ///   * 📄逻辑或「多项加和」的起始量
    // const ZERO: Self;

    // /// 常数「1」
    // /// * 🎯用于各种「逻辑计算」的常量
    // ///   * 📄逻辑或「多项加和」的起始量
    // const ONE: Self;

    /// n次开根
    /// * 🎯用于NAL的「几何均值」（n次开根）
    fn root(self, n: usize) -> Self;

    // ! ❌【2024-05-02 18:00:33】暂且不追加对「与NAL直接相关的数值运算」的实现要求，只涉及最基本的数学运算
    // * 📄不直接要求「w2c」和「c2w」（c2w已超出范围）
}

/// 对「0-1浮点数」提供默认实现
/// * ✅已解决「常量0、常量1无法自动提供」的问题：使用`From<FloatPrecision>`自动获取
mod impl_num_float {
    use super::*;
    use crate::api::FloatPrecision;
    use nar_dev_utils::floats::ZeroOneFloat;

    /// 对所有「0-1 浮点数」批量实现「证据数值」
    /// * 🎯对[`f32`]、[`f64`]统一提供默认实现
    /// * 🚩【2024-04-17 10:53:45】目前直接采用「0-1 实数」的处理方法
    /// * 📝【2024-04-17 10:54:27】对「外部类型」批量实现「已有类型」没问题
    ///   * ✅不会被「孤儿规则」限制
    impl<F> EvidentNumber for F
    where
        F: ZeroOneFloat
            + Add<Output = Self>
            + Sub<Output = Self>
            + Mul<Output = Self>
            + Div<Output = Self>
            + PartialEq
            + Copy
            + PartialOrd<Self>
            + From<FloatPrecision>
            + Into<FloatPrecision>,
    {
        #[inline(always)]
        fn is_valid(&self) -> bool {
            self.is_in_01()
        }

        #[inline(always)]
        fn try_validate(&self) -> Result<&Self, &str> {
            self.try_validate_01()
        }

        #[inline(always)]
        fn validate(&self) -> &Self {
            self.validate_01()
        }

        #[inline(always)]
        fn zero() -> Self {
            Self::from(0.0)
        }

        #[inline(always)]
        fn one() -> Self {
            Self::from(1.0)
        }

        // ! ❌无法真正贯彻「关联常量」的报错：`cannot call non-const fn `<F as std::convert::From<f64>>::from` in constants`
        // const ZERO: Self = Self::from(0.0);
        // const ONE: Self = Self::from(1.0);

        #[inline(always)]
        fn root(self, n: usize) -> Self {
            // * 🚩通过「转换为标准浮点数」默认支持「n次开根」
            Self::from(self.into().powf(1.0 / (n as FloatPrecision)))
        }
    }
}

/// 抽象API「证据值」
/// * 🚩【2024-04-16 18:59:46】目前所内含的类型**必须实现四则运算**
///   * 💭【2024-04-16 19:11:58】后续有可能为此要添加更多特征约束
///   * 📌「频率」「信度」必须是一种类型：实际真值函数中会包含「频率×信度」等情况
/// * 🚩【2024-04-17 10:33:56】在「获取内部值」方面，**不强制要求返回引用**
///   * ✨若要求返回自身部分的引用，可以将`V`限定为引用类型
///     * ⚠️由引用类型带来的复杂度，实现者自行处理
///   * ❌
pub trait EvidentValue<V: EvidentNumber> {
    /// 获取「频率」
    /// * 📌对应「真值」和「欲望值」中的「频率」
    fn get_frequency(&self) -> V;

    /// 获取「信度」
    /// * 📌对应「真值」和「欲望值」中的「信度」
    fn get_confidence(&self) -> V;

    /// 获取「(频率, 信度)」
    /// * 🎯获取「频率」「信度」二者
    #[inline(always)]
    fn get_frequency_confidence(&self) -> (V, V) {
        (self.get_frequency(), self.get_confidence())
    }

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
    V: EvidentNumber,
{
    /// 设置「频率」
    /// * 📌对应「真值」和「欲望值」中的「频率」
    /// * 🚩【2024-04-16 19:09:41】此处使用不可变引用，旨在显式提示「从复杂类型中拷贝的成本」
    fn set_frequency(&mut self, new_f: &V);

    /// 设置「信度」
    /// * 📌对应「真值」和「欲望值」中的「信度」
    /// * 🚩【2024-04-16 19:09:41】此处使用不可变引用，旨在显式提示「从复杂类型中拷贝的成本」
    fn set_confidence(&mut self, new_c: &V);

    /// 同时设置「频率」与「信度」
    /// * 🎯便捷集成「设置频率」与「设置信度」
    /// * 🎯零成本抽象：可以被自动内联展开
    /// * 📜默认实现：同时设置「频率」与「信度」
    #[inline(always)]
    fn set_frequency_confidence(&mut self, new_f: &V, new_c: &V) {
        self.set_frequency(new_f);
        self.set_confidence(new_c);
    }
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

/// 为实现了[`Copy`]的二元组`(f, c)`自动实现「证据值」与「可变证据值」
/// * 🚩【2024-04-17 10:42:14】需要[`Copy`]：引用类型会带来一堆生命周期问题
impl<V: EvidentNumber + Copy> EvidentValue<V> for (V, V) {
    fn get_frequency(&self) -> V {
        self.0
    }

    fn get_confidence(&self) -> V {
        self.1
    }
}

/// 单元测试
#[cfg(test)]
mod test {
    use super::*;
    use nar_dev_utils::{asserts, for_in_ifs, macro_once, manipulate, pipe};

    /// 统一的浮点数类型
    type V = f64;

    /// 测试用真值
    #[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
    struct TruthV {
        f: V,
        c: V,
    }

    /// 实现「证据值」
    impl EvidentValue<V> for TruthV {
        fn get_frequency(&self) -> V {
            self.f
        }

        fn get_confidence(&self) -> V {
            self.c
        }
    }

    /// 实现「可变证据值」
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

    /// W值
    /// * 🎯表示在[`EvidentValue`]之外的「w」「w⁺」「w⁻」
    /// * 🎯抽象、可扩展地表征诸如「w2c」的真值函数
    /// * 🚩【2024-04-17 11:29:11】添加[`Copy`]约束以避开所有权问题（所有权🆚简洁度）
    trait ValueW: Sized + Add<Output = Self> + Div<Output = Self> + Copy {}

    macro_once! {
        /// 对浮点数实现「[W值](ValueW)」
        macro impl_value_w_for_float($($t:ty)*) {
            $(
                impl ValueW for $t {
                }
            )*
        }
        // 32位和64位浮点数
        f32
        f64
    }

    /// 测试/真值函数
    /// * 🚩仅用于「原地计算」不在其中创建任何新对象
    ///   * 要使用「创建新对象的函数」可以「先[`Default`]，再修改」
    /// * 📝【2024-04-16 19:21:41】目前有两个逻辑
    ///   * 纯不可变逻辑：每次推理即创建一个新的值，基于「从频率、信度来」
    ///     * 💭性能问题：创建新对象需要分配内存
    ///   * 可变逻辑：每次推理都基于现有的值，即便不可避免会有「模板值」的问题
    ///     * 💭特征要求问题：需要都实现「可变证据值」
    /// * 🚩【2024-04-17 12:49:38】目前方案：基于「可变证据值」辅以「快捷辅助函数」兼顾「高性能」与「简洁性」
    /// * 🎯【2024-04-17 12:50:24】亦用作NAL真值函数的有关示范
    trait TruthWithFunctions<V>
    where
        Self: EvidentValueMut<V>,
        // ! 🚩【2024-04-17 11:35:59】↓对此约束`ValueW`，以便实现`w2c`
        V: EvidentNumber + ValueW,
    {
        // 辅助函数 //

        /// 辅助函数/短获取「频率」
        /// * 🎯短别名获取「频率」
        #[inline(always)]
        fn f(&self) -> V {
            self.frequency()
        }

        /// 辅助函数/短获取「信度」
        /// * 🎯短别名获取「信度」
        #[inline(always)]
        fn c(&self) -> V {
            self.confidence()
        }

        /// 辅助函数/短获取「(频率, 信度)」
        /// * 🎯短别名获取「频率」「信度」二者
        #[inline(always)]
        fn fc(&self) -> (V, V) {
            self.get_frequency_confidence()
        }

        /// 辅助函数/短同时设置「频率」与「信度」
        /// * 🎯短别名「设置频率与信度」
        /// * 🎯简洁性考量：无需刻意引用
        ///   * 🚩以「消耗所有权」为代价，换取「调用时无需显式引用」的便捷
        ///   * 📄【2024-04-17 11:45:18】目前大多数「最终设置」都是「设置完便删除」以及「自身能被隐式复制」的
        #[inline(always)]
        fn set_fc(&mut self, new_f: V, new_c: V)
        where
            V: Copy,
        {
            self.set_frequency(&new_f);
            self.set_confidence(&new_c);
        }

        /// 辅助函数/计算「频率の与」
        fn f_and(&self, other: &Self) -> V {
            Self::and(self.f(), other.f())
        }

        /// 辅助函数/计算「信度の与」
        fn c_and(&self, other: &Self) -> V {
            Self::and(self.c(), other.c())
        }

        /// 辅助函数/计算「频率の与」和「信度の与」
        fn fc_and(&self, other: &Self) -> (V, V) {
            (Self::f_and(self, other), Self::c_and(self, other))
        }

        /// 辅助函数/计算「频率の或」
        fn f_or(&self, other: &Self) -> V {
            Self::or(self.f(), other.f())
        }

        /// 辅助函数/计算「信度の或」
        fn c_or(&self, other: &Self) -> V {
            Self::or(self.c(), other.c())
        }

        /// 辅助函数/计算「频率の或」和「信度の或」
        fn fc_or(&self, other: &Self) -> (V, V) {
            (Self::f_or(self, other), Self::c_or(self, other))
        }

        // （证据）数值函数 //
        // * 🎯有关「数值运算」而非「推理规则」的函数
        // * 📝与「推理规则」相关，但又不直接涉及「频率-信度」对

        /// 逻辑与
        /// * 📝这个「逻辑与」就是数值相乘
        #[inline(always)]
        fn and(v1: V, v2: V) -> V {
            v1 * v2
        }

        /// 逻辑与（多个）
        /// * 📜空⇒1
        /// * 📝由交换律、结合律而稳定
        /// * 🚩放弃【必须用引用类型，但`&V`未实现`EvidenceNumber`】的`reduce`方案
        ///   * 🚩【2024-04-17 12:13:31】现在使用从`V::one`开始的`fold`方案
        fn and_multi(v: impl IntoIterator<Item = V>) -> V {
            v.into_iter().fold(V::one(), |acc, vi| acc * vi)
        }

        /// 逻辑非
        /// * 📝就是「1-自身」
        #[inline(always)]
        fn not(v: V) -> V {
            V::one() - v
        }

        /// 逻辑或
        /// * ✅用乘法交换律保证交换律
        /// * 📝这个「逻辑或」是满足结合律的（借助乘法交换律）
        /// * 🚩亦可利用「德摩根律」实现
        #[inline(always)]
        fn or(v1: V, v2: V) -> V {
            let one = V::one();
            one - (one - v1) * (one - v2)
            // Self::not(Self::and(Self::not(v1), Self::not(v2))) // ! 德摩根律实现法，但不够简洁
        }

        /// 逻辑或（多个）
        /// * 📜空⇒0
        /// * 📝由交换律、结合律而稳定
        /// * 🚩利用德摩根律实现高效抽象
        ///   * 🚩【2024-04-17 12:13:31】现在使用从`V::one`开始的`fold`方案
        fn or_multi(v: impl IntoIterator<Item = V>) -> V {
            pipe! {
                // 先转换为迭代器
                v.into_iter()
                // * 🚩非
                => .map(Self::not)
                // * 🚩与
                => Self::and_multi
                // * 🚩非
                => Self::not
            }
        }

        /// 除法，但对「分母为零」作特殊返回
        /// * 🎯对`comparison`作简化
        fn div_avoid_zero(be_div: V, div_by: V, value_when_zero: V) -> V {
            if div_by == V::zero() {
                value_when_zero
            } else {
                be_div / div_by
            }
        }

        /// 除法，但对「分母为零」作特殊返回「0」
        /// * 🎯对`comparison`作简化
        fn div_or_zero(be_div: V, div_by: V) -> V {
            Self::div_avoid_zero(be_div, div_by, V::zero())
        }

        /// 从「总数」变到「信度」
        /// * 🎯复刻NAL中的「总样例数」与「信度」的关系
        /// * 🚩即`w2c`
        fn w2c(v: V) -> V {
            let one = V::one();
            one / (v + one)
        }

        // 具体推理规则 //

        /// 演绎
        /// * ✨对称
        /// * 🚩原理
        ///   * 🚩频率 = 频率の与
        ///   * 🚩信度 = 频率の与 * 信度の与
        /// * ❓【2024-04-16 19:22:54】是否有可能用宏来实现「自动产生多种版本」
        fn deduction(&self, other: &Self, target: &mut Self) {
            let (prod_f, prod_c) = self.fc_and(other);
            target.set_fc(prod_f, prod_f * prod_c);
        }

        /// 归纳
        /// * ⚠️非对称 @ 频率、信度
        /// * 🚩原理
        ///   * 🚩频率 = 第二者の频
        ///   * 🚩信度 = 总数视作信度（第一者の频 * 信度の与）
        fn abduction(&self, other: &Self, target: &mut Self) {
            let prod_c = self.c_and(other);
            let new_f = other.f();
            let new_c = Self::w2c(self.f() * prod_c);
            target.set_fc(new_f, new_c);
        }

        /// 归因
        /// * ⚠️非对称 @ 频率、信度
        /// * 🚩原理：反向归纳
        fn induction(&self, other: &Self, target: &mut Self) {
            other.abduction(self, target)
        }

        /// 解释
        /// * ✨对称
        ///   * 🚩频率 = 1
        ///   * 🚩信度 = 总数视作信度（频率の与 * 信度の与）
        fn exemplification(&self, other: &Self, target: &mut Self) {
            let (prod_f, prod_c) = self.fc_and(other);
            let new_f = V::one();
            let new_c = Self::w2c(prod_f * prod_c);
            target.set_fc(new_f, new_c)
        }

        // ! 诸多`structural_XXX`所谓「结构性推理」蕴含「默认值」(1.0, 0.9)
        // * ❌无法留作一个「获取默认值」的特征函数：需要因此引入「从频率信度构造」的构造函数
        // * ❌除非引入新的特征函数，否则无法准确表示不同「证据数值」中的「0.9」

        // * 🚩原理：other = %1.0, 0.9%
        // fn structural_deduction(&self, target: &mut Self)

        /// 否定
        /// * 🚩原理
        ///   * 🚩频率 = !自の频率
        ///   * 🚩信度 = 自の信度
        fn negation(&self, target: &mut Self) {
            target.set_fc(Self::not(self.f()), self.c())
        }

        /// 否定（对自身）
        /// * 🚩原理
        ///   * 🚩频率 = !自の频率
        ///   * 🚩信度 = 自の信度
        fn negate(&mut self) {
            self.set_fc(Self::not(self.f()), self.c())
        }

        /// 演绎否定
        /// * ✨对称
        /// * 🚩原理
        ///   * 1 演绎产生新值
        ///   * 2 否定新值
        fn deduction_negated(&self, other: &Self, target: &mut Self) {
            self.deduction(other, target);
            target.negate();
        }

        // * 🚩原理：other = %1.0, 0.9%
        // fn structural_deduction_negated(&self, target: &mut Self)

        /// 相交
        /// * ✨对称
        /// * 🚩原理
        ///   * 🚩频率 = 频率の与
        ///   * 🚩信度 = 信度の与
        fn intersection(&self, other: &Self, target: &mut Self) {
            let (new_f, new_c) = self.fc_and(other);
            target.set_fc(new_f, new_c);
        }

        // * 🚩原理：other = %1.0, 0.9%
        // fn structural_intersection(&self, target: &mut Self)

        /// 比较
        /// * ✨对称
        /// * 🚩原理
        ///   * 🚩频率 = 频率の与 / 频率の或 （频率の或=0 ⇒ 0）
        ///   * 🚩信度 = 频率の或
        fn comparison(&self, other: &Self, target: &mut Self) {
            // 缓存变量
            let f_and = self.f_and(other);
            let f_or = self.f_or(other);
            // 使用缓存的变量
            let new_f = Self::div_or_zero(f_and, f_or);
            let new_c = f_or;
            target.set_fc(new_f, new_c)
        }

        /// 类比
        /// * ⚠️非对称 @ 信度
        /// * 🚩原理
        ///   * 🚩频率 = 频率の与
        ///   * 🚩信度 = 频率の与
        fn analogy(&self, other: &Self, target: &mut Self) {
            let new_f = self.f_and(other);
            let new_c = self.c_and(other) * other.f();
            target.set_fc(new_f, new_c);
        }

        /// 类似
        /// * ✨对称
        /// * 🚩原理
        ///   * 🚩频率 = 频率の与
        ///   * 🚩信度 = 频率の与 * 信度の或
        fn resemblance(&self, other: &Self, target: &mut Self) {
            let new_f = self.f_and(other);
            let new_c = self.c_and(other) * self.f_or(other);
            target.set_fc(new_f, new_c);
        }

        /// 相并
        /// * ✨对称
        /// * 🚩原理
        ///   * 🚩频率 = 频率の或
        ///   * 🚩信度 = 信度の与
        fn union(&self, other: &Self, target: &mut Self) {
            let new_f = self.f_or(other);
            let new_c = self.c_and(other);
            target.set_fc(new_f, new_c);
        }

        /// 相差
        /// * ⚠️非对称 @ 频率
        /// * 🚩原理
        ///   * 🚩频率 = 自の频率 * !他の频率
        ///   * 🚩信度 = 信度の与
        fn difference(&self, other: &Self, target: &mut Self) {
            let new_f = self.f() * Self::not(other.f());
            let new_c = self.c_and(other);
            target.set_fc(new_f, new_c);
        }
    }

    /// 全自动批量实现
    impl<T> TruthWithFunctions<V> for T
    where
        T: EvidentValueMut<V>,
        V: EvidentNumber + ValueW,
    {
    }

    /// 测试/数值函数
    /// * 🎯正确性、健壮性
    #[test]
    fn test_number() {
        // 逻辑与、或、非 //
        // 二元 = 多元の二元情况
        for_in_ifs! {
            {
                // 逻辑与
                assert_eq!(
                    TruthV::and(v1, v2),
                    TruthV::and_multi([v1, v2].into_iter())
                );
                // 逻辑或
                assert_eq!(
                    TruthV::or(v1, v2),
                    TruthV::or_multi([v1, v2].into_iter())
                );
            }
            for v1 in ([0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0])
            for v2 in ([0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0])
        }
    }

    /// 测试/推理规则
    /// * 🎯验证「批量实现」起效
    /// * 🎯确保推理过程稳定性
    #[test]
    fn test_rules() {
        // 统一的测试用「目标」
        let mut target = TruthV { f: 0.5, c: 0.5 };

        // 测试用频率、信度、规则的范围
        let f_s = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
        let c_s = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
        let rules = [
            TruthV::deduction,
            TruthV::abduction,
            TruthV::induction,
            TruthV::exemplification,
            TruthV::deduction_negated,
            TruthV::intersection,
            TruthV::comparison,
            TruthV::analogy,
            TruthV::resemblance,
            TruthV::r#union,
            TruthV::difference,
        ];

        // 遍历、计算并保证其中不出panic
        for_in_ifs! {
            {
                // 构造临时真值
                let t1 = TruthV { f: *f_1, c: *c_1 };
                let t2 = TruthV { f: *f_2, c: *c_2 };
                // 计算（共用「目标」结构）
                rule_f(&t1, &t2, &mut target)
            }
            // 遍历所有可能的f、c值
            for f_1 in (f_s.iter())
            for f_2 in (f_s.iter())
            for c_1 in (c_s.iter())
            for c_2 in (c_s.iter())
            // 遍历所有可能的规则
            for rule_f in (rules.iter())
        }
    }

    /// 测试/演绎
    /// * 🎯验证该推理规则的正确性
    #[test]
    fn test_deduction() {
        let t1 = TruthV { f: 1.0, c: 0.9 };
        let t2 = TruthV { f: 1.0, c: 0.9 };

        // 演绎推理の结果
        let deducted = manipulate!(TruthV::default() => TruthV::deduction(&t1, &t2, _));

        // 测试演绎推理
        assert_eq!(dbg!(deducted), TruthV { f: 1.0, c: 0.81 })
    }

    // TODO: 增加更多有关「推理规则」的测试，用以验证抽象API的稳定性
}
