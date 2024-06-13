//! 定义集成「词项/标点/时间戳/真值/预算值」的通用「部分Narsese」
//! * 🎯提供「与具体实现无关」的Narsese数据结构表征
//! * 🎯最初用于统一定义

use nar_dev_utils::matches_or;

/// 集成「词项/标点/时间戳/真值/预算值」的通用「可选Narsese」
/// * 📌泛型顺序遵循ASCII Narsese格式
///   * 📄`$0.9;0.9;0.8$ <A --> B>. :|: %1.0;0.9%`
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct NarseseOptions<Budget, Term, Punctuation, Stamp, Truth> {
    /// 预算值 @ 任务
    pub budget: Option<Budget>,
    /// 词项
    pub term: Option<Term>,
    /// 标点 @ 语句
    pub punctuation: Option<Punctuation>,
    /// 时间戳 @ 语句
    pub stamp: Option<Stamp>,
    /// 真值 @ 语句
    pub truth: Option<Truth>,
}

// 基础功能实现
impl<Budget, Term, Punctuation, Stamp, Truth>
    NarseseOptions<Budget, Term, Punctuation, Stamp, Truth>
{
    /// 构造一个全空的结果
    pub fn new() -> Self {
        Self {
            term: None,
            truth: None,
            budget: None,
            stamp: None,
            punctuation: None,
        }
    }

    /// 拿走其中所有结果，将自身变回空值
    /// * 🚩一个个字段[`Option::take`]
    pub fn take(&mut self) -> Self {
        Self {
            budget: self.budget.take(),
            term: self.term.take(),
            punctuation: self.punctuation.take(),
            stamp: self.stamp.take(),
            truth: self.truth.take(),
        }
    }

    /// 拿出其中的预算值
    #[inline]
    pub fn take_budget(&mut self) -> Option<Budget> {
        self.budget.take()
    }
    /// 拿出其中的词项
    #[inline]
    pub fn take_term(&mut self) -> Option<Term> {
        self.term.take()
    }
    /// 拿出其中的标点
    #[inline]
    pub fn take_punctuation(&mut self) -> Option<Punctuation> {
        self.punctuation.take()
    }
    /// 拿出其中的时间戳
    #[inline]
    pub fn take_stamp(&mut self) -> Option<Stamp> {
        self.stamp.take()
    }
    /// 拿出其中的真值
    #[inline]
    pub fn take_truth(&mut self) -> Option<Truth> {
        self.truth.take()
    }

    /// 判断其中是否具有「语句」
    /// * 🚩条件：同时具有「词项」「标点」
    /// * 💭【2024-06-13 20:33:03】可能「时间戳」「真值」不一定有
    pub fn has_sentence(&self) -> bool {
        matches!(
            self,
            Self {
                term: Some(..),
                punctuation: Some(..),
                ..
            }
        )
    }

    /// 判断其中是否具有「任务」
    /// * 🚩条件：同时具有「词项」「标点」「预算值」
    /// * 💭虽然某些情况下没有「预算值」也能成为「任务」（使用默认预算），
    ///   * 但此处需要和[`Self::has_sentence`]区分
    pub fn has_task(&self) -> bool {
        matches!(
            self,
            Self {
                budget: Some(..),
                term: Some(..),
                punctuation: Some(..),
                ..
            }
        )
    }

    /// 「拿出」其中的「语句」
    /// * 🚩当同时具有「词项」「标点」时拿出「词项」「标点」「时间戳」「真值」
    pub fn take_sentence(&mut self) -> Option<(Term, Punctuation, Option<Stamp>, Option<Truth>)> {
        matches_or!(
            ?self,
            Self {
                term: term @ Some(..),
                punctuation: punctuation @ Some(..),
                stamp,
                truth,
                ..
            } => (
                term.take().unwrap(),
                punctuation.take().unwrap(),
                stamp.take(),
                truth.take(),
            )
        )
    }

    /// 「拿出」其中的「任务」
    /// * 🚩当同时具有「预算值」「词项」「标点」时，拿出所有字段
    /// * 💭其返回值结构稍有复杂
    #[allow(clippy::type_complexity)]
    pub fn take_task(
        &mut self,
    ) -> Option<(Budget, Term, Punctuation, Option<Stamp>, Option<Truth>)> {
        matches_or!(
            ?self,
            Self {
                budget:budget @ Some(..),
                term: term @ Some(..),
                punctuation: punctuation @ Some(..),
                stamp,
                truth,
                ..
            } => (
                budget.take().unwrap(),
                term.take().unwrap(),
                punctuation.take().unwrap(),
                stamp.take(),
                truth.take(),
            )
        )
    }
}
