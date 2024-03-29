//! 定义抽象的「词项类别」API
//! * 🎯用于在「总体类别」上快速分类词项
//!   * 📌同时作为**区分「原子」「复合」「陈述」的标准属性**
//! * 🚩【2024-03-29 21:26:50】自「枚举Narsese」独立而来
//! * 📌【2024-03-29 21:29:34】仍然遵循「原子」「复合」「陈述」的框架

/// 词项类别
/// * 🎯用于对词项快速分类
/// * 🚩【2024-03-29 21:46:36】移除对「偏序/全序」的派生宏：**「类别」不具有可比性**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TermCategory {
    /// 原子词项
    Atom,
    /// 复合词项
    Compound,
    /// 陈述
    Statement,
}
// 模块内导出以便快捷使用
use TermCategory::*;

/// 特征「获取词项类别」
/// * 🎯作为**区分「原子」「复合」「陈述」的标准属性**实现
pub trait GetCategory {
    /// 获取词项的「类别」属性
    fn get_category(&self) -> TermCategory;

    /// （在类别上）是否为「原子」
    #[inline]
    fn is_atom(&self) -> bool {
        self.get_category() == Atom
    }

    /// （在类别上）是否为「复合词项」
    #[inline]
    fn is_compound(&self) -> bool {
        self.get_category() == Compound
    }

    /// （在类别上）是否为「陈述」
    #[inline]
    fn is_statement(&self) -> bool {
        self.get_category() == Statement
    }
}
