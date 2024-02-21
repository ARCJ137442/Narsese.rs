//! 统一定义词项实现

use crate::GetTerm;

use super::structs::*;
use std::any::type_name;
use std::error::Error;
use std::hash::Hash;
use std::io::ErrorKind;

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
    // * 📌在`placeholder_index == vec.len()`时，相当于「占位符在最后一个」的情况
    if placeholder_index > vec.len() {
        panic!("占位符超出范围")
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
    // * 📌在`placeholder_index == vec.len()`时，相当于「占位符在最后一个」的情况
    test_term_vec_for_image(placeholder_index, &vec);
    // 返回
    vec
}

/// 实现/构造函数
impl Term {
    // 原子词项 //

    /// 构造/词语
    pub fn new_word(word: &str) -> Self {
        Word(word.to_string())
    }

    /// 构造/占位符
    pub fn new_placeholder() -> Self {
        PlaceHolder
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

    /// 实例（派生） | {S} --> P
    pub fn new_instance(subject: Term, predicate: Term) -> Self {
        Term::new_inheritance(Term::new_set_extension(vec![subject]), predicate)
    }

    /// 属性（派生） | S --> [P]
    pub fn new_property(subject: Term, predicate: Term) -> Self {
        Term::new_inheritance(subject, Term::new_set_intension(vec![predicate]))
    }

    /// 实例属性（派生） | {S} --> [P]
    pub fn new_instance_property(subject: Term, predicate: Term) -> Self {
        Term::new_inheritance(
            Term::new_set_extension(vec![subject]),
            Term::new_set_intension(vec![predicate]),
        )
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

    /// 测试合法的占位符位置
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
            // 测试所有位置的占位符
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

/// 类型判断相关
impl Term {
    // 通用 //

    /// 获取类型名称
    /// * 📝Rust使用[`std::any`]实现类似「获取类型名」的反射代码
    pub fn type_name(&self) -> &str {
        type_name::<Self>()
    }

    /// 获取词项类别
    pub fn get_category(&self) -> TermCategory {
        match self {
            // 原子词项
            Word(..)
            | PlaceHolder
            | VariableIndependent(..)
            | VariableDependent(..)
            | VariableQuery(..)
            | Interval(..)
            | Operator(..) => TermCategory::Atom,
            // 复合词项
            SetExtension(..)
            | SetIntension(..)
            | IntersectionExtension(..)
            | IntersectionIntension(..)
            | DifferenceExtension(..)
            | DifferenceIntension(..)
            | Product(..)
            | ImageExtension(..)
            | ImageIntension(..)
            | Conjunction(..)
            | Disjunction(..)
            | Negation(..)
            | ConjunctionSequential(..)
            | ConjunctionParallel(..) => TermCategory::Compound,
            // 陈述
            Inheritance(..) | Similarity(..) | Implication(..) | Equivalence(..) => {
                TermCategory::Statement
            }
        }
    }

    /// 获取词项容量
    pub fn get_capacity(&self) -> TermCapability {
        match self {
            // 原子词项
            Word(..)
            | PlaceHolder
            | VariableIndependent(..)
            | VariableDependent(..)
            | VariableQuery(..)
            | Interval(..)
            | Operator(..) => TermCapability::Atom,
            // 一元
            Negation(..) => TermCapability::Unary,
            // 二元序列
            DifferenceExtension(..)
            | DifferenceIntension(..)
            | Inheritance(..)
            | Implication(..) => TermCapability::BinaryVec,
            // 二元集合
            Similarity(..) | Equivalence(..) => TermCapability::BinarySet,
            // 序列
            Product(..) | ImageExtension(..) | ImageIntension(..) | ConjunctionSequential(..) => {
                TermCapability::Vec
            }
            // 集合
            SetExtension(..)
            | SetIntension(..)
            | IntersectionExtension(..)
            | IntersectionIntension(..)
            | Conjunction(..)
            | Disjunction(..)
            | ConjunctionParallel(..) => TermCapability::Set,
        }
    }

    // 专用 //

    /// 判型/原子词项
    /// * 1 词语
    /// * 6 独立变量
    /// * 6 非独变量
    /// * 6 查询变量
    /// * 7 间隔
    pub fn is_atom(&self) -> bool {
        self.get_category() == TermCategory::Atom
    }

    /// 判型/复合词项
    /// * 3 外延集
    /// * 3 内涵集
    /// * 3 外延交
    /// * 3 内涵交
    /// * 3 外延差
    /// * 3 内涵差
    /// * 4 乘积
    /// * 4 外延像
    /// * 4 内涵像
    /// * 5 合取
    /// * 5 析取
    /// * 5 否定
    /// * 7 顺序合取
    /// * 7 平行合取
    pub fn is_compound(&self) -> bool {
        self.get_category() == TermCategory::Compound
    }

    /// 判型/陈述
    /// * 1 继承
    /// * 2 相似
    /// * 5 蕴含
    /// * 5 等价
    pub fn is_statement(&self) -> bool {
        self.get_category() == TermCategory::Statement
    }

    /// 获取词项作为原子词项的字符串名
    /// * 🚩返回新字串，而非原字串
    /// * 🚩对「间隔」而言，会转换成字符串形式
    /// * ⚠️对**非原子词项**会**panic**
    pub fn get_atom_name_unchecked(&self) -> String {
        match self {
            Word(name)
            | VariableIndependent(name)
            | VariableDependent(name)
            | VariableQuery(name)
            | Operator(name) => name.clone(),
            // 特殊处理/占位符 ⇒ 空名
            PlaceHolder => String::new(),
            // 特殊处理/间隔 ⇒ 转换数值为字符串形式
            Interval(interval) => interval.to_string(),
            // 其他词项 ⇒ panic
            other => panic!("`{}`并非原子词项", other.type_name()),
        }
    }

    /// 获取词项作为原子词项的字符串名
    /// * 📌名称**无前缀**
    /// * 📌当词项非原子词项时，返回[`None`]
    /// * 🚩对「间隔」而言，会转换成字符串形式
    pub fn get_atom_name(&self) -> Option<String> {
        match self.is_atom() {
            true => Some(self.get_atom_name_unchecked()),
            false => None,
        }
    }

    /// 设置词项作为原子词项的词项名
    /// * ⚠️对其它情况：静默失败
    /// * ⚠️对「占位符」：静默失败
    /// * 📌对「间隔」会自动转换成数值类型
    pub fn set_atom_name(&mut self, new_name: &str) -> Result<(), impl Error> {
        match self {
            // 原子词项
            Word(name)
            | VariableIndependent(name)
            | VariableDependent(name)
            | VariableQuery(name)
            | Operator(name) => {
                // 清空重建
                name.clear();
                name.push_str(new_name);
                Ok(())
            }
            // 占位符⇒静默失败
            PlaceHolder => Ok(()),
            // 间隔⇒解析数值
            Interval(interval) => match new_name.parse() {
                Ok(new_interval) => {
                    *interval = new_interval;
                    Ok(())
                }
                // 需要转换类型
                Err(_) => Err(std::io::Error::new(
                    ErrorKind::InvalidInput,
                    "尝试在间隔中设置无效的数值",
                )),
            },
            // 其它情况：静默失败
            _ => Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "尝试在非原子词项中设置词项名",
            )),
        }
    }

    /// 获取词项作为复合词项的「所有词项」
    /// * 📌原子词项⇒返回自身
    /// * 📌陈述⇒返回主谓词
    /// * 📝Rust会自动根据返回类型，为变量加引用/解引用
    pub fn get_components(&self) -> Vec<&Term> {
        match self {
            // 原子词项⇒返回自身
            Word(..)
            | PlaceHolder
            | VariableIndependent(..)
            | VariableDependent(..)
            | VariableQuery(..)
            | Interval(..)
            | Operator(..) => vec![self],

            // 一元容器⇒返回包装后的容器
            Negation(term) => vec![term],

            // 二元容器⇒返回包装后的容器
            DifferenceExtension(term1, term2)
            | DifferenceIntension(term1, term2)
            | Inheritance(term1, term2)
            | Similarity(term1, term2)
            | Implication(term1, term2)
            | Equivalence(term1, term2) => vec![term1, term2],

            // 有序容器⇒返回拷贝后的容器
            Product(vec)
            | ImageExtension(_, vec)
            | ImageIntension(_, vec)
            | ConjunctionSequential(vec) => vec.iter().collect(),

            // 集合容器⇒返回收集后的容器
            SetExtension(set)
            | SetIntension(set)
            | IntersectionExtension(set)
            | IntersectionIntension(set)
            | Conjunction(set)
            | Disjunction(set)
            | ConjunctionParallel(set) => set.iter().collect(),
        }
    }

    /// 获取词项作为复合词项的「所有词项」
    /// * 📌仅对复合词项起效
    ///   * ⚠️其它情况返回[`None`]
    pub fn get_compound_components(&self) -> Option<Vec<&Term>> {
        match self.is_compound() {
            true => Some(self.get_components()),
            false => None,
        }
    }
}

#[test]
fn test_components() {
    let set = Term::new_set_extension(vec![Term::new_word("a"), Term::new_word("b")]);
    println!("set: {:?}", set.get_components());
    assert_eq!(set.get_components().len(), 2);
}

/// 散列化「无序不重复词项容器」
/// * ⚠️潜在假设：集合相同⇒遍历顺序相同⇒散列化顺序相同⇒散列化结果相同
fn hash_term_set<H: std::hash::Hasher>(set: &TermSetType, state: &mut H) {
    // 逐个元素散列化
    for term in set {
        term.hash(state)
    }
}

/// 实现/散列化逻辑
///
/// ?【2024-02-21 14:21:10】是否一定要实现
/// * 如「占位符」就没有「进一步散列化」的组分
impl Hash for Term {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            // 原子词项 //
            Word(word) => word.hash(state),
            PlaceHolder => "_".hash(state), // !【2024-02-21 14:21:59】目前暂时使用"_"来进行散列化
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

/// 实现/判等逻辑
impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // 原子词项 //
            (Word(word), Word(other_word)) => word == other_word,
            (PlaceHolder, PlaceHolder) => true,
            (VariableIndependent(name), VariableIndependent(other_name)) => name == other_name,
            (VariableDependent(name), VariableDependent(other_name)) => name == other_name,
            (VariableQuery(name), VariableQuery(other_name)) => name == other_name,
            (Interval(i1), Interval(i2)) => i1 == i2,
            (Operator(name), Operator(other_name)) => name == other_name,
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

/// 实现/获取词项
impl GetTerm for Term {
    fn get_term(&self) -> &Term {
        &self
    }
}

/// 单元测试
///
/// TODO: 完善
#[cfg(test)]
mod tests {
    use crate::show;

    use super::*;

    /// 测试一个普通词项
    /// * 仅测试其作为普通词项的内涵
    fn _test_term(term: Term) {
        // 类型详尽性
        assert!(term.is_atom() || term.is_compound() || term.is_statement());
        // 展示类别
        show!(term.get_category());
        // 展示容量
        show!(term.get_capacity());
    }

    /// 测试一个原子词项
    fn _test_atom(atom: Term) {
        // 确认是原子词项
        assert!(atom.is_atom());
        assert_eq!(atom.get_category(), TermCategory::Atom);
        // 并非复合词项、陈述
        assert!(!atom.is_compound());
        assert!(!atom.is_statement());
        // 获取（检查）名称
        show!(atom.get_atom_name());
        // 拷贝，并检查是否相等
        assert_eq!(atom, atom.clone());
    }

    /// 有效性测试
    #[test]
    fn test_term() {
        // 原子词项
        _test_atom(Term::new_word("word"));
        _test_atom(Term::new_placeholder());
        _test_atom(Term::new_variable_independent("i_var"));
        _test_atom(Term::new_variable_dependent("d_var"));
        _test_atom(Term::new_variable_query("q_var"));
        _test_atom(Term::new_interval(1));
        _test_atom(Term::new_operator("op"));
        // 复合词项 // TODO: 构造&完善
        // 陈述 // TODO: 构造&完善
    }
}
