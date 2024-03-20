//! 统一定义「时间戳」
//!
//! 📌分类
//! * 永恒
//! * 过去
//! * 现在
//! * 未来
//! * 固定

use crate::api::hyper_parameters::*;

/// 时间戳
#[derive(Debug, Clone, PartialEq)]
pub enum Stamp {
    /// 永恒 | 空
    Eternal,
    /// 过去 | 预测性
    Past,
    /// 现在 | 并发性
    Present,
    /// 未来 | 回顾性
    Future,
    /// 固定
    Fixed(IntPrecision),
}

// 💭无需实现特别的「构造函数」：足够简单

/// 实现/属性
impl Stamp {
    /// 是否为「永恒」
    pub fn is_eternal(&self) -> bool {
        matches!(self, Stamp::Eternal)
    }

    /// 是否为「固定时间」
    /// * 📝使用[`matches`]宏，快速判断「是否符合模式」
    pub fn is_fixed(&self) -> bool {
        matches!(self, Stamp::Fixed(_))
    }
}
