//! 构建一个简单的REPL
//! 🎯用于格式化展示一个Narsese对象
//! * 基于「枚举Narsese」实现
#![cfg(feature = "enum_narsese")]

use std::io::{stdin, stdout, Stdin, Write};

use enum_narsese::conversion::string::{
    format_instances::FORMAT_ASCII, impl_enum::NarseseResult, NarseseFormat,
};

/// REPL主函数
fn main() {
    // 指定格式
    const FORMAT: NarseseFormat<&str> = FORMAT_ASCII;

    // 构造输入与缓冲区
    let io = stdin();
    let mut buffer = String::new();

    // 无限循环的REPL
    loop {
        // 读取文本 | 📌【2024-02-22 15:54:50】目前只需读取一行
        input_line(&io, &mut buffer, "narsese> ");
        // 预处理文本
        let to_parse = buffer.trim();
        // 为空⇒退出
        if to_parse.is_empty() {
            break;
        }
        // 解析文本 & 处理结果
        match FORMAT.parse(to_parse) {
            // 解析成功⇒debug输出CommonNarsese结构
            Ok(result) => {
                // 根据结果分派信息
                match result {
                    NarseseResult::Term(value) => println!("[词项] {value:#?}"),
                    NarseseResult::Sentence(value) => println!("[语句] {value:#?}"),
                    NarseseResult::Task(value) => println!("[任务] {value:#?}"),
                }
            }
            // 解析失败⇒输出错误信息
            Err(e) => {
                println!("解析失败！\n被解析文本：{to_parse:?}\n{e}");
            }
        }
        // 清空缓冲区
        buffer.clear();
        // 打印下一行
        println!();
    }
    // 程序退出
    println!("一秒后程序将退出。。。");
    // 最后等待一秒
    std::thread::sleep(std::time::Duration::from_secs(1))
}

// 输入输出实用库 //

/// 从给定的「标准输入」输入一行
/// * 📝需要立即【刷新】标准输出，否则无法在读取前打印
///   * 📄参见[`stdout`]
pub fn input_line(io: &Stdin, buffer: &mut String, prompt: &str) {
    // 打印提示词 // ! 但还没完
    print!("{prompt}");
    // ↓此处需要使用`stdout().flush()`刷新标准输出，以便立即打印文本
    stdout().flush().expect("标准输出无法写入！");
    // 从输入中获取一行
    io.read_line(buffer).expect("标准输入无法读取行！");
}
