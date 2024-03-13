//! Narsese通用API
//! * 🎯提供【与具体实现无关】的各类Narsese接口
//!   * 💭这些接口的抽象层次各有千秋
//!   * 目前主要是「枚举Narsese」与「词法Narsese」
//! * 🚩目前主要提供一些可用的特征，以供各「Narsese实现」实现
//!   * 💫目前这些概念仍然比较凌乱
//! * 🚩目前除非遇到「重名问题」，否则「导出模块并同时使用其内符号」

// API「获取内部元素」
pub mod get_inner;
pub use get_inner::*;
