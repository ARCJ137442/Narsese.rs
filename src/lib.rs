//! 库的主模块

// 实用 | 包括工具宏
pub mod util;

// 共用API
pub mod api;

// 词法
#[cfg(feature = "lexical_narsese")]
pub mod lexical;

#[cfg(feature = "enum_narsese")]
pub mod enum_narsese;

// 转换
pub mod conversion;

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;

    #[test]
    fn main() {}
}
