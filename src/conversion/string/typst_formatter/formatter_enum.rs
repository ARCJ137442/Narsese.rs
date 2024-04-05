//! 枚举Narsese的Typst格式化
//! * 🎯首先是「简洁」：追求代码量尽可能少
//! * 🎯其次是「性能」：尽可能少创建值

use super::definition::*;
use crate::{
    api::{
        ExtractTerms, FloatPrecision, FormatTo, GetBudget, GetCategory, GetPunctuation, GetStamp,
        GetTerm, GetTruth, TermCategory,
    },
    conversion::string::{template_atom, template_components, template_statement},
    enum_narsese::{Budget, Punctuation, Sentence, Stamp, Task, Term, Truth},
};
use util::{manipulate, ToDebug};
use Term::*;
use TermCategory::*;

/// 内部格式化方法
impl FormatterTypst {
    /// 【内部】格式化/词项特征字串
    /// * 🎯统一「原子词项前缀」「复合词项连接词」「陈述系词」
    ///   * 用于合并「格式化/词项」中冗余的`match`分支条件
    /// * 🚩部分不用的直接置空
    #[inline]
    fn _feature_string(&self, term: &Term) -> &str {
        match term {
            // * 🚩原子词项 ⇒ 原子词项前缀
            Word(..) => TERM_PREFIX_WORD,
            Placeholder => TERM_PREFIX_PLACEHOLDER,
            VariableIndependent(..) => TERM_PREFIX_I_VAR,
            VariableDependent(..) => TERM_PREFIX_D_VAR,
            VariableQuery(..) => TERM_PREFIX_Q_VAR,
            Interval(..) => TERM_PREFIX_INTERVAL,
            Operator(..) => TERM_PREFIX_OPERATOR,
            // * 🚩复合词项 ⇒ 复合词项连接词
            SetExtension(..) => "", // ! 置空不用（后续有特殊处理逻辑）
            SetIntension(..) => "", // ! 置空不用（后续有特殊处理逻辑）
            IntersectionExtension(..) => CONNECTER_EXT_INTERSECT,
            IntersectionIntension(..) => CONNECTER_INT_INTERSECT,
            DifferenceExtension(..) => CONNECTER_EXT_DIFFERENCE,
            DifferenceIntension(..) => CONNECTER_INT_DIFFERENCE,
            Product(..) => CONNECTER_PRODUCT,
            ImageExtension(..) => CONNECTER_EXT_IMAGE,
            ImageIntension(..) => CONNECTER_INT_IMAGE,
            Conjunction(..) => CONNECTER_CONJUNCTION,
            Disjunction(..) => CONNECTER_DISJUNCTION,
            Negation(..) => CONNECTER_NEGATION,
            ConjunctionSequential(..) => CONNECTER_SEQ_CONJUNCTION,
            ConjunctionParallel(..) => CONNECTER_PAR_CONJUNCTION,
            // * 🚩陈述 ⇒ 陈述系词
            Inheritance(..) => COPULA_INHERITANCE,
            Similarity(..) => COPULA_SIMILARITY,
            Implication(..) => COPULA_IMPLICATION,
            Equivalence(..) => COPULA_EQUIVALENCE,
            ImplicationPredictive(..) => COPULA_IMPLICATION_PREDICTIVE,
            ImplicationConcurrent(..) => COPULA_IMPLICATION_CONCURRENT,
            ImplicationRetrospective(..) => COPULA_IMPLICATION_RETROSPECTIVE,
            EquivalencePredictive(..) => COPULA_EQUIVALENCE_PREDICTIVE,
            EquivalenceConcurrent(..) => COPULA_EQUIVALENCE_CONCURRENT,
        }
    }

    /// 【内部】格式化/括弧字串
    /// * 🎯统一「一般复合词项」与「外延集/内涵集」的「左右括弧」
    ///   * 用于合并「格式化/词项」中冗余的`match`分支条件
    /// * 🚩不用的直接置空
    #[inline]
    fn _brackets_str(&self, term: &Term) -> (&str, &str) {
        match term {
            // * 🚩外延集
            SetExtension(..) => BRACKETS_EXT_SET,
            // * 🚩内涵集
            SetIntension(..) => BRACKETS_INT_SET,
            // * 🚩剩下的⇒匹配「词项类别」
            _ => match term.get_category() {
                // * 🚩一般复合词项⇒复合词项括弧
                Compound => BRACKETS_COMPOUND,
                // * 🚩陈述⇒陈述括弧
                Statement => BRACKETS_STATEMENT,
                // * 🚩其它⇒置空
                _ => ("", ""),
            },
        }
    }

    /// 模板/一般复合词项
    /// * 🎯使用「连接符」区分「复合类型」的词项
    /// * 📝对于「字符串字面量数组」，`Vec<&str>`的引用类型对应`&[&str]`而非`&[str]`
    ///   * ⚠️后者的`str`是大小不定的：the size for values of type `str` cannot be known at compilation time
    fn template_compound(
        out: &mut String,
        brackets: (&str, &str),
        connecter: &str,
        components: impl Iterator<Item = String>,
        separator: &str,
    ) {
        // 先收集迭代器
        let strings = components.collect::<Vec<_>>();
        // 左括号
        out.push_str(brackets.0);
        // 分派方法：针对内容数目、连接符是否为「集合词项」（是否为空）
        match (strings.len(), connecter) {
            // 集合⇒直接上内容
            (_, "") => template_components(out, strings.into_iter(), separator, ""),
            // 二元非集合⇒中缀形式
            // * 🚩组分 & 连接符 as 分隔符 | `A * B`
            (2, _) => template_components(out, strings.into_iter(), connecter, ""),
            // 一元/多元 非集合⇒前缀形式
            // * 🚩组分 | `A, B, C`
            _ => {
                //连接符与分隔符
                out.push_str(connecter);
                // 分隔符
                out.push_str(separator);
                // 组分
                template_components(out, strings.into_iter(), separator, "")
            }
        }
        // 右括号 | `)`
        out.push_str(brackets.1);
    }

    /// 【内部】格式化/词项
    fn format_term(&self, out: &mut String, term: &Term) {
        // 特征字串/括弧字串
        let feature_str = self._feature_string(term);
        let brackets_str = self._brackets_str(term);

        // 直接按「词项类别」格式化
        match term.get_category() {
            // 原子词项 | 特征字串 as 前缀 + 词项名
            Atom => template_atom(
                out,
                feature_str,
                // 使用`to_debug`转义其中的字符
                &term.get_atom_name_unchecked().to_debug(),
            ),
            // 复合词项
            Compound => Self::template_compound(
                out,
                brackets_str,
                feature_str,
                term.clone().extract_terms().map(|t| self.format(&t)),
                SEPARATOR_COMPOUND,
            ),
            // 陈述
            Statement => template_statement(
                out,
                brackets_str.0,
                &self.format(term.get_components()[0]),
                feature_str,
                &self.format(term.get_components()[1]),
                SEPARATOR_STATEMENT,
                brackets_str.1,
            ),
        }
    }

    /// 【内部】格式化/标点
    fn format_punctuation(&self, out: &mut String, punctuation: &Punctuation) {
        use Punctuation::*;
        out.push_str(match punctuation {
            Judgement => PUNCTUATION_JUDGEMENT,
            Goal => PUNCTUATION_GOAL,
            Question => PUNCTUATION_QUESTION,
            Quest => PUNCTUATION_QUEST,
        })
    }

    /// 【内部】格式化/时间戳
    fn format_stamp(&self, out: &mut String, stamp: &Stamp) {
        use Stamp::*;
        // 前缀
        let prefix = match stamp {
            Eternal => STAMP_ETERNAL,
            Past => STAMP_PAST,
            Present => STAMP_PRESENT,
            Future => STAMP_FUTURE,
            Fixed(_) => STAMP_FIXED,
        };
        // 内容
        let content = match stamp {
            // * 仅「固定」需要把内容转换为字符串
            Fixed(t) => t.to_string(),
            _ => String::new(),
        };
        // 拼接
        manipulate!(
            out
            => .push_str(prefix)
            => .push_str(&content)
        );
    }

    /// 【内部】格式化浮点序列
    fn _format_floats(
        &self,
        out: &mut String,
        brackets: (&str, &str),
        separator: &str,
        floats: &[FloatPrecision],
    ) {
        out.push_str(brackets.0);
        for (i, f) in floats.iter().enumerate() {
            // 分隔符
            if i != 0 {
                out.push_str(separator);
            }
            out.push_str(&f.to_string());
        }
        out.push_str(brackets.1);
    }

    /// 【内部】格式化/真值
    fn format_truth(&self, out: &mut String, truth: &Truth) {
        use Truth::*;
        // * ❌【2024-04-05 17:36:48】无法统一成「先获取『浮点数列表』再『统一格式化列表』」的形式
        //   * 📝`match`分支不能直接返回引用，即便从绑定变量中解引用，也会导致「返回临时变量引用」的问题
        match truth {
            // 空真值⇒直接为空
            Empty => {}
            // 单真值⇒单元素数组
            Single(f) => self._format_floats(out, BRACKETS_TRUTH, SEPARATOR_TRUTH, &[*f]),
            // 双真值⇒二元数组
            Double(f, c) => self._format_floats(out, BRACKETS_TRUTH, SEPARATOR_TRUTH, &[*f, *c]),
        }
    }

    /// 【内部】格式化/预算值
    fn format_budget(&self, out: &mut String, budget: &Budget) {
        use Budget::*;
        match budget {
            // 空预算⇒空数组，仅含括弧 // ! 若无括弧，解析器将识别成语句
            Empty => self._format_floats(out, BRACKETS_BUDGET, SEPARATOR_BUDGET, &[]),
            // 单预算⇒单元素数组
            Single(p) => self._format_floats(out, BRACKETS_BUDGET, SEPARATOR_BUDGET, &[*p]),
            // 双预算⇒二元数组
            Double(p, d) => self._format_floats(out, BRACKETS_BUDGET, SEPARATOR_BUDGET, &[*p, *d]),
            // 三预算⇒三元数组
            Triple(p, d, q) => {
                self._format_floats(out, BRACKETS_BUDGET, SEPARATOR_BUDGET, &[*p, *d, *q])
            }
        }
    }
}

/// 格式化/词项
impl FormatTo<&FormatterTypst, String> for Term {
    fn format_to(&self, formatter: &FormatterTypst) -> String {
        manipulate!(
            String::new()
            // 格式化
            => [formatter.format_term](_, self)
            // 后处理
            => post_process_whitespace
        )
    }
}

/// 格式化/标点
impl FormatTo<&FormatterTypst, String> for Punctuation {
    fn format_to(&self, formatter: &FormatterTypst) -> String {
        manipulate!(
            String::new()
            // 格式化
            => [formatter.format_punctuation](_, self)
            // 后处理
            => post_process_whitespace
        )
    }
}

/// 格式化/时间戳
impl FormatTo<&FormatterTypst, String> for Stamp {
    fn format_to(&self, formatter: &FormatterTypst) -> String {
        manipulate!(
            String::new()
            // 格式化
            => [formatter.format_stamp](_, self)
            // 后处理
            => post_process_whitespace
        )
    }
}

/// 格式化/真值
impl FormatTo<&FormatterTypst, String> for Truth {
    fn format_to(&self, formatter: &FormatterTypst) -> String {
        manipulate!(
            String::new()
            // 格式化
            => [formatter.format_truth](_, self)
            // 后处理
            => post_process_whitespace
        )
    }
}

/// 格式化/预算值
impl FormatTo<&FormatterTypst, String> for Budget {
    fn format_to(&self, formatter: &FormatterTypst) -> String {
        manipulate!(
            String::new()
            // 格式化
            => [formatter.format_budget](_, self)
            // 后处理
            => post_process_whitespace
        )
    }
}

/// 格式化/语句
/// * 🚩「词项」与「标点」间无间隔
/// * 🚩时间戳、真值可能缺省
impl FormatTo<&FormatterTypst, String> for Sentence {
    fn format_to(&self, formatter: &FormatterTypst) -> String {
        manipulate!(
            String::new()
            // 词项 & 标点
            => [formatter.format_term](_, self.get_term())
            => [formatter.format_punctuation](_, self.get_punctuation())
            // 时间戳
            => [formatter.format_stamp](_, self.get_stamp())
            => .push_str(SEPARATOR_ITEM)
            // 真值 | 默认为空
            => [formatter.format_truth](_, self.get_truth().unwrap_or(&Truth::Empty))
            // 后处理
            => post_process_whitespace
        )
    }
}

/// 格式化/任务
/// * 🚩【2024-04-05 19:00:13】无需再担心「缺省问题」与「空格问题」：统一交给后处理
impl FormatTo<&FormatterTypst, String> for Task {
    fn format_to(&self, formatter: &FormatterTypst) -> String {
        manipulate!(
            String::new()
            // 预算值
            => [formatter.format_budget](_, self.get_budget())
            => .push_str(SEPARATOR_ITEM)
            // 词项 & 标点
            => [formatter.format_term](_, self.get_term())
            => [formatter.format_punctuation](_, self.get_punctuation())
            => .push_str(SEPARATOR_ITEM)
            // 时间戳
            => [formatter.format_stamp](_, self.get_stamp())
            => .push_str(SEPARATOR_ITEM)
            // 真值 | 默认为空
            => [formatter.format_truth](_, self.get_truth().unwrap_or(&Truth::Empty))
            // 后处理
            => post_process_whitespace
        )
    }
}

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        conversion::string::impl_enum::tests::_sample_task,
        enum_narsese::tests::generate_term_testset, enum_nse as nse,
    };
    use util::{asserts, f_parallel};

    /// 测试一个Narsese值
    /// * 🎯成功格式化
    /// * 🎯不包含连续空格
    fn _test<'a>(value: &impl FormatTo<&'a FormatterTypst, String>) {
        // 格式化
        let formatted = FormatterTypst.format(value);
        // 打印
        println!("{formatted}");
        // 检查空格
        asserts! {
            // 左右不包含多余空格
            formatted == formatted.trim()
            // 不包含连续空格
            !formatted.contains("  ")
        }
    }

    /// 测试一个Narsese值
    /// * 基于枚举Narsese文本，测试语义稳定性
    fn _test_example<'a>(value: &impl FormatTo<&'a FormatterTypst, String>, expected_str: &str) {
        // 格式化
        let formatted = FormatterTypst.format(value);
        // 打印
        println!("{formatted}");
        // 是否与预期相等
        assert_eq!(formatted, expected_str);
    }

    /// 测试
    #[test]
    fn test() {
        // 测试词项
        let terms = generate_term_testset();
        for term in terms {
            _test(&term);
        }
        // 测试任务
        let sample_task = _sample_task();
        _test(&sample_task);
    }

    /// 测试/样例
    #[test]
    fn test_examples() {
        // 测试预期相等
        f_parallel![
            _test_example;
            &nse!(<A --> B>), r#"lr(angle.l "A" arrow.r "B" angle.r)"#;
            &nse!(<A ==> B>.), r#"lr(angle.l "A" arrow.r.double "B" angle.r) . space"#;
            &nse!($0.4; 0.4; 0.4$ <{SELF} --> [good]>! :|: %1.0;0.9%), r#"lr(\$ 0.4";"0.4";"0.4 \$) space lr(angle.l lr({ "SELF" }) arrow.r lr([ "good" ]) angle.r) ! space \|#h(-0.6em)arrow.r.double space lr(angle.l 1,0.9 angle.r)"#;
        ];
    }
}
