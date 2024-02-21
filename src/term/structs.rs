//! 统一定义词项结构

use std::collections::HashSet;

// 定义 //

/// 统一定义「词项引用」 | 避免循环引用
pub type TermRefType = Box<Term>;
/// 统一定义「无序不重复词项容器」
pub type TermSetType = HashSet<Term>;
/// 统一定义「有序可重复词项容器」
pub type TermVecType = Vec<Term>;

/// 统一定义「词项」
/// * 自动实现[`Debug`]、[`Clone`]
#[derive(Debug, Clone)]
pub enum Term {
    // 原子词项 //
    /// 词语
    Word(String),
    /// 占位符 | 可用于构建「像」
    PlaceHolder,
    /// 独立变量
    VariableIndependent(String),
    /// 非独变量
    VariableDependent(String),
    /// 查询变量
    VariableQuery(String),
    /// 间隔
    Interval(usize),
    /// 操作符
    Operator(String),
    // 复合词项 //
    /// 外延集
    SetExtension(TermSetType),
    /// 内涵集
    SetIntension(TermSetType),
    /// 外延交
    IntersectionExtension(TermSetType),
    /// 内涵交
    IntersectionIntension(TermSetType),
    /// 外延差
    DifferenceExtension(TermRefType, TermRefType),
    /// 内涵差
    DifferenceIntension(TermRefType, TermRefType),
    /// 乘积
    Product(TermVecType),
    /// 外延像
    ImageExtension(usize, TermVecType),
    /// 内涵像
    ImageIntension(usize, TermVecType),
    /// 合取
    Conjunction(TermSetType),
    /// 析取
    Disjunction(TermSetType),
    /// 否定
    Negation(TermRefType),
    /// 顺序合取
    ConjunctionSequential(TermVecType),
    /// 平行合取
    ConjunctionParallel(TermSetType),
    // 陈述 //
    /// 继承
    Inheritance(TermRefType, TermRefType),
    /// 相似 | 暂不考虑对称性，后续判等时会优化
    Similarity(TermRefType, TermRefType),
    /// 蕴含
    Implication(TermRefType, TermRefType),
    /// 等价 | 暂不考虑对称性，后续判等时会优化
    Equivalence(TermRefType, TermRefType),
}

/// 词项类别
/// * 🎯用于对词项快速分类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TermCategory {
    /// 原子词项
    Atom,
    /// 复合词项
    Compound,
    /// 陈述
    Statement,
}

/// 词项容量
/// * 🎯用于对词项快速分类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TermCapability {
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

// 直接导出内部所有
pub use Term::*;
