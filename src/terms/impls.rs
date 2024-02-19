//! 统一定义词项实现

use crate::terms::structs::*;
use std::hash::Hash;

// 实现 //

/// 统一创建「词项引用」
/// * ⚠️需要获得内部词项的所有权
pub fn new_term_ref_type(term: Term) -> TermRefType {
    Box::new(term)
}

/// 统一创建空「无序不重复词项容器」
pub fn new_term_set_type() -> TermSetType {
    TermSetType::new()
}

/// 统一创建空「有序可重复词项容器」
pub fn new_term_vec_type() -> TermVecType {
    TermVecType::new()
}

/* /// 可从中遍历词项的接口
/// * 🎯用于通用化构造「词项容器」
///
/// ! ⚠️弃用：`impl Trait` in type aliases is unstable
///   * 🔗issue #63063 <https://github.com/rust-lang/rust/issues/63063>
pub type TermSettable = impl IntoIterator<Item = Term>; */

/// 通用：从各大容器中构造词项集
fn from_term_settable_to_term_set(settable: impl IntoIterator<Item = Term>) -> TermSetType {
    // 创建
    let mut set = new_term_set_type();
    // 添加
    for term in settable {
        set.insert(term);
    }
    // 返回
    set
}

/// 通用：从各大容器中构造词项序列
fn from_term_settable_to_term_vec(settable: impl IntoIterator<Item = Term>) -> TermVecType {
    // 创建
    let mut vec = new_term_vec_type();
    // 添加
    for term in settable {
        vec.push(term);
    }
    // 返回
    vec
}

/// 在像中测试像索引
/// * ⚠️若不合法，则panic
fn test_term_vec_for_image(placeholder_index: usize, vec: &TermVecType) {
    // 检查 | 判断索引是否越界
    // * 📌在`placeholder_index == vec.len()`时，相当于「像占位符在最后一个」的情况
    if placeholder_index > vec.len() {
        panic!("placeholder index out of range")
    }
}

/// 创造一个合法的像与索引
fn new_term_vec_for_image(
    placeholder_index: usize,
    terms: impl IntoIterator<Item = Term>,
) -> TermVecType {
    // 创建
    let vec = from_term_settable_to_term_vec(terms);
    // 检查 | 判断索引是否越界
    // * 📌在`placeholder_index == vec.len()`时，相当于「像占位符在最后一个」的情况
    test_term_vec_for_image(placeholder_index, &vec);
    // 返回
    vec
}

// 导出其中所有的枚举项
use Term::*;

/// 实现/构造函数
impl Term {
    // 原子词项 //

    /// 构造/词语
    pub fn new_word(word: &str) -> Self {
        Word(word.to_string())
    }

    /// 构造/独立变量
    pub fn new_variable_independent(name: &str) -> Self {
        VariableIndependent(name.to_string())
    }

    /// 构造/非独变量
    pub fn new_variable_dependent(name: &str) -> Self {
        VariableDependent(name.to_string())
    }

    /// 构造/查询变量
    pub fn new_variable_query(name: &str) -> Self {
        VariableQuery(name.to_string())
    }

    /// 构造/间隔
    pub fn new_interval(interval: usize) -> Self {
        Interval(interval)
    }

    /// 构造/操作符
    pub fn new_operator(operator: &str) -> Self {
        Operator(operator.to_string())
    }

    // 复合词项 //

    /// 构造/外延集
    pub fn new_set_extension(terms: impl IntoIterator<Item = Term>) -> Self {
        SetExtension(from_term_settable_to_term_set(terms))
    }

    /// 构造/内涵集
    pub fn new_set_intension(terms: impl IntoIterator<Item = Term>) -> Self {
        SetIntension(from_term_settable_to_term_set(terms))
    }

    /// 构造/外延交
    pub fn new_intersection_extension(terms: impl IntoIterator<Item = Term>) -> Self {
        IntersectionExtension(from_term_settable_to_term_set(terms))
    }

    /// 构造/内涵交
    pub fn new_intersection_intension(terms: impl IntoIterator<Item = Term>) -> Self {
        IntersectionIntension(from_term_settable_to_term_set(terms))
    }

    /// 构造/外延差
    pub fn new_difference_extension(left: Term, right: Term) -> Self {
        DifferenceExtension(new_term_ref_type(left), new_term_ref_type(right))
    }

    /// 构造/内涵差
    pub fn new_difference_intension(left: Term, right: Term) -> Self {
        DifferenceIntension(new_term_ref_type(left), new_term_ref_type(right))
    }

    /// 构造/乘积
    pub fn new_product(terms: impl IntoIterator<Item = Term>) -> Self {
        Product(from_term_settable_to_term_vec(terms))
    }

    /// 构造/外延像
    pub fn new_image_extension(
        placeholder_index: usize,
        terms: impl IntoIterator<Item = Term>,
    ) -> Self {
        ImageExtension(
            placeholder_index,
            new_term_vec_for_image(placeholder_index, terms),
        )
    }

    /// 构造/内涵像
    pub fn new_image_intension(
        placeholder_index: usize,
        terms: impl IntoIterator<Item = Term>,
    ) -> Self {
        ImageIntension(
            placeholder_index,
            new_term_vec_for_image(placeholder_index, terms),
        )
    }

    /// 构造/合取
    pub fn new_conjunction(terms: impl IntoIterator<Item = Term>) -> Self {
        Conjunction(from_term_settable_to_term_set(terms))
    }

    /// 构造/析取
    pub fn new_disjunction(terms: impl IntoIterator<Item = Term>) -> Self {
        Disjunction(from_term_settable_to_term_set(terms))
    }

    /// 构造/否定
    pub fn new_negation(term: Term) -> Self {
        Negation(new_term_ref_type(term))
    }

    /// 构造/顺序合取
    pub fn new_conjunction_sequential(terms: impl IntoIterator<Item = Term>) -> Self {
        ConjunctionSequential(from_term_settable_to_term_vec(terms))
    }

    /// 构造/平行合取
    pub fn new_conjunction_parallel(terms: impl IntoIterator<Item = Term>) -> Self {
        ConjunctionParallel(from_term_settable_to_term_set(terms))
    }

    // 陈述 //

    /// 继承
    pub fn new_inheritance(subject: Term, predicate: Term) -> Self {
        Inheritance(new_term_ref_type(subject), new_term_ref_type(predicate))
    }

    /// 相似
    pub fn new_similarity(subject: Term, predicate: Term) -> Self {
        Similarity(new_term_ref_type(subject), new_term_ref_type(predicate))
    }

    /// 蕴含
    pub fn new_implication(subject: Term, predicate: Term) -> Self {
        Implication(new_term_ref_type(subject), new_term_ref_type(predicate))
    }

    /// 等价
    pub fn new_equivalence(subject: Term, predicate: Term) -> Self {
        Equivalence(new_term_ref_type(subject), new_term_ref_type(predicate))
    }
}

/// 单元测试/构造
#[cfg(test)]
mod test_new {
    use std::vec;

    use super::*;

    /// 辅助函数：传入构造好的词项，并打印
    fn _universal(term: &Term) {
        println!("term: {term:?}");
    }

    #[test]
    fn atoms() {
        _universal(&Term::new_word("word"));
        _universal(&Term::new_variable_independent("independent"));
        _universal(&Term::new_variable_dependent("dependent"));
        _universal(&Term::new_variable_query("query"));
        _universal(&Term::new_interval(42));
        _universal(&Term::new_operator("op"));
    }

    #[test]
    fn compound() {
        let a = Term::new_word("A");
        let b: Term = Term::new_word("B");
        let ab = vec![a.clone(), b.clone()];
        let a_c = || a.clone();
        let b_c = || b.clone();
        let ab_c = || ab.clone();

        // 外延集
        _universal(&Term::new_set_extension(ab_c()));
        // 内涵集
        _universal(&Term::new_set_intension(ab_c()));
        // 外延交
        _universal(&Term::new_intersection_extension(ab_c()));
        // 内涵交
        _universal(&Term::new_intersection_intension(ab_c()));
        // 外延差
        _universal(&Term::new_difference_extension(a_c(), b_c()));
        // 内涵差
        _universal(&Term::new_difference_intension(a_c(), b_c()));
        // 积
        _universal(&Term::new_product(ab_c()));
        // 外延像
        _universal(&Term::new_image_extension(0, ab_c()));
        // 内涵像
        _universal(&Term::new_image_intension(2, ab_c()));
        // 合取
        _universal(&Term::new_conjunction(ab_c()));
        // 析取
        _universal(&Term::new_disjunction(ab_c()));
        // 否定
        _universal(&Term::new_negation(a_c()));
        // 顺序合取
        _universal(&Term::new_conjunction_sequential(ab_c()));
        // 平行合取
        _universal(&Term::new_conjunction_parallel(ab_c()));
    }

    #[test]
    fn statement() {
        let a = Term::new_word("A");
        let b: Term = Term::new_word("B");
        let a_c = || a.clone();
        let b_c = || b.clone();

        // 继承
        _universal(&Term::new_inheritance(a_c(), b_c()));
        // 相似
        _universal(&Term::new_inheritance(a_c(), b_c()));
        // 蕴含
        _universal(&Term::new_similarity(a_c(), b_c()));
        // 等价
        _universal(&Term::new_equivalence(a_c(), b_c()));
    }

    /// 测试合法的像占位符位置
    /// * 复杂度：O(N²)
    #[test]
    fn valid_image() {
        let x = Term::new_word("");
        // 在一个基础的长度中测试
        const N: usize = 10000;
        for len in 1..(N + 1) {
            // 构造一个长度为L的词项数组
            let mut vec: TermVecType = vec![];
            // 添加L个元素
            for _ in 0..len {
                vec.push(x.clone());
            }
            assert_eq!(vec.len(), len);
            // 测试所有位置的像占位符
            for i in 0..(len + 1) {
                test_term_vec_for_image(i, &vec);
            }
        }
    }

    #[test]
    #[should_panic]
    fn invalid_image_1() {
        // 均超过索引
        new_term_vec_for_image(1, vec![]);
    }

    #[test]
    #[should_panic]
    fn invalid_image_2() {
        // 均超过索引
        new_term_vec_for_image(2, vec![Term::new_word("")]);
    }
}

/// 散列化「无序不重复词项容器」
/// * ⚠️潜在假设：集合相同⇒遍历顺序相同⇒散列化顺序相同⇒散列化结果相同
fn hash_term_set<H: std::hash::Hasher>(set: &TermSetType, state: &mut H) {
    // 逐个元素散列化
    for term in set {
        term.hash(state)
    }
}

/// 散列化逻辑
impl Hash for Term {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            // 原子词项 //
            Word(word) => word.hash(state),
            VariableIndependent(name) => name.hash(state),
            VariableDependent(name) => name.hash(state),
            VariableQuery(name) => name.hash(state),
            Interval(i) => i.hash(state),
            Operator(name) => name.hash(state),
            // 复合词项
            SetExtension(set) => hash_term_set(set, state),
            SetIntension(set) => hash_term_set(set, state),
            IntersectionExtension(set) => hash_term_set(set, state),
            IntersectionIntension(set) => hash_term_set(set, state),
            DifferenceExtension(t1, t2) => {
                t1.hash(state);
                t2.hash(state);
            }
            DifferenceIntension(t1, t2) => {
                t1.hash(state);
                t2.hash(state);
            }
            Product(terms) => {
                for term in terms {
                    term.hash(state);
                }
            }
            ImageExtension(i, terms) => {
                i.hash(state);
                for term in terms {
                    term.hash(state);
                }
            }
            ImageIntension(i, terms) => {
                i.hash(state);
                for term in terms {
                    term.hash(state);
                }
            }
            Conjunction(set) => hash_term_set(set, state),
            Disjunction(set) => hash_term_set(set, state),
            Negation(t) => t.hash(state),
            ConjunctionSequential(terms) => {
                for term in terms {
                    term.hash(state);
                }
            }
            ConjunctionParallel(set) => hash_term_set(set, state),
            // 陈述
            Inheritance(t1, t2) => {
                t1.hash(state);
                t2.hash(state);
            }
            Similarity(t1, t2) => {
                t1.hash(state);
                t2.hash(state);
            }
            Implication(t1, t2) => {
                t1.hash(state);
                t2.hash(state);
            }
            Equivalence(t1, t2) => {
                t1.hash(state);
                t2.hash(state);
            }
        }
    }
}

/// 判等逻辑
impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // 原子词项 //
            (Word(word), Word(other_word)) => word == other_word,
            (VariableIndependent(name), VariableIndependent(other_name)) => name == other_name,
            (VariableDependent(name), VariableDependent(other_name)) => name == other_name,
            (VariableQuery(name), VariableQuery(other_name)) => name == other_name,
            (Interval(i1), Interval(i2)) => i1 == i2,
            // 复合词项 //
            (SetExtension(s1), SetExtension(s2)) => s1 == s2,
            (SetIntension(s1), SetIntension(s2)) => s1 == s2,
            (IntersectionExtension(s1), IntersectionExtension(s2)) => s1 == s2,
            (IntersectionIntension(s1), IntersectionIntension(s2)) => s1 == s2,
            (DifferenceExtension(t1, t2), DifferenceExtension(u1, u2)) => t1 == u1 && t2 == u2,
            (DifferenceIntension(t1, t2), DifferenceIntension(u1, u2)) => t1 == u1 && t2 == u2,
            (Product(terms1), Product(terms2)) => terms1 == terms2,
            (ImageExtension(i1, terms1), ImageExtension(i2, terms2)) => {
                i1 == i2 && terms1 == terms2
            }
            (ImageIntension(i1, terms1), ImageIntension(i2, terms2)) => {
                i1 == i2 && terms1 == terms2
            }
            (Conjunction(set1), Conjunction(set2)) => set1 == set2,
            (Disjunction(set1), Disjunction(set2)) => set1 == set2,
            (Negation(t1), Negation(t2)) => t1 == t2,
            (ConjunctionSequential(terms1), ConjunctionSequential(terms2)) => terms1 == terms2,
            (ConjunctionParallel(set1), ConjunctionParallel(set2)) => set1 == set2,
            // 陈述
            (Inheritance(t1, t2), Inheritance(u1, u2)) => t1 == u1 && t2 == u2,
            (Similarity(t1, t2), Similarity(u1, u2)) => {
                // 📌对称：反过来也相等
                (t1 == u1 && t2 == u2) || (t1 == u2 && t2 == u1)
            }
            (Implication(t1, t2), Implication(u1, u2)) => t1 == u1 && t2 == u2,
            (Equivalence(t1, t2), Equivalence(u1, u2)) => {
                // 📌对称：反过来也相等
                (t1 == u1 && t2 == u2) || (t1 == u2 && t2 == u1)
            }
            // 其它⇒默认不等 //
            _ => false,
        }
    }
}
/// 实现全相等
impl Eq for Term {}
