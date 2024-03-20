//! 统一定义语句相关特性

pub trait GetTruth {
    /// 获取「真值」
    fn get_truth(&self) -> &Truth;
}
