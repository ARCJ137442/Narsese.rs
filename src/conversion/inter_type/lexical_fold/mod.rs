//! 「词法折叠」功能支持
//! * 🎯用于从「词法Narsese」转换到其它形式的Narsese
//! * 📄词法Narsese→枚举Narsese

use nar_dev_utils::*;

pub_mod_and_pub_use! {
    // 特征
    traits
}

feature_pub_mod_and_reexport! {
    // 枚举Narsese
    "enum_narsese" => impl_enum
}
