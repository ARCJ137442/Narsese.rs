//! 库的主模块

// 实用
pub mod macros;
pub mod util;

// 共用GPI
pub mod common_api;

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
