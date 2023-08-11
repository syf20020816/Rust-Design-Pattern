//! # 链表
//! 非连续内存地址
//! ## 结构
//! ```code
//!  head
//!   ↓      ↗ → *2 → → *3 →
//!   *1 → ↗                 ↘
//!                 *4 ← ← ← ↙
//!
//!  each el structure
//!   |————————————————————————|
//!   |  data  |  next pointer | ——→ next el
//!   |————————————————————————|
//! ```
//! ## 使用场景
//!1. 实现容器：链表可以用于实现各种容器，如列表、队列和栈。链表具有动态扩展和收缩的特性，可以高效地添加、删除和访问元素。
//!2. 内存管理：链表在内存分配中有广泛的应用。例如，操作系统中的内存管理器可以使用链表来跟踪未使用的内存块。
//!3. 图和树的表示：链表可以用于表示图和树的结构。每个节点可以使用链表中的指针来连接其他节点，从而形成图或树的结构。
//!4. 文件系统：文件系统中的目录结构可以使用链表来表示。每个目录可以使用链表中的指针来连接子目录或文件。
//!5. 缓存实现：链表可以用于实现缓存结构。当需要缓存一部分数据时，可以使用链表来维护缓存中数据的顺序，以便快速访问最近使用的数据。
//!6. 字符串操作：链表可以用于实现字符串操作，如字符串的反转、拼接和分割等。每个字符可以用链表中的节点来表示。
//!7. 数据结构的实现：链表可以作为其他数据结构的基础实现，如哈希表、图和树等。通过使用链表的节点和指针，可以构建出更复杂的数据结构。
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/10
//! @version:0.0.1
//! @description:
//! ```

/// # 单个节点
/// 1. 数据域
/// 2. 指针域：指向下一个节点
#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    /// # 初始化空节点
    /// 传入数据域
    /// 初始化指针域为None，表示没有下个节点
    pub fn new(data: T) -> Self {
        Node {
            data,
            next: None,
        }
    }
    /// 设置数据域
    pub fn set(&mut self, data: T) -> () {
        self.data = data;
    }
    /// 返回数据域
    pub fn get(&self) -> Option<&T> {
        Some(&self.data)
    }
    /// 设置后续节点
    pub fn set_next(&mut self, el: Node<T>) -> () {
        self.next = Some(Box::new(el));
    }
}
#[derive(Debug)]
pub struct LinkedList<T> {
    len: usize,
    node: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    /// 初始化空链表
    pub fn new() -> Self {
        LinkedList {
            len: 0,
            node: None,
        }
    }
    /// 获取链表长度
    pub fn len(&self) -> usize { self.len }
    /// 判断是否为空链表
    pub fn is_empty(&self) -> bool { 0_usize.eq(&self.len()) }
    /// 添加元素
    pub fn push(&mut self, el: T) -> () {
        let node = Box::new(Node{
            data: el,
            next: self.node.take()
        });
        self.node = Some(node);
        self.len += 1;
    }
    /// # 取出头节点
    /// 1. 获取头节点
    /// 2. 后续节点前移
    pub fn pop(&mut self)->Option<T>{
        self.node.take().map(|node|{
            self.node = node.next;
            self.len-=1;
            node.data
        })
    }
    /// 获取头节点的引用
    pub fn peek(&self)->Option<&Box<Node<T>>>{
        self.node.as_ref()
    }
    /// 获取头节点可变引用
    pub fn peek_as_mut(&mut self)->Option<&mut Box<Node<T>>>{
        self.node.as_mut()
    }
    pub fn into_iter(self)->IntoIter<T>{
        IntoIter(self)
    }
    pub fn iter(&self)->Iter<T>{
        Iter{
            next: self.node.as_deref()
        }
    }
    pub fn iter_mut(&mut self)->IterMut<T>{
        IterMut{
            next: self.node.as_deref_mut()
        }
    }
}

/// 清空
impl<T> Drop for LinkedList<T>{
    fn drop(&mut self) {
        self.node.take().map(|mut node|{
           node.next.take()
        });
    }
}

/// 转为迭代器
pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
/// 转为不可变引用迭代器(Node)
/// 通过迭代器获取Node的数据域
/// 获取到之后将下一个前移（None也一样）
pub struct Iter<'a, T:'a>{
    next:Option<&'a Node<T>>
}

impl <'a,T> Iterator for Iter<'a ,T>{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node|{
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

/// 转为可变引用迭代器（Node）
pub struct IterMut<'a,T:'a>{
    next:Option<&'a mut Node<T>>
}

impl<'a ,T> Iterator for IterMut<'a ,T>{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node|{
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}
