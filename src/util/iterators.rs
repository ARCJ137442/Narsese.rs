use std::collections::VecDeque;

/// 缓冲迭代器
/// * 🎯最初用于「只会从前往后解析字符串，除了『缓冲区』不会进行回溯」的字符串解析器
/// * 🚩用于**带缓冲地从某个迭代器里迭代东西**
///
/// ! ⚠️【2024-03-03 23:29:48】目前因为「需要迭代出去，同时还要缓存」要求其内元素可以被复制（实现[`Clone`]，如[`char`]）
///   * 因此，该迭代器会**自动复制**其所封装迭代器中的元素
pub struct BufferIterator<T, I>
where
    T: Clone,
    I: Iterator<Item = T>,
{
    iterator: I,
    /// 记录「已迭代未清理」的元素
    /// * 🚩使用**队列**以便在「缓冲区递进」时弹出元素
    buffer: VecDeque<T>,
    /// 记录迭代到的「头索引」（缓冲区末尾）
    /// * 可能为空：尚未开始迭代时（最开始迭代将设置在0）
    ///
    /// ! ⚠️不同于「缓冲区开头」所迭代到的索引
    head: usize,
    /// 是否开始迭代
    /// * 🎯为了在获取「头索引」时避免「获取空迭代器的头索引」
    is_began: bool,
    /// 是否迭代到了末尾
    /// * 🎯为了在获取「是否迭代完」时不修改迭代器
    is_ended: bool,
}

impl<T, I> BufferIterator<T, I>
where
    T: Clone,
    I: Iterator<Item = T>,
{
    /// 构造函数
    /// * 📌`head`初始为`0`，`is_began`初始为`false`，`is_ended`初始为`false`
    /// * 📌`buffer`初始为空
    pub fn new(iterator: I) -> Self {
        BufferIterator {
            // 载入迭代器
            iterator,
            buffer: VecDeque::new(),
            // 头索引初始化为0
            head: 0,
            // 未开始迭代，未结束迭代
            is_began: false,
            is_ended: false,
        }
    }

    /// 获取「头索引」
    /// * 📌当【缓冲区非空】时，不会随[`Self::buffer_next`]的调用而改变
    /// * ⚠️不是「缓冲区开头」所在的索引
    ///   * 后者为「缓冲区头索引」[`Self::buffer_head`]
    /// * ⚠️当自身【未开始迭代】时，「头索引」仍然为`0`
    pub fn head(&self) -> usize {
        self.head
    }

    /// 获取「缓冲区头索引」
    /// * 🚩是「缓冲区开头」所在的索引
    /// * 📌不会随[`Self::next`]的调用而改变
    /// * ⚠️当自身【未开始迭代】时，「缓冲区头索引」为`0`
    ///   * 📌「缓冲区长度」永远不会大于「头索引+1」
    ///   * 📌这也说明：**当「缓冲区头索引>头索引」时，缓冲区为空**
    pub fn buffer_head(&self) -> usize {
        (self.head + 1) - self.buffer.len()
    }

    /// 获取「是否已开始」
    pub fn is_began(&self) -> bool {
        self.is_began
    }

    /// 获取「是否迭代完」
    pub fn is_ended(&self) -> bool {
        self.is_ended
    }

    /// 获取「缓冲区长度」
    pub fn len_buffer(&self) -> usize {
        self.buffer.len()
    }

    /// 判断「缓冲区是否为空」
    pub fn is_buffer_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// 缓冲区迭代
    /// * 🚩分「缓冲区是否为空」执行
    ///   * 缓冲区为空⇒通过「内部迭代器」迭代（调用[`Iterator::next`]）
    ///   * 缓冲区非空⇒从缓冲区头部取出一个元素（先进先出），并返回
    pub fn buffer_next(&mut self) -> Option<T> {
        match self.is_buffer_empty() {
            // 缓冲区为空⇒自身迭代出元素（并可能地随之存进缓冲区）
            true => self.next(),
            // 缓冲区非空⇒从缓冲区头部取出元素
            false => self.buffer.pop_front(),
        }
        // ! 此处无需处理「缓冲区索引」：会自动计算
    }

    /// 缓冲区迭代器（不可变引用）
    pub fn buffer_iter(&self) -> impl Iterator<Item = &T> {
        self.buffer.iter()
    }

    /// 缓冲区迭代器（可变引用）
    pub fn buffer_iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.buffer.iter_mut()
    }

    /// 缓冲区清空
    /// * 📌「缓冲区头索引」会自动更新
    pub fn buffer_clear(&mut self) {
        self.buffer.clear();
    }

    /// 缓冲区转移（从前往后）
    /// * 🎯在「清空缓冲区」时，需要使用被清空的元素
    /// * 📌其内元素均转移给参数`f`
    /// * 📌「缓冲区头索引」会自动更新
    pub fn buffer_transfer(&mut self, f: impl Fn(T)) {
        // 清除「缓冲区长度」个元素，即清除所有元素
        for _ in 0..self.len_buffer() {
            f(self.buffer.pop_front().unwrap());
        }
    }

    /// 缓冲区转移（从前往后，可变）
    /// * 🎯在「清空缓冲区」时，需要使用被清空的元素，并且过程中会修改其它对象（如「将元素加入某个数组」）
    /// * 📌其内元素均转移给参数`f`
    pub fn buffer_transfer_mut(&mut self, mut f: impl FnMut(T)) {
        // 清除「缓冲区长度」个元素，即清除所有元素
        for _ in 0..self.len_buffer() {
            f(self.buffer.pop_front().unwrap());
        }
    }
}

/// 实现迭代器接口，兼容[`Self::next`]方法
impl<T, I> Iterator for BufferIterator<T, I>
where
    T: Clone,
    I: Iterator<Item = T>,
{
    type Item = T;

    /// 迭代：迭代出元素存进缓冲区
    fn next(&mut self) -> Option<Self::Item> {
        // 从封装的迭代器中迭代出一个元素
        let item = self.iterator.next();
        // 判断是否结束
        match (self.is_began, &item) {
            // 未开始，将要继续 | 第一个元素
            (false, Some(item)) => {
                // 设置「已经开始」
                self.is_began = true;
                // 复制并存入缓冲区
                self.buffer.push_back(item.clone());
                // 头索引不变
            }
            // 已开始，正在中途
            (true, Some(item)) => {
                // 头索引递增
                self.head += 1;
                // 复制并存入缓冲区
                self.buffer.push_back(item.clone());
            }
            // 将要结束
            (_, None) => {
                // 设置「已经结束」
                self.is_ended = true;
            }
        }
        // 返回迭代出的元素
        item
    }
}

/// 对额外实现了[`PartialEq`]的元素实现「以指定迭代元素开头」等方法
impl<T, I> BufferIterator<T, I>
where
    T: Clone + PartialEq,
    I: Iterator<Item = T>,
{
    /// 判断是否以`other_iter`的元素开头
    /// * 🚩从「缓冲区头索引」开始：**优先使用缓冲区内元素**，比对完了再从「内部迭代器」中拿取元素
    ///   * 最多可能新拿取`other_iter.count()`个元素（**比对者长度**）
    /// * 🎯用于在语法解析中实现「前缀匹配」
    /// * ⚠️会改变缓冲区，且不区分「因不匹配而『非前缀』」与「因迭代完而『非前缀』」
    pub fn starts_with(&mut self, mut other_iter: impl Iterator<Item = T>) -> bool {
        // 先比对缓冲区中的元素（不会改变自身） | 此时「比对者」相对未知
        for item_self in &self.buffer {
            // ! ↑此处`item_self`不能加`&`，只需在需要比对时解引用
            // 从「比对者」中取出元素以对比
            match other_iter.next() {
                // 在`false`之前就没有⇒返回`true`
                None => return true,
                // 比对失败⇒返回`false`
                Some(item_other) if *item_self != item_other => return false,
                // 比对成功⇒继续
                _ => {}
            }
        }
        // 再从自身拿出来比对 | 此时「自身」相对未知
        for item_other in other_iter {
            // 从自身中取出元素以对比
            match self.next() {
                // 自身长度不够⇒返回`false`
                None => return false,
                // 比对失败⇒返回`false`
                Some(item_self) if item_self != item_other => return false,
                // 比对成功⇒继续
                _ => {}
            }
        }
        // 比对都没失败⇒成功⇒`true`
        true
    }

    /// 若以`other_iter`的元素开头⇒跳过元素
    /// * 🚩仍然会返回「是否 匹配+跳过 成功」
    /// * 📌虽然要求「比对者长度」已知，但「比对者长度」在[`Self::starts_with`]返回`true`时已蕴含「比对者长度已知」
    ///   * 🚩因此使用[`Iterator::map`]封装计数逻辑，并消耗迭代器
    /// * 🚩比对成功后，使用「缓冲区递进」[`Self::buffer_next`]跳过元素
    ///   * 📌因为是从缓冲区开始比对的
    pub fn skip_when_starts_with(&mut self, other_iter: impl Iterator<Item = T>) -> bool {
        let mut c: usize = 0;
        if self.starts_with(other_iter.map(|v| {
            // 边迭代边计数
            c += 1;
            v
        })) {
            // 跳过比对者
            for _ in 0..c {
                self.buffer_next();
            }
            // 返回「比对并跳过成功」
            return true;
        }
        // 返回「比对失败」
        false
    }
}

/// 单元测试
#[cfg(test)]
mod tests {
    use crate::assert_eqs;

    use super::*;

    /// 一次性消耗掉迭代器
    #[test]
    fn iter_char_overview() {
        let test_set = [
            "abcd",
            "我是一个迭代器",
            r"/rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library\std\src\panicking.rs:645",
            "⚠️注意：不能使用`collect`❗，🤔其会获取迭代器的所有权（导致无法知晓「迭代后的状态」）",
        ];
        for test_str in test_set {
            _iter_char_overview(test_str);
        }
    }

    fn _iter_char_overview(s: &str) {
        // ✨创建迭代器
        let mut iter = BufferIterator::new(s.chars());

        // ! ⚠️注意：不能使用`collect`，其会获取迭代器的所有权（导致无法知晓「迭代后的状态」）
        assert_eqs! {
            // 迭代之前
            iter.head() => 0 // 此时头索引为`0`（但实际上是「未开始迭代」的状态）
            iter.is_began() => false // 还没开始迭代
            iter.is_ended() => false // 还没终止迭代
            iter.len_buffer() => 0 // 此时缓冲区长度为`0`
            iter.is_buffer_empty() => true // 此时缓冲区为空
            iter.buffer_head() => 1 // 此时缓冲区头索引为`1`
        }

        // 一次性迭代完元素
        let mut to = String::new();
        for c in &mut iter {
            to.push(c);
        }
        // ! 📝字符串长度 ≠ 字符长度（字符个数）
        let len_chars_to = to.chars().count();

        // 迭代之后
        assert_eqs! {
            to => s // 迭代到字符串中，仍然保持原样
            iter.head() => len_chars_to - 1 // 此时头索引为「字符长度-1」（终态）
            iter.is_began() => true // 已经开始迭代
            iter.is_ended() => true // 已经终止迭代
            iter.len_buffer() => len_chars_to // 此时缓冲区长度为「字符长度」
            iter.is_buffer_empty() => false // 此时缓冲区非空
            iter.buffer_head() => 0 // 此时缓冲区头索引为`0`（因为没消耗缓冲区）
        }

        // 再清空缓冲区
        iter.buffer_clear();

        assert_eqs! {
            iter.head() => len_chars_to - 1 // 此时头索引不变（终态）
            iter.is_began() => true // 已经开始迭代
            iter.is_ended() => true // 已经终止迭代
            iter.len_buffer() => 0 // 此时缓冲区长度清零
            iter.is_buffer_empty() => true // 此时缓冲区为空
            iter.buffer_head() => len_chars_to // 此时缓冲区头索引为「字符长度」，为空⇔比「头索引」大`1`
        }
    }

    /// 一步步测试迭代器
    #[test]
    fn iter_char_per_step() {
        // ✨创建迭代器
        let mut iter = BufferIterator::new("abcd".chars());

        // ! 尽可能不要尝试在「开始迭代前」获取「头索引」
        assert_eqs! {
            iter.head() => 0 // 此时头索引为`0`（但实际上是「未开始迭代」的状态）
            iter.is_began() => false // 还没开始迭代
            iter.is_ended() => false // 还没终止迭代
            iter.len_buffer() => 0 // 此时缓冲区长度为`0`
            iter.is_buffer_empty() => true // 此时缓冲区为空
            iter.buffer_head() => 1 // 此时缓冲区头索引为`1`
        }

        // 迭代器【迭代】一次 // ! 迭代出的字符【存进缓冲区】，头也【不移动】
        let a = iter.next();

        assert_eqs! {
            a => Some('a') // 应该读取到第一个字符
            iter.head() => 0 // 此时头索引在`0`
            iter.is_began() => true // 此时已开始迭代
            iter.is_ended() => false // 此时未迭代终止
            iter.is_buffer_empty() => false // 此时缓冲区非空
            iter.len_buffer() => 1 // 此时缓冲区长度为`1`
            iter.buffer_head() => 0 // 此时缓冲区头索引在`0`（缓冲区只有第一个）
        }

        // 迭代器【缓冲区迭代】一次 // ! 此时因为缓冲区已缓存，所以缓冲区消耗并返回最前一个字符`'a'`
        let a2 = iter.buffer_next();

        assert_eqs! {
            a2 => Some('a') // 应该把缓存的第一个字符弹出
            iter.head() => 0 // 此时头索引不变
            iter.is_began() => true // 此时已开始迭代
            iter.is_ended() => false // 此时仍未结束
            iter.is_buffer_empty() => true // 此时缓冲区为空
            iter.len_buffer() => 0 // 此时缓冲区长度为`0`
            iter.buffer_head() => 1 // 此时「缓冲区索引」变为`1`
        }

        // 迭代器再次【缓冲区迭代】 // ! 此时因为缓冲区【为空】，所以进行了「迭代」
        let b = iter.buffer_next();

        assert_eqs! {
            b => Some('b') // 此时没有缓存了，所以迭代出了新字符
            iter.head() => 1 // 此时头索引变为`1`
            iter.is_began() => true // 此时已开始迭代
            iter.is_ended() => false // 此时仍未结束
            iter.is_buffer_empty() => false // 此时缓冲区非空
            iter.len_buffer() => 1 // 此时缓冲区长度为`1`
            iter.buffer_head() => 1 // 此时「缓冲区索引」仍然为`1`
        }

        // 迭代器将缓冲区【转移】给了`vb`
        let mut vb = String::new();
        iter.buffer_transfer_mut(|c| vb.push(c));

        assert_eqs! {
            vb => "b" // 缓冲区内容为"b"
            iter.head() => 1 // 此时头索引还是`1`
            iter.is_began() => true // 此时已开始迭代
            iter.is_ended() => false // 此时仍未结束
            iter.is_buffer_empty() => true // 此时缓冲区为空
            iter.len_buffer() => 0 // 此时缓冲区长度为`0`
            iter.buffer_head() => 2 // 此时「缓冲区索引」增加到`2`
        }

        // 迭代器测试后续是否以"c" "cd" "不会比对成功"开头，在此中将'c'、'd'加入缓冲区
        let starts_with_cd = iter.starts_with("cd".chars());
        let starts_with_c = iter.starts_with("c".chars());
        let starts_with_不会比对成功 = iter.starts_with("不会比对成功".chars());

        assert_eqs! {
            starts_with_cd => true // 的确是以"cd"开头 | 比对者比缓冲区长
            starts_with_c => true // 的确是以"c"开头 | 比对者在缓冲区内
            starts_with_不会比对成功 => false // 的确不以"不会比对成功"开头 | 比对者超出自身界限
            iter.head() => 3 // 此时头索引更新到了`3`——为了「前缀匹配」一直在增加索引
            iter.is_began() => true // 此时已开始迭代
            iter.is_ended() => false // 此时仍未结束 | 临界状态：还未继续调用`next`方法
            iter.is_buffer_empty() => false // 此时缓冲区非空
            iter.len_buffer() => 2 // 此时缓冲区长度为`2`
            iter.buffer_head() => 2 // 此时「缓冲区索引」不变
        }

        // 测试"c"开头，并（在缓冲区里）跳过它
        let skipped = iter.skip_when_starts_with("c".chars());

        assert_eqs! {
            skipped => true // 的确是以"c"开头并跳过了
            iter.head() => 3 // 此时头索引不变——比对没有超出缓冲区
            iter.is_began() => true // 此时已开始迭代
            iter.is_ended() => false // 此时仍未结束 | 临界状态：还未继续调用`next`方法
            iter.is_buffer_empty() => false // 此时缓冲区非空
            iter.len_buffer() => 1 // 此时缓冲区长度减少到`1`（跳过了"c"）
            iter.buffer_head() => 3 // 此时「缓冲区索引」增加到`3`（跳过了"c"）
        }

        // 迭代器走到尽头
        let none = iter.next();

        assert_eqs! {
            none => None // 已经没有可迭代的了
            iter.head() => 3 // 此时头索引不变
            iter.is_began() => true // 此时已开始迭代
            iter.is_ended() => true // 此时已经结束 | 刚好超过
            iter.is_buffer_empty() => false // 此时缓冲区非空
            iter.len_buffer() => 1 // 此时缓冲区长度不变
            iter.buffer_head() => 3 // 此时「缓冲区索引」不变
        }

        // 最后的缓冲区转交
        let mut d = String::new();
        iter.buffer_transfer_mut(|c| d.push(c));

        assert_eqs! {
            d => "d" // 转交出来的字符串是"d"
            iter.head() => 3 // 此时头索引不变
            iter.is_began() => true // 此时已开始迭代
            iter.is_ended() => true // 此时已经结束
            iter.is_buffer_empty() => true // 此时缓冲区为空
            iter.len_buffer() => 0 // 此时缓冲区长度清零
            iter.buffer_head() => 4 // 此时「缓冲区索引」增加到`4`（为空之后比「头索引」大）
        }
    }
}
