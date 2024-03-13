//! 统一定义词项实现

use crate::api::GetTerm;

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
        Placeholder
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

    /// 预测性蕴含 | A =/> C
    pub fn new_implication_predictive(antecedent: Term, consequent: Term) -> Self {
        Term::ImplicationPredictive(new_term_ref_type(antecedent), new_term_ref_type(consequent))
    }

    /// 并发性蕴含 | A =|> C
    pub fn new_implication_concurrent(antecedent: Term, consequent: Term) -> Self {
        Term::ImplicationConcurrent(new_term_ref_type(antecedent), new_term_ref_type(consequent))
    }

    /// 回顾性蕴含 | A =\> C
    pub fn new_implication_retrospective(antecedent: Term, consequent: Term) -> Self {
        Term::ImplicationRetrospective(new_term_ref_type(antecedent), new_term_ref_type(consequent))
    }

    /// 预测性等价 | A </> C
    pub fn new_equivalence_predictive(antecedent: Term, consequent: Term) -> Self {
        Term::EquivalencePredictive(new_term_ref_type(antecedent), new_term_ref_type(consequent))
    }

    /// 并发性等价 | A <|> C
    pub fn new_equivalence_concurrent(antecedent: Term, consequent: Term) -> Self {
        Term::EquivalenceConcurrent(new_term_ref_type(antecedent), new_term_ref_type(consequent))
    }

    /// 回顾性等价 | A <\> C
    /// * ⚠️自动转换成「预测性等价」
    ///   * 转换后形式：`C <\> A`
    pub fn new_equivalence_retrospective(antecedent: Term, consequent: Term) -> Self {
        Term::new_equivalence_predictive(consequent, antecedent)
    }

    // 特殊初始化 //

    /// 工具函数/像：伴随占位符的初始化
    /// * 🚩找到并消耗第一个占位符，并将其用作「占位符位置」
    /// * 📝特征[`IntoIterator`]不直接支持`enumerate`方法
    ///   * 需要先使用[`IntoIterator::into_iter`]进行转换
    ///   * 或使用[`Iterator`]规避所有权问题（若需对自身进行处理）
    /// * 🎯用于解析器处「统一构建复合词项」
    pub fn to_terms_with_image(
        terms: impl IntoIterator<Item = Term>,
        target: &mut Vec<Term>, // ? 是否直接使用数组，以便提升性能
    ) -> Option<usize> {
        let mut placeholder_index = None;
        // 顺序遍历
        for (i, term) in terms.into_iter().enumerate() {
            match (&term, placeholder_index) {
                (Term::Placeholder, None) => {
                    // 置入（忽略返回值）
                    let _ = placeholder_index.insert(i);
                }
                _ => target.push(term),
            }
        }
        // 根据「是否有占位符位置」产生结果（实际上直接返回）
        // * 📝Rust可以直接对[`Option`]进行map，其中[`None`]会保留原样
        placeholder_index
    }

    /// 从「带有占位符的词项迭代器」中直接构建「外延像」
    /// * 📌可能失败（无占位符时）
    ///   * 此时返回[`None`]
    pub fn to_image_extension_with_placeholder(
        terms: impl IntoIterator<Item = Term>,
    ) -> Option<Term> {
        // 解析出词项和索引 | 可能失败，使用`?`传递`None`
        let mut vec = vec![];
        let placeholder_index = Term::to_terms_with_image(terms, &mut vec)?;
        // 返回成功的结果
        Some(Term::new_image_extension(placeholder_index, vec))
    }

    /// 从「带有占位符的词项迭代器」中直接构建「内涵像」
    /// * 📌可能失败（无占位符时）
    ///   * 此时返回[`None`]
    pub fn to_image_intension_with_placeholder(
        terms: impl IntoIterator<Item = Term>,
    ) -> Option<Term> {
        // 解析出词项和索引 | 可能失败，使用`?`传递`None`
        let mut vec = vec![];
        let placeholder_index = Term::to_terms_with_image(terms, &mut vec)?;
        // 返回成功的结果
        Some(Term::new_image_intension(placeholder_index, vec))
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
            | Placeholder
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
            Inheritance(..)
            | Similarity(..)
            | Implication(..)
            | Equivalence(..)
            | ImplicationPredictive(..)
            | ImplicationConcurrent(..)
            | ImplicationRetrospective(..)
            | EquivalencePredictive(..)
            | EquivalenceConcurrent(..) => TermCategory::Statement,
        }
    }

    /// 获取词项容量
    pub fn get_capacity(&self) -> TermCapacity {
        match self {
            // 原子词项
            Word(..)
            | Placeholder
            | VariableIndependent(..)
            | VariableDependent(..)
            | VariableQuery(..)
            | Interval(..)
            | Operator(..) => TermCapacity::Atom,
            // 一元
            Negation(..) => TermCapacity::Unary,
            // 二元序列
            DifferenceExtension(..)
            | DifferenceIntension(..)
            | Inheritance(..)
            | Implication(..)
            | ImplicationPredictive(..)
            | ImplicationConcurrent(..)
            | ImplicationRetrospective(..)
            | EquivalencePredictive(..) => TermCapacity::BinaryVec,
            // 二元集合
            Similarity(..) | Equivalence(..) | EquivalenceConcurrent(..) => TermCapacity::BinarySet,
            // 序列
            Product(..) | ImageExtension(..) | ImageIntension(..) | ConjunctionSequential(..) => {
                TermCapacity::Vec
            }
            // 集合
            SetExtension(..)
            | SetIntension(..)
            | IntersectionExtension(..)
            | IntersectionIntension(..)
            | Conjunction(..)
            | Disjunction(..)
            | ConjunctionParallel(..) => TermCapacity::Set,
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
            Placeholder => String::new(),
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
            Placeholder => Ok(()),
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
            | Placeholder
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
            | Equivalence(term1, term2)
            | ImplicationPredictive(term1, term2)
            | ImplicationConcurrent(term1, term2)
            | ImplicationRetrospective(term1, term2)
            | EquivalencePredictive(term1, term2)
            | EquivalenceConcurrent(term1, term2) => vec![term1, term2],

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

/// 实现/修改
impl Term {
    /// 复合词项：向组分中追加词项
    /// * 📌原子词项|陈述|一元复合词项|二元复合词项⇒失败
    /// * 📌陈述⇒返回主谓词
    /// * 📌复合词项⇒追加词项
    /// * ⚠️对「像」不做特殊处理
    /// * 📝Rust使用[`Extend::extend`]方法批量自迭代器向追加元素
    pub fn push_components(
        &mut self,
        terms: impl IntoIterator<Item = Term>,
    ) -> Result<(), std::io::Error> {
        match self.get_capacity() {
            // 原子|一元|二元⇒失败
            TermCapacity::Atom|
            // ⇒失败
            TermCapacity::Unary|
            // 二元序列
            TermCapacity::BinaryVec|
            // 二元集合
            TermCapacity::BinarySet=>Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "尝试为容量固定的词项添加词项",
            )),
            // 多元词项
            _ =>match  self {
                // 序列 | 忽略「像」的占位符位置
                Product(vec) | ImageExtension(_,vec) | ImageIntension(_,vec) | ConjunctionSequential(vec) => {
                    // 持续追加
                    vec.extend(terms);
                    Ok(())
                },
                // 集合
                SetExtension(set)
                | SetIntension(set)
                | IntersectionExtension(set)
                | IntersectionIntension(set)
                | Conjunction(set)
                | Disjunction(set)
                | ConjunctionParallel(set) => {
                    set.extend(terms);
                    Ok(())
                },
                // 其它⇒未知类型报错
                _ => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "未定义的多元复合词项",
                ))
            },
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
            Placeholder => "_".hash(state), // !【2024-02-21 14:21:59】目前暂时使用"_"来进行散列化
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
            Inheritance(t1, t2)
            | Similarity(t1, t2)
            | Implication(t1, t2)
            | Equivalence(t1, t2)
            | ImplicationPredictive(t1, t2)
            | ImplicationConcurrent(t1, t2)
            | ImplicationRetrospective(t1, t2)
            | EquivalencePredictive(t1, t2)
            | EquivalenceConcurrent(t1, t2) => {
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
            (Placeholder, Placeholder) => true,
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
impl GetTerm<Term> for Term {
    fn get_term(&self) -> &Term {
        self
    }
}

/// 实现/专用/像迭代器
/// * 🎯初次用于统一「复合词项の迭代」与「像の迭代」：自动迭代出「占位符」
/// * 🎯也用于迭代「像」词项（词法上迭代出「占位符」）
/// * 📝此中使用泛型参数，将类型变得更通用更宽泛
pub struct ImageIterator<'a, I: Iterator<Item = &'a Term>> {
    raw_components: I,
    now_index: usize,
    placeholder_index: usize,
}

impl<'a, I> ImageIterator<'a, I>
where
    I: Iterator<Item = &'a Term>,
{
    pub fn new(raw_components: I, placeholder_index: usize) -> Self {
        Self {
            raw_components,
            now_index: 0,
            placeholder_index,
        }
    }
}

/// 实现：在「『当前索引』到达『占位符索引』」时返回占位符
/// * 🚩细节：避免创建临时变量
impl<'a, I> Iterator for ImageIterator<'a, I>
where
    I: Iterator<Item = &'a Term>,
{
    type Item = &'a Term;

    fn next(&mut self) -> Option<Self::Item> {
        // 检查是否到了「占位符位置」
        match self.now_index == self.placeholder_index {
            // 若至⇒返回占位符（引用）
            true => {
                self.now_index += 1;
                Some(&Placeholder)
            }
            // 未至⇒继续使用迭代器
            false => {
                self.now_index += 1;
                self.raw_components.next()
            }
        }
    }
}

/// 单元测试 | 构造
#[cfg(test)]
mod tests {
    use crate::show;

    use super::*;

    /// 【通用】生成一个「词项测试集」
    /// * 所有类型的词项均生成一遍
    pub fn generate_term_testset() -> Vec<Term> {
        // 这俩用来做复合词项组分
        let a = Term::new_word("A");
        let b = Term::new_word("B");
        // 直接返回一个数组
        vec![
            // 原子词项
            Term::new_word("word"),
            Term::new_placeholder(),
            Term::new_variable_independent("i_var"),
            Term::new_variable_dependent("d_var"),
            Term::new_variable_query("q_var"),
            Term::new_interval(1),
            Term::new_operator("op"),
            // 复合词项
            Term::new_set_extension(vec![a.clone(), b.clone()]),
            Term::new_set_intension(vec![a.clone(), b.clone()]),
            Term::new_intersection_extension(vec![a.clone(), b.clone()]),
            Term::new_intersection_intension(vec![a.clone(), b.clone()]),
            Term::new_difference_extension(a.clone(), b.clone()),
            Term::new_difference_intension(a.clone(), b.clone()),
            Term::new_product(vec![a.clone(), b.clone()]),
            Term::new_image_extension(1, vec![a.clone(), b.clone()]),
            Term::new_image_intension(0, vec![a.clone(), b.clone()]),
            Term::new_conjunction(vec![a.clone(), b.clone()]),
            Term::new_disjunction(vec![a.clone(), b.clone()]),
            Term::new_negation(a.clone()),
            Term::new_conjunction_sequential(vec![a.clone(), b.clone()]),
            Term::new_conjunction_parallel(vec![a.clone(), b.clone()]),
            // 陈述
            Term::new_inheritance(a.clone(), b.clone()),
            Term::new_similarity(a.clone(), b.clone()),
            Term::new_implication(a.clone(), b.clone()),
            Term::new_equivalence(a.clone(), b.clone()),
            Term::new_implication_predictive(a.clone(), b.clone()),
            Term::new_implication_concurrent(a.clone(), b.clone()),
            Term::new_implication_retrospective(a.clone(), b.clone()),
            Term::new_equivalence_predictive(a.clone(), b.clone()),
            Term::new_equivalence_concurrent(a.clone(), b.clone()),
        ]
    }

    /// 测试一个普通词项
    /// * 仅测试其作为普通词项的内涵
    fn _test_term(term: &Term) {
        // 类型详尽性
        assert!(term.is_atom() || term.is_compound() || term.is_statement());
        // 展示类别
        show!(term.get_category());
        // 展示容量
        show!(term.get_capacity());
    }

    /// 测试一个原子词项
    fn _test_atom(atom: Term) {
        // 首先得是一个词项
        _test_term(&atom);
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

    /// 测试一个复合词项
    fn _test_compound(compound: Term) {
        // 首先得是一个词项
        _test_term(&compound);
        // 确认是原子词项
        assert!(compound.is_compound());
        assert_eq!(compound.get_category(), TermCategory::Compound);
        // 并非原子词项、陈述
        assert!(!compound.is_atom());
        assert!(!compound.is_statement());
        // 获取（检查）内容
        show!(compound.get_compound_components());
        // 拷贝，并检查是否相等
        assert_eq!(compound, compound.clone());
    }

    /// 测试一个陈述
    fn _test_statement(statement: Term) {
        // 首先得是一个词项
        _test_term(&statement);
        // 确认是陈述
        assert!(statement.is_statement());
        assert_eq!(statement.get_category(), TermCategory::Statement);
        // 并非原子词项、复合词项
        assert!(!statement.is_atom());
        assert!(!statement.is_compound());
        // 获取（检查）内容
        show!(statement.get_components());
        // 拷贝，并检查是否相等
    }

    /// 有效性测试
    #[test]
    fn test_term() {
        // 生成测试集
        let testset = generate_term_testset();
        // 遍历测试集
        for term in testset {
            // 分类别测试
            match term.get_category() {
                TermCategory::Atom => _test_atom(term),
                TermCategory::Compound => _test_compound(term),
                TermCategory::Statement => _test_statement(term),
            }
        }
    }
}
