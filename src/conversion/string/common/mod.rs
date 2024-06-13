//! 所有「字符串转换」共用的库

// 解析实用结构体
// * ⚠️包含如「NarseseResult」等「不同模块需要特化定义」的符号
//   * 因此不进行重导出
// * 🚩现已删除，因唯一的「NarseseResult」被「NarseseValue」取代

// 字符串格式化模板
// * 进行重导出
nar_dev_utils::pub_mod_and_pub_use! {
    common_narsese_templates
}
