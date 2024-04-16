//! 提取词项内的元素
//! * ✨允许通用地从原子词项、复合词项、陈述等词项中提取「词项」作为元素
//!   * 📌原子词项⇒【只迭代出自身】的迭代器
//!   * 📌复合词项⇒迭代其中组分（**包括像占位符**）的迭代器
//!   * 📌陈述⇒迭代其主词、系词的迭代器
//! * 🎯BabelNAR中用于从`<(*,{SELF},x)-->^op>`中提取「操作参数」

pub trait ExtractTerms {
    type Term;

    /// 提取词项内的元素
    /// * ⚠️消耗自身
    /// * 📌原子词项⇒【只迭代出自身】
    /// * 📌复合词项⇒迭代其中组分（**包括像占位符**）
    /// * 📌陈述⇒迭代其主词、系词的迭代器
    fn extract_terms(self) -> impl Iterator<Item = Self::Term>;

    /// 提取词项内元素，并收集到[`Vec`]中
    /// * 📄提取过程参见[`extract_terms`]
    fn extract_terms_to_vec(self) -> Vec<Self::Term>
    where
        Self: Sized,
    {
        self.extract_terms().collect()
    }
}
