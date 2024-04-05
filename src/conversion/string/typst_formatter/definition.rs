//! Typst格式化器
//! * 🎯定义数据结构
//! * 🎯提供（数据结构无关的）通用格式化常量/方法
//! * 🚩【2024-04-05 18:47:24】现在所有「非空内容」均自带环绕空格
//!   * ✅其后无需担心「空格问题」
//!   * 🚩多余的空格将被「后处理函数」合并
//!   * ✅即便是「标点」也会在最后Typst呈现时被忽略
//!   * ⚡平衡：牺牲部分性能，成就代码简洁度
//! * 🚩【2024-04-05 20:12:09】目前选择将「语法常量」保存于此
//!   * 🎯除用于格式化「枚举Narsese」外，还可被其它 解析器/格式化器 用于更多目的

use crate::api::FormatTo;
use util::if_return;

/// Typst格式化器
/// * 仅作为一个「格式化对者」使用
pub struct FormatterTypst;

impl FormatterTypst {
    /// 格式化任何能格式化的类型
    /// * ✨枚举Narsese
    /// * ❌词法Narsese
    ///   * 🚩【2024-04-05 20:13:46】缺乏语义信息
    pub fn format<'s, T>(&'s self, target: &impl FormatTo<&'s Self, T>) -> T {
        target.format_to(self)
    }
}

// * 原子词项前缀 * //

/// 原子词项前缀/词语
pub const TERM_PREFIX_WORD: &str = "";
/// 原子词项前缀/占位符
pub const TERM_PREFIX_PLACEHOLDER: &str = " diamond.small ";
/// 原子词项前缀/独立变量
pub const TERM_PREFIX_I_VAR: &str = r" \$ #h(-0.05em) ";
/// 原子词项前缀/非独变量
pub const TERM_PREFIX_D_VAR: &str = r" \# #h(-0.05em) ";
/// 原子词项前缀/查询变量
pub const TERM_PREFIX_Q_VAR: &str = " ? #h(-0.05em) ";
/// 原子词项前缀/间隔
pub const TERM_PREFIX_INTERVAL: &str = " + #h(-0.05em) ";
/// 原子词项前缀/操作符
pub const TERM_PREFIX_OPERATOR: &str = " arrow.t.double #h(-0.05em) ";

// * 括弧 * //
/// * 🚩在各自代码中区分「是否内包空格」

/// 复合词项括弧
pub const BRACKETS_COMPOUND: (&str, &str) = (" lr(( ", " )) ");
/// 外延集括弧
pub const BRACKETS_EXT_SET: (&str, &str) = (" lr({ ", " }) ");
/// 内涵集括弧
pub const BRACKETS_INT_SET: (&str, &str) = (" lr([ ", " ]) ");
/// 陈述括弧
pub const BRACKETS_STATEMENT: (&str, &str) = (" lr(angle.l ", " angle.r) ");
/// 真值括弧
pub const BRACKETS_TRUTH: (&str, &str) = (" lr(angle.l ", " angle.r) ");
/// 预算值括弧
pub const BRACKETS_BUDGET: (&str, &str) = (r" lr(\$ ", r" \$) ");

// * 分隔符 * //

/// 复合词项分隔符 | 🎯复合词项
pub const SEPARATOR_COMPOUND: &str = " space ";
/// 陈述分隔符 | 🚩陈述不带空格
pub const SEPARATOR_STATEMENT: &str = "";
/// 条目 | 🎯词项 标点 + 时间戳 + 真值
pub const SEPARATOR_ITEM: &str = " space ";
/// 真值
pub const SEPARATOR_TRUTH: &str = ",";
/// 预算值 | ⚠️在实际情况中，`lr(\$ 1; 0 \$)`会导致语法错误
pub const SEPARATOR_BUDGET: &str = "\";\"";

// * 复合词项连接词 * //

// 外延交
pub const CONNECTER_EXT_INTERSECT: &str = " sect ";
// 内涵交
pub const CONNECTER_INT_INTERSECT: &str = " union ";
// 外延差
pub const CONNECTER_EXT_DIFFERENCE: &str = " minus ";
// 内涵差
pub const CONNECTER_INT_DIFFERENCE: &str = " minus.circle ";
// 乘积
pub const CONNECTER_PRODUCT: &str = " times ";
// 外延像
pub const CONNECTER_EXT_IMAGE: &str = r" \/ ";
// 内涵像
pub const CONNECTER_INT_IMAGE: &str = r" \\ ";
// 合取
pub const CONNECTER_CONJUNCTION: &str = " and ";
// 析取
pub const CONNECTER_DISJUNCTION: &str = " or ";
// 否定
pub const CONNECTER_NEGATION: &str = " not ";
// 顺序合取
pub const CONNECTER_SEQ_CONJUNCTION: &str = " , ";
// 平行合取
pub const CONNECTER_PAR_CONJUNCTION: &str = " ; ";

// * 陈述系词 * //

/// 继承
pub const COPULA_INHERITANCE: &str = " arrow.r ";
/// 相似
pub const COPULA_SIMILARITY: &str = " arrow.l.r ";
/// 蕴含
pub const COPULA_IMPLICATION: &str = " arrow.r.double ";
/// 等价
pub const COPULA_EQUIVALENCE: &str = " arrow.l.r.double ";
/// 实例
pub const COPULA_INSTANCE: &str = " compose#h(-0.05em)arrow.r ";
/// 属性
pub const COPULA_PROPERTY: &str = " arrow.r#h(-0.05em)compose ";
/// 实例属性
pub const COPULA_INSTANCE_PROPERTY: &str = " compose#h(-0.05em)arrow.r#h(-0.05em)compose ";
/// 预测性蕴含
pub const COPULA_IMPLICATION_PREDICTIVE: &str = r" space\/#h(-0.6em)arrow.r.double ";
/// 并发性蕴含
pub const COPULA_IMPLICATION_CONCURRENT: &str = r" space\|#h(-0.6em)arrow.r.double ";
/// 回顾性蕴含
pub const COPULA_IMPLICATION_RETROSPECTIVE: &str = r" space\\#h(-0.6em)arrow.r.double ";
/// 预测性等价
pub const COPULA_EQUIVALENCE_PREDICTIVE: &str = r" space\/#h(-0.6em)arrow.l.r.double ";
/// 并发性等价
pub const COPULA_EQUIVALENCE_CONCURRENT: &str = r" space\|#h(-0.6em)arrow.l.r.double ";
/// 回顾性等价
pub const COPULA_EQUIVALENCE_RETROSPECTIVE: &str = r" space\\#h(-0.6em)arrow.l.r.double ";

// * 时间戳 * //
// * 🚩以「前缀+内容」的形式进行格式化
//   * 🎯统一「枚举Narsese」和「词法Narsese」
//   * 🚩对「过去/现在/未来」采取「内容空置」的措施

/// 永恒
pub const STAMP_ETERNAL: &str = r"";
/// 过去
pub const STAMP_PAST: &str = r" \/#h(-0.6em)arrow.r.double ";
/// 现在
pub const STAMP_PRESENT: &str = r" \|#h(-0.6em)arrow.r.double ";
/// 未来
pub const STAMP_FUTURE: &str = r" \\#h(-0.6em)arrow.r.double ";
/// 固定
pub const STAMP_FIXED: &str = r" t= ";

// * 标点 * //

/// 判断
pub const PUNCTUATION_JUDGEMENT: &str = " . ";
/// 目标
pub const PUNCTUATION_GOAL: &str = " ! ";
/// 问题
pub const PUNCTUATION_QUESTION: &str = " ? ";
/// 请求
pub const PUNCTUATION_QUEST: &str = " quest.inv ";

// * 通用格式化函数 * //

/// 后处理：多个空白符⇒一个空白符
pub fn post_process_whitespace(s: &mut String) {
    // 预先剪去左右空白符
    let trimmed_s = s.trim();
    // 剪去后空⇒直接清空
    if_return! { trimmed_s.is_empty() => s.clear() }
    // 其它情况⇒追加第一个字符，其后遍历剩余字符串
    let mut result = String::new();
    let chars = trimmed_s.chars().collect::<Vec<_>>();
    result.push(chars[0]);
    for i in 1..chars.len() {
        match (chars[i - 1].is_whitespace(), chars[i].is_whitespace()) {
            (true, true) => {}
            _ => result.push(chars[i]),
        }
    }
    // 最后直接赋值替换
    *s = result
}

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use util::{for_in_ifs, manipulate};

    /// 单个字串的测试
    fn _test(s: &str) {
        let processed = manipulate!(
            s.to_string()
            => post_process_whitespace
        );
        assert!(!processed.contains("  "));
    }

    /// 总测试
    #[test]
    fn test() {
        for_in_ifs![
            {_test(i)}
            for i in ([
                "",
                " ",
                "  ",
                "a",
                "a ",
                " a",
                " a ",
                " a ",
                "a b",
                " a b",
                "a b ",
                " a b ",
                " a  b ",
                " lr(\\$ 0.5\";\"0.75\";\"0.4 \\$)  lr(angle.l  lr((  ,  space  lr(angle.l  lr({  space \"ball\" })  arrow.r  lr([  space \"left\" ])  angle.r)  space  lr(angle.l  lr((  times  space  lr({  space \"SELF\" })  space  \\$ #h(-0.05em) \"any\" space  \\# #h(-0.05em) \"some\" ))  arrow.r  arrow.t.double #h(-0.05em) \"do\" angle.r)  ))  arrow.l.r  lr(angle.l  lr({  space \"SELF\" })  arrow.r  lr([  space \"good\" ])  angle.r)  angle.r)  .  t= -1 lr(angle.l 1,0.9 angle.r) ",
            ])
        ];
    }
}
