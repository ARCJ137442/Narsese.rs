//! 与「词法折叠」有关的特征
//!
//! ! ❌弃用「统一词法折叠格式」与「关键字字典」机制

/// 用以实现「尝试朝某个类型折叠」
/// * 🎯最初用于「词法Narsese→枚举Narsese」
/// * ⚠️可能会有「折叠失败」的场景
///
/// * 🚩【2024-03-20 11:26:26】设计方案：引入一个`Folder`参数
/// * 🚩必须显示引入生命周期支持：因为其中的`Folder`可能用到
///   * 📄case：枚举Narsese中的[`crate::conversion::string::impl_enum::NarseseFormat`]
pub trait TryFoldInto<'a, Target, Error> {
    /// 关联参数「折叠器」
    /// * 🎯**统一**给「词法折叠」提供信息
    ///   * 📝使用「关联参数」而非「类型参数」是为了「让所有`Target`都有一样的折叠参数」
    ///   * 🎯最初用于对不同的「枚举Narsese格式」进行适配
    ///     * 📄一套格式有一套「原子词项前缀→不同原子词项结构」
    type Folder;

    /// 尝试朝需要的类型进行「词法折叠」
    /// * ✨可根据类型推断进行方法分派，形如[`TryInto::try_into`]
    /// * 📌需要一个指定的「折叠器」提供附加信息
    /// * ⚠️可能会有折叠错误
    ///   * 📄如「非法原子词项前缀」
    fn try_fold_into(self, folder: &'a Self::Folder) -> Result<Target, Error>;
}
