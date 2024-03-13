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
/// * 🎯复刻Julia中常用的宏`@show 表达式`
///   * 相当于Julia`@show(表达式)`，但功能更强大
/// * 📌核心：打印`表达式 = 值`，并（可选地）返回表达式的值
///   * 🚩只有一个表达式⇒计算、打印并返回表达式的值
///   * 🚩多个表达式⇒计算、打印并返回表达式值的元组 | Julia则是返回最后一个值
///   * 🚩一个表达式+尾缀分号⇒计算并打印，**不返回**值
///   * 🚩多个表达式+尾缀分号⇒批量计算并打印，不返回任何值（并且无运行时损耗）
/// * ✅允许尾缀逗号
/// * 📝对于文档测试，必须自包名导入相应的宏以便进行测试
/// * 🔗亦可参考其它实现如[show](https://crates.io/crates/show)
///
/// ## 用例
///
/// ```rust
/// use enum_narsese::show;
/// fn see<'a>(v: &'a str, v2: &'a str) -> (&'a str, &'a str) {
///     // 用`show!`打印`v`、`v2`，不返回值
///     show!(&v, &v2;);
///     // 用`show!`打印`v`，并返回其值
///     show!(v, v2)
/// }
/// ```
///
/// 将被转换为
///
/// ```rust
/// fn see<'a>(v: &'a str, v2: &'a str) -> (&'a str, &'a str) {
///     // 用`show!`打印`v`、`v2`，不返回值
///     println!("{} = {:?}", "&v", (&v));
///     println!("{} = {:?}", "&v2", (&v2));
///     // 用`show!`打印`v`，并返回其值
///     (
///         {
///             let value = v;
///             println!("{} = {:?}", "v", value);
///             value
///         },
///         {
///             let value = v2;
///             println!("{} = {:?}", "v2", value);
///             value
///         },
///     )
/// }
/// ```
///
/// 调用`see("我是一个值", "我是另一个值")`将输出
///
/// ```plaintext
/// &v = "我是一个值"
/// &v2 = "我是另一个值"
/// v = "我是一个值"
/// v2 = "我是另一个值"
/// ```
///
/// 并返回`("我是一个值", "我是另一个值")`
#[macro_export]
macro_rules! show {
    // 单参数：求值、打印、返回
    ($e:expr) => {
        {
            // 求值 | 内部赋值
            let value = $e;
            // 打印
            println!("{} = {:?}", stringify!($e), value);
            // 返回 | 上交值（所有权）
            value
        }
    };
    // 单参数but不返回：求值、打印
    // * ↓注意：末尾使用了分号
    ($e:expr;) => {
        // 直接求值并打印
        println!("{} = {:?}", stringify!($e), $e)
    };
    // 多参数&返回：分别求值&打印，输出到元组
    ($($e:expr),+ $(,)?) => {
        // 构造元组
        ( $( show!($e) ),* )
    };
    // 多参数&不返回：分别求值&打印
    ($($e:expr),+ $(,)?;) => {
        // 直接不构造元组
        $( show!($e;) );*;
    };
}

#[allow(clippy::test_attr_in_doctest)] // * 📝告诉Clippy「这只是用来生成单元测试的示例，并非要运行测试」
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
///
/// * 📝暂时还没法在文档字符串（不管是注释还是#[doc = "..."]）中插值
#[macro_export]
macro_rules! fail_tests {
    // 匹配空块
    {} => {
        // 无操作
    };
    // 匹配代码块
    {$name:ident $code:block $($tail:tt)*} => {
        /// 失败测试 | 代码块
        #[test]
        #[should_panic]
        fn $name() {
            $code
        }
        // 尾递归
        fail_tests!($($tail)*);
    };
    // 匹配表达式
    {$name:ident $code:expr; $($tail:tt)*} => {
        /// 失败测试 | 表达式
        #[test]
        #[should_panic]
        fn $name() {
            $code; // ← 用分号分隔
        }
        // 尾递归
        fail_tests!($($tail)*);
    };
    // 匹配语句
    {$name:ident $code:stmt; $($tail:tt)*} => {
        /// 失败测试 | 语句
        #[test]
        #[should_panic]
        fn $name() {
            $code
        }
        fail_tests!($($tail)*);
    };
}

/// 用于简化「连续判断相等」的宏
/// * 🎯用于统一
///   * ⚠️缺点：不易定位断言出错的位置（需要靠断言的表达式定位）
/// * 🚩模型：标记树撕咬机
///   * ⚠️缺点：无法一次性展开
///   * 🔗参考：<https://www.bookstack.cn/read/DaseinPhaos-tlborm-chinese/pat-incremental-tt-munchers.md>
///
/// # 用例
///
/// ```rust
/// use enum_narsese::asserts;
/// asserts! {
///     1 + 1 > 1, // 判真
///     1 + 1 => 2, // 判等
///     1 + 1 < 3 // 连续
///     1 + 2 < 4, // 判真（与「判等」表达式之间，需要逗号分隔）
///     1 + 2 => 3 // 连续
///     2 + 2 => 4 // 判等（其间无需逗号分隔）
/// }
/// ```
#[macro_export]
macro_rules! asserts {
    // 连续判等逻辑（无需逗号分隔）
    {
        $($left:expr => $right:expr $(,)?)*
    } => {
        $(
            assert_eq!($left, $right, "{} != {}", stringify!($left), stringify!($right));
        )*
    };
    // 连续判真逻辑（无需逗号分隔）
    {
        $($assertion:expr $(,)?)*
    } => {
        $(
            assert!($assertion, "{} != true", stringify!($assertion));
        )*
    };
    // 新形式/空
    {} => {
        // 无操作
    };
    // 新形式/判真
    {
        $($assertion:expr)*,
        $($tail:tt)*
    } => {
        // 分派到先前情形
        asserts!($($assertion)*);
        // 尾递归
        asserts!($($tail)*)
    };
    // 新形式/判等
    {
        $($left:expr => $right:expr)*,
        $($tail:tt)*
    } => {
        // 分派到先前情形
        asserts!($($left => $right)*);
        // 尾递归
        asserts!($($tail)*)
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

/// 更通用的「函数参数张量展开」宏
/// * 🎯用于最终版简化一系列「笛卡尔积式组合调用」
/// * 🚩【2024-03-09 15:01:24】与「函数参数矩阵展开」宏合并
///   * 📌后者（矩阵）可看作「二维张量」
/// * ⚠️对「内部转换」`@inner`的规定性约束：
///   * 🚩统一使用方括号，避免「圆括号→表达式值」的歧义
///   * 🚩统一使用逗号分隔（强制尾后逗号），避免「连续圆括号→函数调用」的歧义
///
/// # Example
///
/// ```rust
/// use enum_narsese::f_tensor;
/// fn add(a: i32, b: i32) -> i32 {
///     a + b
/// }
/// fn add3(a: i32, b: i32, c: i32) -> i32 {
///     a + b + c
/// }
///
///  // fallback情况
/// let m = f_tensor!(@inner [add] [1, 2,]);
/// assert_eq!(m, 3);
///
///  // fallback情况 2
/// let m = f_tensor!(@inner [add] [1, 2,] []);
/// assert_eq!(m, 3);
///
///  // 正常情况
/// let m1 = f_tensor![add [1 2] [3 4 5]];
/// let m2 = f_tensor![add3; 1 2; 3 4; 5 6];
/// // 📌↓此处对「括号表达式」可用逗号明确分隔，以避免「函数调用」歧义
/// let m3 = f_tensor![add3 [(2-1), (1+1)] [3 4] [5 6]];
///
/// assert_eq!(m1, [[4, 5, 6], [5, 6, 7]]);
/// assert_eq!(
///     m2,
///     // ↓展开结果
///     [
///         [[1 + 3 + 5, 1 + 3 + 6], [1 + 4 + 5, 1 + 4 + 6]],
///         [[2 + 3 + 5, 2 + 3 + 6], [2 + 4 + 5, 2 + 4 + 6]],
///     ]
/// );
/// // ↓计算结果
/// assert_eq!(m3, [[[9, 10], [10, 11]], [[10, 11], [11, 12]],]);
/// ```
///
/// # Experiences
///
/// * 📝使用「前缀特殊标识符」控制宏匹配时的分派路径
///   * 💭此举特别像Julia的多分派系统
/// * 📝涉及「嵌套笛卡尔积展开」时，把其它变量都变成一个维度，在一次调用中只展开一个维度
///   * 🚩源自GitHub Issue的方法
///     * 1 先使用「数组」之类的包装成一个令牌树（tt）
///     * 2 展开另一个维度
///     * 3 再将原先包装的维度解包
///
/// # References
///
/// * 🔗宏小册「使用`@`标识子分派」<https://www.bookstack.cn/read/DaseinPhaos-tlborm-chinese/aeg-ook.md>
/// * 🔗开发者论坛：<https://users.rust-lang.org/t/why-is-the-innermost-meta-variable-expansion-impacted-by-the-outmost-one/99099/4>
/// * 🔗GitHub Issue：<https://github.com/rust-lang/rust/issues/96184>
#[macro_export]
macro_rules! f_tensor {
    // 入口/空格分号形式 | 可选逗号进行无歧义分隔
    // * f_tensor![f; 1 2 3; 4 5 6]
    [
        // 要被调用的函数（标识符）
        $($path:ident).+;
        // 参数的表达式序列
        $($($arg:expr $(,)?)+);+ $(;)?
    ] => {
        // * 0 包装后边的参数到数组（这样后续可以用tt替代）
        f_tensor![
            $($path).* $( [ $($arg)+ ] )+
        ]
    };
    // 入口/数组形式（内外桥梁） | 可选逗号进行无歧义分隔
    // * `f_tensor![f [1 2 3] [4 5 6]]` => ``f_tensor![[f] [] [[1, 2, 3,] [4, 5, 6,]]]``
    [
        // 要被调用的函数（标识符序列）
        $($path:ident).+
        // 参数的表达式序列
        $( [ $($arg:expr $(,)? )+ ] )+
    ] => {
        // * 1 开始解析
        f_tensor![
            // 加上标识符
            @inner
            // 将「被调用函数」打包（以便支持如`self.add`的表达形式）
            [$($path).+]
            // 空参数集（未开始填充）
            []
            // 包装：`([参数集1], [参数集2] ...)`
            [ $( [ $($arg,)+ ], )+ ]
        ]
    };
    // 【内部】「纯参数」fallback情况
    // * `f_tensor![[f] [1, 2, 3,]]` => `f(1, 2, 3)`
    [
        // 内部标识符
        @inner
        // 要被调用的函数（已作`[fn]`包装，此处解包）
        [ $($f:tt)+ ]
        // 只有一个表达式序列
        [ $($arg:expr,)+ ]
    ] => {
        // 直接解包
        $($f)* ($($arg),+)
    };
    // 【内部】参数+空括号 情况
    // * `f_tensor![[f] [1, 2, 3,] []]` => `f_tensor![[f] [1, 2, 3,]]`
    [
        // 内部标识符
        @inner
        // 要被调用的函数（已作`[fn]`包装）
        $f:tt
        // 表达式序列
        $args:tt
        // 空括号
        []
    ] => {
        // 去掉空括号
        f_tensor![@inner $f $args]
    };
    // 【内部】参数+参数 情况
    // * `f_tensor![[f] [1, 2, 3,] [[x1, x2, ...x,] ...tail]]` => `...f_tensor![[f] [1, 2, 3, x,] [...tail]]`
    [
        // 内部标识符
        @inner
        // 要被调用的函数（已作`[fn]`包装）
        $f:tt
        // 表达式序列（此处延迟解包，留给后边的`append`）
        $args_head:tt
        // [[参数头...] 其它参数...]
        [ [ $($x:expr,)+ ], $($tail:tt)* ]
    ] => {
        // * 解构，留给专门的函数进行展开（因为x和tail不能同时展开）
        f_tensor![
            // 调用新函数
            @inner_expand
            // 直接传递被调用者
            $f
            // 直接传递表达式序列（后续「展开」「追加」要一次完成）
            $args_head
            // 提取x序列
            [ $($x,)+ ]
            // 去头 | 先展开tail
            [ $($tail)* ]
        ]
    };
    // * 【内部】工具分派/展开
    [
        // 内部标识符
        @inner_expand
        // 要被调用的函数（已作`[fn]`包装）
        $f:tt
        // 表达式序列（此处延迟解包，留给后边的`append`）
        $args_head:tt
        // 提取的x序列（预备展开）
        [ $($x:expr,)+ ]
        // (其它参数...)
        $other_args:tt
    ] => {
        // * 开始【展开】一个维度
        [
            $(f_tensor![
                // 在展开之后专门追加
                @inner_append
                // 直接传递被调用者
                $f
                // 表达式序列原样传递
                $args_head
                // ! 这里不能「宏套宏」：「表示『追加』的宏调用」被认成表达式了
                // f_tensor!( @append $args_head $x )
                [ $x ] // 提取出来的x
                // 留下的尾部序列
                $other_args
            ]),+
        ]
    };
    // * 【内部】工具分派/追加
    [
        // 内部标识符
        @inner_append
        // 要被调用的函数（已作`[fn]`包装）
        $f:tt
        // 表达式序列（此处解包）
        [ $($arg_head:expr,)* ]
        // 提取的x
        [ $x:expr ]
        // (其它参数...)
        $other_args:tt
    ] => {
        f_tensor![
            // 回到原先的展开进程
            @inner
            // 直接传递被调用者
            $f
            // 展开的参数【追加】到函数参数序列中
            [ $($arg_head,)* $x, ]
            // 留下的尾部序列
            $other_args
        ]
    };
}

/// 平行将参数填充进函数
/// * 📄形式：`f_parallel![add3; 1 2 3; 4 5 6]` => `[add3(1, 2, 3), add3(4, 5, 6)]`
///
/// # Example
///
/// ```rust
/// use enum_narsese::f_parallel;
/// fn add3(a: i32, b: i32, c: i32) -> i32 {
///     a + b + c
/// }
/// let m = f_parallel![
///     add3;
///     1 2 3; // add3(1, 2, 3)
///     4 5 6; // add3(4, 5, 6)
///     7, (8) 9; // add3(7, 8, 9) // ! 📌此处使用逗号避免「调用歧义」`7(8)`
/// ];
/// assert_eq!(m, [6, 15, 24]);
/// ```
///
#[macro_export]
macro_rules! f_parallel {
    // 入口/空格分号形式 | 可选逗号进行无歧义分隔
    [
        // 要被调用的函数（标识符）
        $($path:ident).+;
        // 参数的表达式序列 // ! ↓此处必须限制为「+」，不然无法实现「尾后分号」（会引发解析歧义）
        $( $( $arg:expr $(,)? )+ );* $(;)?
    ] => {
        // * 🚩先封装好：`f_parallel![add3; 1 2 3; 4 5 6]` => `f_parallel![@inner [add3] [1, 2, 3,] [4, 5, 6,]]`
        f_parallel![
            // 内部标识符
            @inner
            // 要被调用的函数（标识符序列）
            [$($path).+]
            // 参数的表达式序列
            $( [ $($arg,)* ] )*
        ]
    };
    // 【内部】先展开参数
    // * `f_parallel![@inner [add3] [1, 2, 3,] [4, 5, 6,]]` => `f_parallel![@inner [add3] [1, 2, 3,] [4, 5, 6,]]`
    [
        @inner
        // 要被调用的函数（标识符）
        $f:tt
        // 参数的表达式序列的序列
        $( [ $($arg:expr,)* ] )*
    ] => {
        [
            $(f_parallel![
                // 内部标识符
                @inner_expand
                // 要被调用的函数（已作`[fn]`包装）
                $f
                // 参数的表达式序列
                [ $($arg,)+ ]
            ]),*
        ]
    };
    // 【内部】再展开函数表达式
    [
        @inner_expand
        // 要被调用的函数（标识符）
        [ $($f:tt)* ]
        // 参数的表达式序列
        [ $($arg:expr,)* ]
    ] => {
        $($f)* ($($arg),+)
    };
}
