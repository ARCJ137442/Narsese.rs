//! 与「词法折叠」有关的特征

/// 用以实现「朝某个类型折叠」
/// * 🎯最初用于「词法Narsese→枚举Narsese」
pub trait FoldInto<Target> {
    /// 朝需要的类型折叠
    fn fold_into(self) -> Target;
}
