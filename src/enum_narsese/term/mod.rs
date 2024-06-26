//! 直接用枚举实现简单的Narsese词项
//!
//! 📝各层Narsese词项类型及其特点（from旧稿）
//! * 1 词语 - 原子词项
//!   * 继承 - 二元有序词项容器
//! * 2 相似 - 词项容器
//! * 3 词项集 - 词项容器
//!   * 词项交集 - 词项容器
//!   * 词项差集 - 二元有序词项容器
//! * 4 词项乘积 - 有序词项容器
//!   * 像 - 有序词项容器 | // ! 不含「占位符」
//!   * 占位符 - 原子词项 | 传承自JuNarsese；🎯可作为后续「通用组分构造像」的组分
//! * 5 蕴含 - 二元有序词项容器
//!   * 等价 - 词项容器
//!   * 合取 - 词项容器
//!   * 析取 - 词项容器
//!   * 否定 - 一元词项容器
//! * 6 独立变量 - 原子词项
//!   * 非独变量 - 原子词项
//!   * 查询变量 - 原子词项
//! * 7 间隔 - 原子词项
//!   * 顺序合取 - 有序词项容器
//!   * 平行合取 - 词项容器
//! * 8 操作符 - 原子词项
//!
//! 📝上述词项类型的分类树
//! * 原子词项
//!   * 1 词语
//!   * 6 独立变量
//!   * 6 非独变量
//!   * 6 查询变量
//!   * 7 间隔
//! * 复合词项
//!   * 3 外延集
//!   * 3 内涵集
//!   * 3 外延交
//!   * 3 内涵交
//!   * 3 外延差
//!   * 3 内涵差
//!   * 4 乘积
//!   * 4 外延像
//!   * 4 内涵像
//!   * 5 合取
//!   * 5 析取
//!   * 5 否定
//!   * 7 顺序合取
//!   * 7 平行合取
//! * 陈述
//!   * 1 继承
//!   * 2 相似
//!   * 5 蕴含
//!   * 5 等价
//!
//! 📌简化方法：
//! * 扁平化：不使用任何「枚举套枚举」的举措
//!   * 避免形如`Term::Atom(Atom::Word(/* ... */))`的繁杂

nar_dev_utils::mod_and_pub_use! {
    // 结构
    structs
    // 实现
    impls
}
