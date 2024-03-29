//! 定义抽象的「词项类别」API
//! * 🎯用于在「容纳性」（可包含的词项数目）对词项快速归类
//! * 🚩【2024-03-29 21:26:50】自「枚举Narsese」独立而来

/// 词项容量
/// * 🎯在「容纳性」（可包含的词项数目）上对词项快速分类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// 实现/偏序关系 | 通过「基数」比较
/// * 🚩基于[`Ord::cmp`]实现[`PartialOrd::partial_cmp`]
impl PartialOrd for TermCapacity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
/// 实现/全序关系 | 通过「基数」比较
impl Ord for TermCapacity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.base_num().cmp(&other.base_num())
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
    use util::asserts;

    /// 测试/全序关系
    /// * 仅仅是大体上的关系，并不能根据「大小」锁定
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
        asserts! {
            Atom == Unary // 1 = 1
            Unary < BinarySet // 1 < 2
            BinarySet == BinaryVec // 2 = 2
            BinaryVec < Set // 2 < 3
            Set == Vec // 3 = 3
        }
    }
}
