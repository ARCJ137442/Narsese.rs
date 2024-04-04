//! 构建一个简单的REPL
//! 🎯用于格式化展示一个Narsese对象
//! * 基于「枚举Narsese」实现
#![cfg(feature = "enum_narsese")]
#![cfg(feature = "lexical_narsese")]

use narsese::{
    api::NarseseValue,
    conversion::string::{
        impl_enum::{format_instances::*, NarseseFormat},
        impl_lexical::{
            format_instances::{
                FORMAT_ASCII as FORMAT_ASCII_LEXICAL, FORMAT_HAN as FORMAT_HAN_LEXICAL,
                FORMAT_LATEX as FORMAT_LATEX_LEXICAL,
            },
            NarseseFormat as NarseseFormatLexical,
        },
    },
};
use std::io::{stdin, stdout, Stdin, Write};

/// 格式化模式
/// * 🎯允许展示器切换多种模式
pub enum FormatMode {
    EnumAscii,
    EnumLatex,
    EnumHan,
    LexicalAscii,
    LexicalLatex,
    LexicalHan,
}
use FormatMode::*;

impl FormatMode {
    /// 获取格式名称
    pub fn name(&self) -> &str {
        match self {
            EnumAscii => "枚举-ASCII",
            EnumLatex => "枚举-LaTeX",
            EnumHan => "枚举-漢文",
            LexicalAscii => "词法-ASCII",
            LexicalLatex => "词法-LaTeX",
            LexicalHan => "词法-漢文",
        }
    }

    /// （解析并）展示Narsese
    pub fn demonstrate(&self, narsese_str: &str) {
        match self {
            EnumAscii => Self::_demonstrate_enum(narsese_str, &FORMAT_ASCII),
            EnumLatex => Self::_demonstrate_enum(narsese_str, &FORMAT_LATEX),
            EnumHan => Self::_demonstrate_enum(narsese_str, &FORMAT_HAN),
            LexicalAscii => Self::_demonstrate_lexical(narsese_str, &FORMAT_ASCII_LEXICAL),
            LexicalLatex => Self::_demonstrate_lexical(narsese_str, &FORMAT_LATEX_LEXICAL),
            LexicalHan => Self::_demonstrate_lexical(narsese_str, &FORMAT_HAN_LEXICAL),
        }
    }

    /// （解析并）展示枚举Narsese
    fn _demonstrate_enum(narsese_str: &str, format: &NarseseFormat<&str>) {
        match format.parse(narsese_str) {
            // 解析成功⇒debug输出CommonNarsese结构
            Ok(value) => {
                // 根据结果分派信息
                match value {
                    NarseseValue::Term(value) => println!("[词项] {value:#?}"),
                    NarseseValue::Sentence(value) => println!("[语句] {value:#?}"),
                    NarseseValue::Task(value) => println!("[任务] {value:#?}"),
                }
            }
            // 解析失败⇒输出错误信息
            Err(e) => {
                println!("解析失败！\n被解析文本：{narsese_str:?}\n{e}");
            }
        }
    }

    /// （解析并）展示词法Narsese
    fn _demonstrate_lexical(narsese_str: &str, format: &NarseseFormatLexical) {
        match format.parse(narsese_str) {
            // 解析成功⇒debug输出CommonNarsese结构
            Ok(value) => {
                // 根据结果分派信息
                match value {
                    NarseseValue::Term(value) => println!("[词项] {value:#?}"),
                    NarseseValue::Sentence(value) => println!("[语句] {value:#?}"),
                    NarseseValue::Task(value) => println!("[任务] {value:#?}"),
                }
            }
            // 解析失败⇒输出错误信息
            Err(e) => {
                println!("解析失败！\n被解析文本：{narsese_str:?}\n{e}");
            }
        }
    }
}

/// 所有格式化模式
/// * 🎯用于循环遍历
const FORMAT_MODES: &[FormatMode] = &[
    EnumAscii,
    EnumLatex,
    EnumHan,
    LexicalAscii,
    LexicalLatex,
    LexicalHan,
];

/// REPL主函数
fn main() {
    // 指定格式
    let mut format_mode_i = 0;

    // 构造输入与缓冲区
    let io = stdin();
    let mut buffer = String::new();

    // 无限循环的REPL
    loop {
        // 读取文本 | 📌【2024-02-22 15:54:50】目前只需读取一行
        input_line(&io, &mut buffer, "narsese> ");
        // 预处理文本
        let to_parse = buffer.trim();
        // 为空⇒切换模式 | 📌【2024-04-05 03:20:58】退出可以`Ctrl+C`代替
        if to_parse.is_empty() {
            format_mode_i += 1;
            format_mode_i %= FORMAT_MODES.len();
            println!("\n已切换模式到「{}」", FORMAT_MODES[format_mode_i].name());
            continue;
        }
        // 解析文本 & 处理结果
        FORMAT_MODES[format_mode_i].demonstrate(to_parse);

        // 清空缓冲区
        buffer.clear();
        // 打印下一行
        println!();
    }
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
