//! 向「枚举Narsese」的折叠
#![allow(unused, unreachable_code)]

use super::*;
use crate::{
    conversion::string::impl_enum::NarseseFormat as EnumNarseseFormat,
    enum_narsese::{
        Narsese as EnumNarsese, Sentence as EnumSentence, Task as EnumTask, Term as EnumTerm,
    },
    lexical::{Narsese, Sentence, Task, Term},
};

/// 一个简单的「折叠错误」
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FoldError(String);

/// 实现/全体Narsese
/// * 📌一次性实现
/// * 🚩向下分派
impl<'a> TryFoldInto<'a, EnumNarsese, FoldError> for Narsese {
    /// 统一使用「枚举Narsese格式」提供信息
    type Folder = EnumNarseseFormat<&'a str>;

    fn try_fold_into(self, folder: &Self::Folder) -> Result<EnumNarsese, FoldError> {
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

    fn try_fold_into(self, folder: &Self::Folder) -> Result<EnumTerm, FoldError> {
        match self {
            // 原子词项
            Term::Atom { prefix, name } => todo!(),
            // 复合词项
            Term::Compound { connecter, terms } => todo!(),
            // 集合词项
            Term::Set {
                left_bracket,
                terms,
                right_bracket,
            } => todo!(),
            // 陈述
            Term::Statement {
                copula,
                subject,
                predicate,
            } => todo!(),
        }
    }
}

/// 实现/语句
impl<'a> TryFoldInto<'a, EnumSentence, FoldError> for Sentence {
    /// 统一使用「枚举Narsese格式」提供信息
    type Folder = EnumNarseseFormat<&'a str>;

    fn try_fold_into(self, folder: &'a Self::Folder) -> Result<EnumSentence, FoldError> {
        todo!()
    }
}

/// 实现/任务
impl<'a> TryFoldInto<'a, EnumTask, FoldError> for Task {
    /// 统一使用「枚举Narsese格式」提供信息
    type Folder = EnumNarseseFormat<&'a str>;

    fn try_fold_into(self, folder: &'a Self::Folder) -> Result<EnumTask, FoldError> {
        todo!()
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
        folder: &EnumNarseseFormat<&str>,
        lexical_narsese: Narsese,
    ) -> EnumNarsese {
        lexical_narsese
            .try_fold_into(folder)
            .expect("词法折叠失败！")
    }

    /// 测试/词项
    #[test]
    fn test_fold_term() {
        let task = _sample_task_ascii();
        let folder = &FORMAT_ASCII;
        dbg!(_test_fold_narsese(folder, Narsese::Task(task)));
    }
}
