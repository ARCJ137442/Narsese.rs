//! 使用枚举简单实现一个Narsese数据结构库
//! 三种数据结构
//! * 词项（首要）
//! * 语句（封装）
//! * 任务（封装）

use crate::api::NarseseValue;

// 词项/语句/任务
nar_dev_utils::pub_mod_and_pub_use! {
    // 词项
    term
    // 语句
    sentence
    // 任务
    task
}

// 统合结构体

/// 集「词项/语句/任务」于一身的「枚举Narsese」（Narsese值）
pub type Narsese = NarseseValue<Term, Sentence, Task>;
