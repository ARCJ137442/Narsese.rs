//! 实现和「任务」相关的结构
//! * 🎯仅用于表征语法结构
//!   * 后续多半需要再转换
//!
//! 实现内容
//! * 预算值
//! * 任务

use crate::{sentence::Sentence, util::*};

// 预算值 //

mod budget;
pub use budget::*;

// 任务 //

mod task;
pub use task::*;
