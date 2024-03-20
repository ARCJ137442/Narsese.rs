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
        &folder.compound.brackets_set_intension => EnumTerm::new_set_intension(terms),
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
        folder.atom.prefix_word => EnumTerm::Word(name),
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
        folder.atom.prefix_operator => EnumTerm::Operator(name),

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
        conversion::string::{
            impl_enum::format_instances::*,
            impl_lexical::{
                format_instances::{
                    FORMAT_ASCII as L_ASCII, FORMAT_HAN as L_HAN, FORMAT_LATEX as L_LATEX,
                },
                NarseseFormat,
            },
        },
        lexical::tests::_sample_task_ascii,
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
        let narsese_enum_2 = format.parse(&formatted).expect("枚举Narsese解析失败");
        // 「从枚举Narsese解析的」，应该与「从词法Narsese解析的」一致
        assert_eq!(narsese_enum, narsese_enum_2);
        // 返回折叠后的Narsese
        narsese_enum
    }

    /// 测试/综合
    #[test]
    fn test_fold() {
        let task = _sample_task_ascii();
        let format = &FORMAT_ASCII;
        dbg!(_test_fold_narsese(format, Narsese::Task(task)));
    }

    /// 根据传入的「枚举Narsese格式」「词法Narsese格式」分别生成解析器、格式化器
    fn _generate_from_format<'a>(
        enum_format: &'a EnumNarseseFormat<&'a str>,
        lexical_format: &'a NarseseFormat,
    ) -> (
        impl Fn(String) -> EnumNarsese + 'a,
        impl Fn(String) -> EnumNarsese + 'a,
        impl Fn(EnumNarsese) -> Narsese + 'a,
        impl Fn(Narsese) -> EnumNarsese + 'a,
        impl Fn(EnumNarsese) -> String + 'a,
        impl Fn(Narsese) -> String + 'a,
    ) {
        // 字符串→枚举 枚举解析器：直接进行解析
        let enum_parser = |input: String| {
            enum_format
                .parse::<EnumNarsese>(&input)
                .expect("字符串→枚举：枚举Narsese解析失败")
        };
        // 字符串→枚举 词法解析器：先进行词法解析，然后再词法折叠
        let lexical_parser = |input: String| {
            {
                lexical_format
                    .parse(&input)
                    .expect("字符串→枚举：词法Narsese解析失败")
                    .try_fold_into(enum_format)
            }
            .expect("字符串Narsese：词法Narsese折叠失败")
        };
        // 枚举→词法：先格式化，然后词法解析
        let enum_to_lexical = |input: EnumNarsese| {
            lexical_format
                .parse(&enum_format.format_narsese(&input))
                .expect("枚举→词法：词法Narsese解析失败")
        };
        // 词法→枚举：先格式化，再枚举解析（不同于「词法折叠」）
        let lexical_to_enum = |input: Narsese| {
            enum_format
                .parse::<EnumNarsese>(&lexical_format.format_narsese(&input))
                .expect("词法→枚举：枚举Narsese解析失败")
        };
        // 枚举→字符串 枚举格式化器：直接进行格式化
        let enum_formatter = |input: EnumNarsese| enum_format.format_narsese(&input);
        // 词法→字符串 词法格式化器：先进行词法折叠，然后再格式化（不同于「直接格式化」）
        let lexical_formatter = |input: Narsese| {
            enum_format.format_narsese(
                &input
                    .clone()
                    .try_fold_into(enum_format)
                    .expect("词法→字符串：词法Narsese折叠失败"),
            )
        };
        (
            // 解析器
            enum_parser,
            lexical_parser,
            // 互转器
            enum_to_lexical,
            lexical_to_enum,
            // 格式化器
            enum_formatter,
            lexical_formatter,
        )
    }

    /// 测试/比对性
    fn _test_comparability<'a>(
        enum_format: &'a EnumNarseseFormat<&'a str>,
        lexical_format: &'a NarseseFormat,
        sample_narsese: EnumNarsese,
    ) {
        // 获取解析器、格式化器、互转器
        let (
            // 解析器
            enum_parser,
            lexical_parser,
            // 互转器
            enum_to_lexical,
            lexical_to_enum,
            // 格式化器
            enum_formatter,
            lexical_formatter,
        ) = _generate_from_format(enum_format, lexical_format);
        // 辅助宏：复合函数
        macro_rules! combine {
            ($f1:ident, $($f2:tt)*) => {
                |value| $($f2)*($f1(value))
            };
        }
        // 一些辅助的函数（用于`String`↔`&str`这类细碎转换）
        let lexical_parser_str = |s: String| {
            lexical_format
                .parse(&s)
                .expect("辅助函数：词法Narsese解析失败")
        };
        // 构造「单位函数」 | 可逆性
        let unit_enum_enum = combine!(enum_formatter, enum_parser);
        let unit_enum_lexical = combine!(enum_to_lexical, lexical_to_enum);
        let unit_lexical_enum = combine!(lexical_to_enum, enum_to_lexical);
        let unit_lexical_lexical = combine!(lexical_formatter, lexical_parser_str);
        let unit_string_enum = combine!(enum_parser, enum_formatter);
        let unit_string_lexical_enum = combine!(lexical_parser, enum_formatter);
        // 样例
        let sample_enum = sample_narsese;
        let sample_lexical = enum_to_lexical(sample_enum.clone());
        let sample_string = lexical_format.format_narsese(&sample_lexical);
        dbg!(&sample_lexical, &sample_enum, &sample_string);

        /// 统一判等方法
        trait TestEq {
            /// 统一的判等测试方法
            fn test_eq<'a>(
                self,
                other: Self,
                enum_format: &'a EnumNarseseFormat<&'a str>,
                lexical_format: &'a NarseseFormat,
            ) where
                Self: std::marker::Sized,
            {
                // 转换自身与「其他」
                assert_eq!(
                    self.convert(enum_format, lexical_format),
                    other.convert(enum_format, lexical_format),
                    "转换后仍不相等！"
                );
            }
            fn convert<'a>(
                self,
                enum_format: &'a EnumNarseseFormat<&'a str>,
                lexical_format: &'a NarseseFormat,
            ) -> EnumNarsese;
        }
        /// 枚举Narsese直接判等
        impl TestEq for EnumNarsese {
            fn convert<'a>(
                self,
                enum_format: &'a EnumNarseseFormat<&'a str>,
                lexical_format: &'a NarseseFormat,
            ) -> EnumNarsese {
                self
            }
        }
        /// 词法Narsese：折叠为「枚举Narsese」再判等
        impl TestEq for Narsese {
            fn convert<'a>(
                self,
                enum_format: &'a EnumNarseseFormat<&'a str>,
                lexical_format: &'a NarseseFormat,
            ) -> EnumNarsese {
                self.try_fold_into(enum_format)
                    .expect("词法Narsese：词法Narsese折叠失败")
            }
        }
        /// 字符串：解析成「枚举Narsese」再判等
        impl TestEq for String {
            fn convert<'a>(
                self,
                enum_format: &'a EnumNarseseFormat<&'a str>,
                lexical_format: &'a NarseseFormat,
            ) -> EnumNarsese {
                enum_format
                    .parse(&self)
                    .expect("枚举Narsese：枚举Narsese解析失败")
            }
        }
        /// 辅助宏：执行「单位函数」后是否自相等
        /// * 🚩相等条件：依照[`TestEq`]
        macro_rules! test_unit {
            ($f:ident, $value:expr) => {
                TestEq::test_eq(
                    $f($value.clone()),
                    $value.clone(),
                    enum_format,
                    lexical_format,
                );
            };
        }

        // 💭最终还是要转换到「枚举Narsese」以便判等
        // * 📌字符串/词法Narsese会因为「排序不稳定」而出错
        // 开始「单位函数」测试
        test_unit!(unit_enum_enum, sample_enum);
        test_unit!(unit_enum_lexical, sample_enum);
        test_unit!(unit_lexical_enum, sample_lexical);
        test_unit!(unit_lexical_lexical, sample_lexical);
        test_unit!(unit_string_enum, sample_string);
        test_unit!(unit_string_lexical_enum, sample_string);
        // * 📝【2024-03-21 00:29:14】后记：
        // * 虽然测试期间用了大量`clone`到处复制值，
        // * 混合集成测试也用上了所有case和样例，
        // * 但速度还是快到飞起（跑完就0.23s）
    }

    /// 测试/比对性
    /// * 🎯混合集成测试：当「词法解析」「词法折叠」等基础功能有用之后，才进行
    #[test]
    fn test_comparability() {
        // ! 📌【2024-03-21 00:07:55】由始至终必须使用「枚举Narsese」以保证最大的「跨格式性」与「有序无序性」
        let task = _sample_task_ascii();
        let task = task
            .try_fold_into(&FORMAT_ASCII)
            .expect("词法Narsese：折叠失败");
        // ASCII
        _test_comparability(&FORMAT_ASCII, &L_ASCII, EnumNarsese::Task(task.clone()));
        // LATEX
        _test_comparability(&FORMAT_LATEX, &L_LATEX, EnumNarsese::Task(task.clone()));
        // HAN
        _test_comparability(&FORMAT_HAN, &L_HAN, EnumNarsese::Task(task.clone()));
    }
}
