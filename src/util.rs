//! 一些实用工具、定义、函数

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
    /// 验证「0-1」合法性
    /// * 📌短暂借走所有权，比对后归还
    fn validate_01(self) -> Self;
}

/// 实现
impl ZeroOneFloat for FloatPrecision {
    fn validate_01(self) -> Self {
        if self < 0.0 || self > 1.0 {
            panic!("「0-1」区间外的值（建议：`0<x<1`）");
        }
        self
    }
}
