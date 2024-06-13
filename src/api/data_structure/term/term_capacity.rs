//! 定义抽象的「词项类别」API
//! * 🎯用于在「容纳性」（可包含的词项数目）对词项快速归类
//! * 🚩【2024-03-29 21:26:50】自「枚举Narsese」独立而来

/// 词项容量
/// * 🎯在「容纳性」（可包含的词项数目）上对词项快速分类
/// * 📌排序
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TermCapacity {
    /// 原子
    Atom,
    /// 一元
    Unary,
    /// 二元序列
    BinaryVec,
    /// 二元集合
    BinarySet,
    /// （多元）序列
    Vec,
    /// （多元）集合
    Set,
}
// 模块内导出以便快捷使用
use TermCapacity::*;

/// 实现/基础功能
impl TermCapacity {
    /// 获取这些「容量」枚举的「基数」
    /// * 🚩原子 = 一元 = 1
    /// * 🚩二元序列 = 二元集合 = 2
    /// * 🚩（多元）序列 = （多元）集合 = 3
    pub fn base_num(&self) -> usize {
        match self {
            Atom | Unary => 1,
            BinaryVec | BinarySet => 2,
            Vec | Set => 3,
        }
    }
}

/// 特征「获取词项容量」
/// * ⚠️仅考察「潜在容量」而非「实际容量」
///   * 📌仅考察词项「可能容纳的大小」，不考察「词项实际容纳的多少」
///   * 📄即便「只装了一个元素的外延集」也是「（多元）集合」
/// * ⚠️如实反映词项在**数据结构**上的容纳模式，而非「语法层次的容纳模式」
///   * 📌仅考察词项「逻辑上存取的方式」，不考察「语法解析中使用的方式」
///   * 📄即便词法Narsese集合「大多会被折叠成真正的『集合』」，它存取上直接使用[`Vec`]，那就是「（多元）序列」
pub trait GetCapacity {
    /// 获取词项的「容量」属性
    fn get_capacity(&self) -> TermCapacity;

    /// 在容量上是否为「原子」
    #[inline]
    fn is_capacity_atom(&self) -> bool {
        self.get_capacity() == Atom
    }

    /// 在容量上是否为「一元复合」
    #[inline]
    fn is_capacity_unary(&self) -> bool {
        self.get_capacity() == Unary
    }

    /// 在容量上是否为「二元」
    #[inline]
    fn is_capacity_binary(&self) -> bool {
        matches!(self.get_capacity(), BinaryVec | BinarySet)
    }

    /// 在容量上是否为「二元序列」
    #[inline]
    fn is_capacity_binary_vec(&self) -> bool {
        self.get_capacity() == BinaryVec
    }

    /// 在容量上是否为「二元集合」
    #[inline]
    fn is_capacity_binary_set(&self) -> bool {
        self.get_capacity() == BinarySet
    }

    /// 在容量上是否为「多元」
    #[inline]
    fn is_capacity_multi(&self) -> bool {
        matches!(self.get_capacity(), Vec | Set)
    }

    /// 在容量上是否为「（多元）序列」
    #[inline]
    fn is_capacity_vec(&self) -> bool {
        self.get_capacity() == Vec
    }

    /// 在容量上是否为「（多元）集合」
    #[inline]
    fn is_capacity_set(&self) -> bool {
        self.get_capacity() == Set
    }
}

/// 单元测试
#[cfg(test)]
mod test {
    use super::*;
    use nar_dev_utils::{asserts, for_in_ifs};

    /// 测试/全序关系
    /// * 仅仅是大体上的关系，并不能根据「大小」锁定
    /// * 📌确保「基数小⇒整体小」
    #[test]
    fn test_ord() {
        // 基数
        asserts! {
            Atom.base_num() => 1
            Unary.base_num() => 1
            BinarySet.base_num() => 2
            BinaryVec.base_num() => 2
            Set.base_num() => 3
            Vec.base_num() => 3
        }
        // 大小
        // ! 不能直接使用运算符，特别指定`.base_num()`
        asserts! {
            Atom.base_num() == Unary.base_num() // 1 `=` 1
            BinarySet.base_num() == BinaryVec.base_num() // 2 `=` 2
            Set.base_num() == Vec.base_num() // 3 `=` 3
            Unary.base_num() < BinarySet.base_num() // 1 < 2
            BinaryVec.base_num() < Set.base_num() // 2 < 3
        }

        // 测试「保向性」：基数小⇒自身小
        let types = [Atom, Unary, BinarySet, BinaryVec, Set, Vec];
        // 用类似Python列表生成式的方式遍历测试用例
        for_in_ifs! {
            // 要么基数不小，要么自身小
            {assert!(x.base_num() >= y.base_num() || x < y)}
            for x in (types)
            for y in (types)
        }
    }
}
