# Narsese.rs

|**简体中文** | [English](README.en.md)|
|:-:|:-:|

🕒最后更新时间：【2024-09-14 10:19:55】

<!-- 徽章安排参考：https://daily.dev/blog/readme-badges-github-best-practices#organizing-badges-in-your-readme -->

![License](https://img.shields.io/crates/l/narsese?style=for-the-badge&color=ff7043)
![Code Size](https://img.shields.io/github/languages/code-size/ARCJ137442/Narsese.rs?style=for-the-badge&color=ff7043)
![Lines of Code](https://www.aschey.tech/tokei/github.com/ARCJ137442/Narsese.rs?style=for-the-badge&color=ff7043)
[![Language](https://img.shields.io/badge/language-Rust-orange?style=for-the-badge&color=ff7043)](https://www.rust-lang.org)

<!-- 面向用户 -->

Cargo状态：

[![crates.io](https://img.shields.io/crates/v/narsese?style=for-the-badge)](https://crates.io/crates/narsese)
[![docs.rs](https://img.shields.io/docsrs/narust-158?style=for-the-badge)](https://docs.rs/narsese)
![Recent Downloads](https://img.shields.io/crates/dr/narsese?style=for-the-badge)
![Downloads](https://img.shields.io/crates/d/narsese?style=for-the-badge)
![Crate Size](https://img.shields.io/crates/size/narsese?style=for-the-badge)

<!-- 面向开发者 -->

开发状态：

[![CI status](https://img.shields.io/github/actions/workflow/status/ARCJ137442/Narsese.rs/ci.yml?style=for-the-badge)](https://github.com/ARCJ137442/Narsese.rs/actions/workflows/ci.yml)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-2.0.0-%23FE5196?style=for-the-badge)](https://conventionalcommits.org)
![GitHub commits since latest release](https://img.shields.io/github/commits-since/ARCJ137442/Narsese.rs/latest?style=for-the-badge)

![Created At](https://img.shields.io/github/created-at/ARCJ137442/Narsese.rs?style=for-the-badge)
![Last Commit](https://img.shields.io/github/last-commit/ARCJ137442/Narsese.rs?style=for-the-badge)

## 简介

该项目使用[语义化版本 2.0.0](https://semver.org/lang/zh-CN/)进行版本号管理。

**Narsese**的[**Rust**](https://www.rust-lang.org)实现

- ✨基于枚举`enum`类型实现的[**枚举Narsese**](#枚举narsese)
- ✨基于「嵌套字串词法树」实现的[**词法Narsese**](#词法narsese)
- 🏗️一个可用于在Rust中开发NARS的**Narsese API**（开发中）

## 安装

（最后更新：2024-04-10）

作为已发布于[crates.io](https://crates.io/)的Rust库，可直接在`Cargo.toml`中添加依赖：

```toml
[dependencies.narsese]
version = "0" # 请按需选择合适的最新版本，必要时参考crates.io的对应侧边栏
features = ["bundled"] # 启用所有特性，包括「枚举Narsese」和「词法Narsese」
```

库特性列表：

- `bundled`：启用所有特性
- `enum_narsese`：启用「枚举Narsese」
- `lexical_narsese`：启用「词法Narsese」

## 使用

（最后更新：2024-03-29）

### 使用/枚举Narsese

```rust
use narsese::enum_nse;

fn main() {
    // 使用快捷宏解析创建Narsese（需要保证语法正确，否则panic） //
    let term = enum_term!(<A --> B>);
    let sentence = enum_sentence!("<SELF {-] good>!");
    let task = enum_task!($0.8;0.8;0.8$ <robin --> bird>?);

    // 输出 / 检验 //
    println!("{term:?}");
    assert_eq!(term, enum_nse!("<A --> B>").try_into_term().unwrap()); // 字符串形式的解析结果与之相等，并使用`try_into_term`进行向下转换
    println!("{sentence:?}");
    println!("{task:?}");
}
```

### 使用/词法Narsese

```rust
use narsese::lexical_nse;

fn main() {
    // 使用快捷宏解析创建Narsese（需要保证语法正确，否则panic） //
    let term = lexical_term!(<A --> B>);
    let sentence = lexical_sentence!("<SELF {-] good>!");
    let task = lexical_task!($0.8;0.8;0.8$ <robin --> bird>?);

    // 输出 / 检验 //
    println!("{term:?}");
    println!("{sentence:?}");
    println!("{task:?}");
}
```

（更多用例可参考项目的单元测试）

🚧【2024-03-29 00:43:13】WIP：更具体的用例

## 使用/Narsese API

🚧【2024-04-05 00:09:03】WIP：基础功能

## 概念

### 枚举Narsese

✨基于Rust原生枚举`enum`特性实现

- ✅语义精确性
  - 集合语义：元素无序的复合词项中，语义顺序无关
  - 真值/预算值 语义：存储具体的、范围固定的浮点数值
- ✅结构易用性
  - Narsese完备：实现了CommonNarsese的所有数据结构

### 词法Narsese

✨基于「嵌套字串词法树」实现

- ✅结构完备性
  - 词法完备：实现了CommonNarsese的所有词法结构，如「复合词项」「陈述」「语句」「任务」
- ✅数据灵活性
  - 非类别特定：不特别限定「原子词项前缀」「复合词项连接词」「陈述系词」的范围
  - 可扩展：通过「词法折叠」机制，允许向特定、专用数据结构的进一步转换
    - ✅已对[枚举Narsese](#枚举narsese)提供了「词法折叠」机制

### NarseseAPI

（最后更新：2024-04-05）

- 🎯可扩展性
  - 通过`trait`s，统一表述并定义「具体结构无关」的抽象特征
- 🎯兼容性
  - 📌基于CommonNarsese特性：在「原子/复合/陈述」的模型中，抽取出有关CommonNarsese所必备的**语义**特征
  - 📄如：Narsese值、词项类别、词项容量、内部词项获取 等概念

#### 标准ASCII词法

（最后更新：2024-04-05）

可作为CommonNarsese词法解析器（同时用于下游NAVM、BabelNAR）的「标准ASCII词法」

该定义兼容[OpenNARS wiki的Narsese语法](https://github.com/opennars/opennars/wiki/Narsese-Grammar-(Input-Output-Format))，
一些不同之处与设计原则在于：

- 📌**一种形式只有一种表达方法**：语义相同的词项，必定有相同的表示形式（重点在对「复合词项」表示形式的限定上）
- 📌不是「内容特定」的：「ASCII词法Narsese」通过「不特定做枚举」给自定义词项（原子/复合/陈述）留足了空间
- 📌任务→语句 单义性：对「缺省预算值的任务」，预算值的缺省总会导致解析器识别成「语句」，不产生任何歧义

使用如下[PEG文法](https://zh.wikipedia.org/wiki/%E8%A7%A3%E6%9E%90%E8%A1%A8%E8%BE%BE%E6%96%87%E6%B3%95)定义：

```pest
/// 空白符 | 所有Unicode空白符，解析前忽略
WHITESPACE = _{ WHITE_SPACE }

/// 总入口：词法Narsese | 优先级：任务 > 语句 > 词项
narsese = {
    task
  | sentence
  | term
}

/// 任务：有预算的语句
task = {
    budget ~ sentence
}

/// 预算值 | 不包括「空字串」隐含的「空预算」
budget = {
    "$" ~ budget_content ~ "$"
}
/// 预算值内容
budget_content = {
    (truth_budget_term ~ (";" ~ truth_budget_term)* ~ ";"*)
  | "" // 空预算（但带括号）
}
/// 通用于真值、预算值的项 | 用作内部数值，不约束取值范围
truth_budget_term = @{(ASCII_DIGIT|".")+}

/// 语句 = 词项 标点 时间戳? 真值?
sentence = {
    term ~ punctuation ~ stamp? ~ truth?
}

/// 词项 = 陈述 | 复合 | 原子
term = {
    statement | compound | atom
}

/// 陈述 = <词项 系词 词项>
statement = {
    "<" ~ term ~ copula ~ term ~ ">"
}

/// 陈述系词
copula = @{
    (punct_sym ~ "-" ~ punct_sym) // 继承/相似/实例/属性/实例属性
  | (punct_sym ~ "=" ~ punct_sym) // 蕴含/等价
  | ("=" ~ punct_sym ~ ">") // 时序性蕴含
  | ("<" ~ punct_sym ~ ">") // 时序性等价
}

/// 标点符号 | 用于「原子词项前缀」「复合词项连接词」和「陈述系词」
punct_sym = { (PUNCTUATION | SYMBOL) }

/// 复合 = (连接词, 词项...) | {外延集...} | [内涵集...]
compound = {
      ("(" ~ connecter ~ "," ~ term ~ ("," ~ term)* ~ ")") // 基于连接词
    | ("{" ~ term ~ ("," ~ term)* ~ "}") // 外延集
    | ("[" ~ term ~ ("," ~ term)* ~ "]") // 内涵集
}

/// 复合词项连接词
connecter = @{ punct_sym ~ (!"," ~ punct_sym)* }

/// 原子 = 前缀（可选） 内容
atom = {
      "_"+ // 占位符
    | (atom_prefix ~ atom_content) // 变量/间隔/操作……
    | atom_content // 词语
}
/// 原子词项前缀
atom_prefix = @{ punct_sym+ }
/// 原子词项内容 | 已避免与「复合词项系词」相冲突
atom_content = @{ atom_char ~ (!copula ~ atom_char)* }
/// 能作为「原子词项内容」的字符
atom_char = { LETTER | NUMBER | "_" | "-" }

/// 标点
punctuation = { (PUNCTUATION | SYMBOL) }

/// 时间戳 | 空时间戳会直接在「语句」中缺省
stamp = {
    ":" ~ (!":" ~ ANY)+ ~ ":"
}

/// 真值 | 空真值会直接在「语句」中缺省
truth = {
  "%" ~ (truth_budget_term ~ (";" ~ truth_budget_term)* ~ ";"*) ~ "%"
}
```

🔗其它另请参考[**JuNarsese.jl**](https://github.com/ARCJ137442/JuNarsese.jl#commonnarsese)中有关「CommonNarsese」的小节。

该定义可直接通过Rust库[**pest.rs**](https://pest.rs/)加载，并已在BabelNAR中用于NARS方言解析

## 开源许可

同大多数Rust项目一样，本项目采用 [MIT](https://choosealicense.com/licenses/mit/) 与 [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/) 双许可发布。

- 可以选择其中任意一种协议进行再分发，但必须保留协议文件。
