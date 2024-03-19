//! 「词法折叠」功能支持
//! * 🎯用于从「词法Narsese」转换到其它形式的Narsese
//! * 📄词法Narsese→枚举Narsese

use util::*;

pub_mod_and_pub_use! {
    // 特征
    traits
    // 格式
    format
}

feature_pub_mod_and_reexport! {
    "enum_narsese" => impl_enum
}
