//! 向「枚举Narsese」的折叠
#![allow(unused, unreachable_code)]

use super::*;
use crate::{
    api::{FromParse, IntPrecision, UIntPrecision},
    conversion::string::impl_enum::NarseseFormat as EnumNarseseFormat,
    enum_narsese::{
        Budget, Narsese as EnumNarsese, Punctuation, Sentence as EnumSentence, Stamp,
        Task as EnumTask, Term as EnumTerm, Truth,
    },
    lexical::{Narsese, Sentence, Task, Term},
};
use util::*;

/// 一个简单的「折叠错误」
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FoldError(String);
/// 简化的「折叠结果」
type FoldResult<T> = Result<T, FoldError>;

/// 批量实现「任何其它（错误）类型⇒自身类型」
/// * 🎯用于和[`Result::transform_err`]联动：`result.transform_err(FoldError::from)`
impl<T: ToString> From<T> for FoldError {
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}
/// 快捷构造宏
macro_rules! FoldError {
    ($($content:tt)*) => {
        FoldError(format!($($content)*))
    };
}

/// 实现/全体Narsese
/// * 📌一次性实现
/// * 🚩向下分派
impl<'a> TryFoldInto<'a, EnumNarsese, FoldError> for Narsese {
    /// 统一使用「枚举Narsese格式」提供信息
    type Folder = EnumNarseseFormat<&'a str>;

    fn try_fold_into(self, folder: &Self::Folder) -> FoldResult<EnumNarsese> {
        // 匹配自己并进行拆包封包
        Ok(match self {
            // 词项
            Narsese::Term(t) => EnumNarsese::Term(t.try_fold_into(folder)?),
            // 语句
            Narsese::Sentence(s) => EnumNarsese::Sentence(s.try_fold_into(folder)?),
            // 任务
            Narsese::Task(t) => EnumNarsese::Task(t.try_fold_into(folder)?),
        })
    }
}

/// 实现/词项
impl<'a> TryFoldInto<'a, EnumTerm, FoldError> for Term {
    /// 统一使用「枚举Narsese格式」提供信息
    type Folder = EnumNarseseFormat<&'a str>;

    fn try_fold_into(self, folder: &Self::Folder) -> FoldResult<EnumTerm> {
        match self {
            // 原子词项
            Term::Atom { prefix, name } => fold_atom(folder, prefix, name),
            // 复合词项
            Term::Compound { connecter, terms } => {
                fold_compound(folder, connecter, fold_terms(terms, folder)?)
            }
            // 集合词项
            Term::Set {
                left_bracket,
                terms,
                right_bracket,
                // * 📌这里需要使用`&*`进行自动转换（String -> &str）
            } => fold_set(
                folder,
                &left_bracket,
                &right_bracket,
                fold_terms(terms, folder)?,
            ),
            // 陈述
            Term::Statement {
                copula,
                subject,
                predicate,
            } => fold_statement(
                folder,
                subject.try_fold_into(folder)?,
                copula,
                predicate.try_fold_into(folder)?,
            ),
        }
    }
}

/// 子函数/折叠陈述
#[inline(always)]
fn fold_statement(
    folder: &EnumNarseseFormat<&str>,
    subject: EnumTerm,
    copula: String,
    predicate: EnumTerm,
) -> Result<EnumTerm, FoldError> {
    Ok(first! {
        (copula.eq) => (_);
        // 基础系词 //
        // 继承
        folder.statement.copula_inheritance => EnumTerm::new_inheritance(subject, predicate),
        // 相似
        folder.statement.copula_similarity => EnumTerm::new_similarity(subject, predicate),
        // 蕴含
        folder.statement.copula_implication => EnumTerm::new_implication(subject, predicate),
        // 等价
        folder.statement.copula_equivalence => EnumTerm::new_equivalence(subject, predicate),
        // 派生系词 //
        // 实例
        folder.statement.copula_instance => EnumTerm::new_instance(subject, predicate),
        // 属性
        folder.statement.copula_property => EnumTerm::new_property(subject, predicate),
        // 实例属性
        folder.statement.copula_instance_property => EnumTerm::new_instance_property(subject, predicate),
        // 预测性蕴含
        folder.statement.copula_implication_predictive => EnumTerm::new_implication_predictive(subject, predicate),
        // 并发性蕴含
        folder.statement.copula_implication_concurrent => EnumTerm::new_implication_concurrent(subject, predicate),
        // 回顾性蕴含
        folder.statement.copula_implication_retrospective => EnumTerm::new_implication_retrospective(subject, predicate),
        // 预测性等价
        folder.statement.copula_equivalence_predictive => EnumTerm::new_equivalence_predictive(subject, predicate),
        // 并发性等价
        folder.statement.copula_equivalence_concurrent => EnumTerm::new_equivalence_concurrent(subject, predicate),
        // 回顾性等价 | ⚠️会在构造时自动转换
        folder.statement.copula_equivalence_retrospective => EnumTerm::new_equivalence_retrospective(subject, predicate),
        // 未知 //
        _ => return Err(FoldError!("非法陈述系词「{copula}」")),
    })
}

/// 子函数/折叠词项数组
#[inline(always)]
fn fold_terms(terms: Vec<Term>, folder: &EnumNarseseFormat<&str>) -> FoldResult<Vec<EnumTerm>> {
    let mut enum_terms = Vec::new();
    for term_result in terms.into_iter().map(|term| term.try_fold_into(folder)) {
        // 处理每个词项的解析结果：在遇到`Err`时抛出错误
        enum_terms.push(term_result.transform_err(FoldError::from)?);
    }
    Ok(enum_terms)
}

/// 子函数/折叠集合词项
#[inline(always)]
fn fold_set(
    folder: &EnumNarseseFormat<&str>,
    left_bracket: &str,
    right_bracket: &str,
    terms: Vec<EnumTerm>,
) -> Result<EnumTerm, FoldError> {
    Ok(first! {
        ((left_bracket, right_bracket).eq) => (_);
        // NAL-3 //
        // 外延集
        &folder.compound.brackets_set_extension => EnumTerm::new_set_extension(terms),
        // 内涵集
        &folder.compound.brackets_set_intension => EnumTerm::new_set_extension(terms),
        // 未知 //
        _ => return Err(FoldError!("非法集合词项括弧组「{left_bracket} {right_bracket}」")),
    })
}

/// 子函数/折叠复合词项
#[inline(always)]
fn fold_compound(
    folder: &EnumNarseseFormat<&str>,
    connecter: String,
    terms: Vec<EnumTerm>,
) -> Result<EnumTerm, FoldError> {
    Ok(first! {
        // * ✅这里不用再怕「短的比长的先被截取」问题
        (connecter.eq) => (_);
        // NAL-3 //
        // 外延交
        folder.compound.connecter_intersection_extension => EnumTerm::new_intersection_extension(terms),
        // 内涵交
        folder.compound.connecter_intersection_intension => EnumTerm::new_intersection_intension(terms),
        // 外延差
        folder.compound.connecter_difference_extension => {
            let mut terms = terms.into_iter(); // * 📝对于「取头部元素，然后抛弃整个数组」的情况，适合用迭代器而非`get`/`remove`
            let left = terms.next().ok_or(FoldError!("在外延差中找不到左词项"))?;
            let right = terms.next().ok_or(FoldError!("在外延差中找不到右词项"))?;
            EnumTerm::new_difference_extension(left, right)
        },
        // 内涵差
        folder.compound.connecter_difference_intension => {
            let mut terms = terms.into_iter(); // * 📝对于「取头部元素，然后抛弃整个数组」的情况，适合用迭代器而非`get`/`remove`
            let left = terms.next().ok_or(FoldError!("在内涵差中找不到左词项"))?;
            let right = terms.next().ok_or(FoldError!("在内涵差中找不到右词项"))?;
            EnumTerm::new_difference_extension(left, right)
        },
        // NAL-4 //
        // 乘积
        folder.compound.connecter_product => EnumTerm::new_product(terms),
        // 外延像
        folder.compound.connecter_image_extension => EnumTerm::to_image_extension_with_placeholder(terms).ok_or(FoldError!("找不到外延像中占位符的位置"))?,
        // 内涵像
        folder.compound.connecter_image_intension => EnumTerm::to_image_intension_with_placeholder(terms).ok_or(FoldError!("找不到外延像中占位符的位置"))?,
        // NAL-5
        // 合取
        folder.compound.connecter_conjunction => EnumTerm::new_conjunction(terms),
        // 析取
        folder.compound.connecter_disjunction => EnumTerm::new_disjunction(terms),
        // 否定
        folder.compound.connecter_negation => EnumTerm::new_negation(
            // * 📝取首元素（并抛掉数组）推荐使用`.into_iter().next()`
            terms.into_iter().next().ok_or(FoldError!("在否定中找不到词项"))?
        ),
        // NAL-7 //
        // 顺序合取
        folder.compound.connecter_conjunction_sequential => EnumTerm::new_conjunction_sequential(terms),
        // 平行合取
        folder.compound.connecter_conjunction_parallel => EnumTerm::new_conjunction_parallel(terms),
        // 未知 //
        _ => return Err(FoldError!("非法复合词项连接符「{connecter}」")),
    })
}

/// 子函数/折叠原子词项
#[inline(always)]
fn fold_atom(
    folder: &EnumNarseseFormat<&str>,
    prefix: String,
    name: String,
) -> FoldResult<EnumTerm> {
    Ok(first! {
        (prefix.eq) => (_);
        // 词语 | ✅这里不用再害怕「空前缀」问题
        folder.atom.prefix_word => EnumTerm::VariableQuery(name),
        // 占位符
        folder.atom.prefix_placeholder => EnumTerm::Placeholder,
        // 独立变量
        folder.atom.prefix_variable_independent => EnumTerm::VariableIndependent(name),
        // 非独变量
        folder.atom.prefix_variable_dependent => EnumTerm::VariableDependent(name),
        // 查询变量
        folder.atom.prefix_variable_query => EnumTerm::VariableQuery(name),
        // 间隔 | ℹ️需要特别转换
        folder.atom.prefix_interval => EnumTerm::Interval(
            name
                // 解析成无符号整数
                .parse::<UIntPrecision>()
                // 转换错误并尝试解包
                .transform_err(FoldError::from)?
        ),
        folder.atom.prefix_operator => EnumTerm::VariableQuery(name),

        _ => return Err(FoldError!("非法原子词项词缀「{prefix}」")),
    })
}

/// 实现/语句
impl<'a> TryFoldInto<'a, EnumSentence, FoldError> for Sentence {
    /// 统一使用「枚举Narsese格式」提供信息
    type Folder = EnumNarseseFormat<&'a str>;

    fn try_fold_into(self, folder: &'a Self::Folder) -> FoldResult<EnumSentence> {
        // 先解析出词项
        let term = self.term.try_fold_into(folder)?;
        // 随后解析出真值
        let truth = folder
            // 解析
            .parse::<Truth>(&self.truth)
            // 尝试解包
            .transform_err(FoldError::from)?;
        // 再解析出时间戳
        let stamp = folder
            .parse::<Stamp>(&self.stamp)
            .transform_err(FoldError::from)?;
        // 解析标点
        let punctuation = folder
            .parse::<Punctuation>(&self.punctuation)
            .transform_err(FoldError::from)?;
        // 通过标点构造语句
        let sentence = EnumSentence::from_punctuation(term, punctuation, stamp, truth);
        // 返回
        Ok(sentence)
    }
}

/// 实现/任务
impl<'a> TryFoldInto<'a, EnumTask, FoldError> for Task {
    /// 统一使用「枚举Narsese格式」提供信息
    type Folder = EnumNarseseFormat<&'a str>;

    fn try_fold_into(self, folder: &'a Self::Folder) -> FoldResult<EnumTask> {
        // 先解析出预算
        let budget = folder
            // 解析
            .parse::<Budget>(self.budget.as_str())
            // 尝试解包
            .transform_err(FoldError::from)?;
        // 组装语句
        let sentence = self.sentence.try_fold_into(folder)?;
        // 返回
        Ok(EnumTask::new(sentence, budget))
    }
}

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        conversion::string::impl_enum::format_instances::*, lexical::tests::_sample_task_ascii,
    };

    fn _test_fold_narsese(
        format: &EnumNarseseFormat<&str>,
        lexical_narsese: Narsese,
    ) -> EnumNarsese {
        // 词法折叠
        let narsese_enum = lexical_narsese
            .try_fold_into(format)
            .expect("词法折叠失败！");
        // 格式化@枚举
        let formatted = format.format_narsese(&narsese_enum);
        // 解析@枚举
        let narsese_enum_2 = format.parse(&formatted).unwrap();
        // 「从枚举Narsese解析的」，应该与「从词法Narsese解析的」一致
        assert_eq!(narsese_enum, narsese_enum_2);
        // 返回折叠后的Narsese
        narsese_enum
    }

    /// 测试/综合
    #[test]
    fn test_fold() {
        let task = _sample_task_ascii();
        let folder = &FORMAT_ASCII;
        dbg!(_test_fold_narsese(folder, Narsese::Task(task)));
    }
}
