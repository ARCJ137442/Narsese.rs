/// # `first!`：匹配首个判据，并返回其值
/// * 🎯用于简写「截断性判断」结构
///   * 📌可用于简写`if-else if-else`「优先分支」结构
///   * 📌可用于简写`match 0 {_ if XXX => Z1, _ if YYY => Z2, _ => ELSE}`「伪优先分支」结构
///
/// 📝Rust的「规则宏」并不能被视作为一个类似「变量」「函数」之类能导出的量
/// * ❌无法使用常规的`pub`（相当于Julia的`export`）导出
///   * 📌需要使用`#[macro_export]`导出
///     * 📝可选的[`local_inner_macros`]：导出在当前模块中定义的「内部宏」(inner macro)。
///       * 内部宏：仅在其他宏的定义体中使用的宏
/// * ❗需要在crate层级导入，而非在定义宏的模块中导入
/// * 📝使用`#[cfg(not(test))]`标注「非测试」
///   * 🎯可防止「定义之前测试宏」导致的「文档测试（doc test）失败」
///   * ❗但也会导致在别的测试中用不了
///   * 📌SOLUTION：在文档代码块中引入`use 【库名】::*;`
///     * ❗不能用`crate` | `help: consider importing this macro`
///
/// ## 用例
///
/// ```rust
/// use enum_narsese::first;
/// fn see(v: &str) -> &str {
///     // 匹配一个无意义的值，使用匹配守卫来确定「唯一进入的分支」
///     first! {
///         v.is_empty() => "空的！",
///         v.starts_with("0") => "以零开头！",
///         v.starts_with("1") => "以一开头！",
///         v.starts_with("2") => "以二开头！",
///         v.len() > 5 => "超长字符串！",
///         v.starts_with("3") => "以三开头！",
///         _ => "这啥玩意…",
///     }
/// }
/// ```
///
/// 将被转换成
///
/// ```rust
/// fn see(v: &str) -> &str {
///    match 0 {
///         _ if v.is_empty() => "空的！",
///         _ if v.starts_with("0") => "以零开头！",
///         _ if v.starts_with("1") => "以一开头！",
///         _ if v.starts_with("2") => "以二开头！",
///         _ if v.len() > 5 => "超长字符串！",
///         _ if v.starts_with("3") => "以三开头！",
///         _ => "这啥玩意…",
///     }
/// }
/// ```
///
/// 此`match`表达式等价于：
///
/// ```rust
/// fn see(v: &str) -> &str {
///     if v.is_empty() {
///        "空的！"
///     } else if v.starts_with("0") {
///         "以零开头！"
///     } else if v.starts_with("1") {
///         "以一开头！"
///     } else if v.starts_with("2") {
///         "以二开头！"
///     } else if v.len() > 5 {
///         "超长字符串！"
///     } else if v.starts_with("3") {
///         "以三开头！"
///     } else {
///         "这啥玩意…"
///     }
/// }
/// ```
#[macro_export]
macro_rules! first {
    // 第一条规则：最后一条保留`_`
    { // * 📝←左边的括号只是标注「推荐用括弧」而对实际解析无限定作用
        // ↓前边标注片段以「,」重复    后边分隔表达式↓
        $($guardian:expr => $value:expr),* ,
        // ↓对字面标识「_」无需`$(...)`引用
        _ => $value_else:expr $(,)?
    } => {
        // 💭实际上转换为`if-else if-else`亦非不可
        // 匹配无用的字符串常量`0`
        match 0 {
            // ↓这一行插入序列
            $(_ if $guardian => $value,)*
            _ => $value_else,
        }
    }; // ! ←记得分号分隔
    // 「最后一条直接写」的规则会导致「表达式歧义」
    // 📄``local ambiguity when calling macro `first`: multiple parsing options: built-in NTs expr ('value_else') or expr ('guardian').``
}

/// # `show!`：复现Julia的`@show`
/// * 🎯模拟Julia中常用的宏`@show 表达式`
///   * 与Julia`@show(表达式)`等价 | Julia的还更宽松，不强制括号
/// * 📌核心：打印`表达式 = 值`，并返回表达式的值
/// * 📝使用`#[cfg(not(test))]`标注「非测试」
///   * 🎯防止「定义之前测试宏」导致的「文档测试（doc test）失败」
///
/// ## 用例
///
/// ```rust
/// use enum_narsese::show;
/// fn see(v: &str) -> &str {
///     // 用`@show`打印`v`，并返回其值
///     show!(v)
/// }
/// ```
///
/// 将被转换为
///
/// ```rust
/// fn see(v: &str) -> &str {
///     // 用`@show`打印`v`，并返回其值
///     {
///         let value = v;
///         println!("{} = {:?}", "v", value);
///         value
///     }
/// }
/// ```
///
/// 调用`see("我是一个值")`将输出
///
/// ```plaintext
/// v = "我是一个值"
/// ```
///
/// 并返回`"我是一个值"`
#[macro_export]
macro_rules! show {
    ($v:expr) => {
        // 返回一个「多行代码表达式」
        {
            let value = $v;
            println!("{} = {:?}", stringify!($v), value);
            value
        }
    };
}

/// # 辅助用测试宏/批量添加失败测试
///
/// * 可极大减轻添加`should_panic`的代码量
///
/// ! 📝`, $(,)?`这里的「,」代表的不是「分隔表达式」，而是「模式中的`,`」
/// * 故应去除这前边的「,」
///
/// 用法：
///
/// ```rust
/// use enum_narsese::fail_tests;
/// // 一般形式：函数名 {代码}
/// fail_tests! {
///     失败测试的函数名 {
///         // 会导致panic的代码
///     }
///     // ... 允许多条
/// }
/// // 亦可：函数名 表达式/语句
/// fail_tests! {
///     失败测试的函数名 if true {panic!("会导致panic的表达式")} else {};
///     // ... 允许多条
/// }
/// fail_tests! {
///     失败测试的函数名 panic!("会导致panic的语句");
///     // ... 允许多条
/// }
/// ```
///
/// ## 用例
///
/// ```rust
/// use enum_narsese::fail_tests;
/// fail_tests! {
///     fail {
///         panic!("这是一个测试")
///     }
///     fail2 {
///         panic!("这是另一个测试")
///     }
/// }
/// ```
///
/// 将被转换为
///
/// ```rust
/// #[test]
/// #[should_panic]
/// fn fail() {
///     panic!("这是一个测试")
/// }
///
/// #[test]
/// #[should_panic]
/// fn fail2() {
///     panic!("这是另一个测试")
/// }
/// ```
#[macro_export]
macro_rules! fail_tests {
    // 匹配代码块
    ($($name:ident $code:block)* $(,)?) => {
        $(
            /// 失败测试_$name
            #[test]
            #[should_panic]
            fn $name() {
                $code
            }
        )*
    };
    // 匹配表达式
    ($($name:ident $code:expr;)* $(,)?) => {
        $(
            /// 失败测试_$name
            #[test]
            #[should_panic]
            fn $name() {
                $code; // ← 用分号分隔
            }
        )*
    };
    // 匹配语句
    ($($name:ident $code:stmt;)* $(,)?) => {
        $(
            /// 失败测试_$name
            #[test]
            #[should_panic]
            fn $name() {
                $code
            }
        )*
    };
}

/// 用于简化「连续追加字符串」的宏
/// * 🎯最初用于「字符串格式化」算法中
/// * 🚩用法：`push_str!(要追加入的字符串; 待追加表达式1, 待追加表达式2, ...)`
///
/// ## 用例
///
/// ```rust
/// use enum_narsese::push_str;
/// let mut s = String::new();
/// push_str!(
///     &mut s;
///     "这",
///     "是",
///     "可以被",
///     &String::from("连续添加"),
///     "\u{7684}",
/// );
/// assert_eq!(s, "这是可以被连续添加的");
/// ```
#[macro_export]
macro_rules! push_str {
    {$out:expr; $($ex:expr),* $(,)?} => {
        {
            $(
                $out.push_str($ex)
            );*
        }
    };
}

/// 用于将「流式追加」捕捉转换成「固定返回值」
/// * 🎯首次应用于「基于[`String::push_str`]动态追加产生字符串」与「直接返回字符串」的转换中
/// 
/// # Example
///
/// ```rust
/// use enum_narsese::catch_flow;
///
/// fn append(out: &mut String) {
///     out.push_str("hello, ");
///     out.push_str("world!");
/// }
/// 
/// let caught = catch_flow!(append;);
/// assert_eq!(caught, "hello, world!");
/// ```
#[macro_export]
macro_rules! catch_flow {
    ( $($path:ident).+ ; $($arg:tt)* ) => {
        {
            let mut s = String::new();
            $($path).+(&mut s, $($arg)*);
            s
        }
    };
}
