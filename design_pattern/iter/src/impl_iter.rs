//! # 实现迭代器
//! rust除了自己写迭代器还可以直接实现标准库提供的迭代器trait
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/12
//! @version:0.0.1
//! @description:
//! ```

use std::borrow::BorrowMut;

#[derive(Debug)]
pub struct Node {
    data: u32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(data: u32) -> Self {
        Node { data, next: None }
    }
    /// 添加后一个节点
    pub fn add(&mut self, next: Node) {
        self.next = Some(Box::new(next))
    }
    /// 获取节点
    pub fn get(&self) -> &u32 {
        &self.data
    }
}

/// 类似链表
#[derive(Debug)]
pub struct NodeList {
    next: Option<Box<Node>>,
}

impl NodeList {
    pub fn new() -> Self {
        NodeList {
            next: None
        }
    }
    pub fn add(&mut self, data:u32) -> () {
        self.next = Some(Box::new(Node{
            data,
            next: self.next.take()
        }))
    }
    pub fn iter(&mut self)->NodeIter{
        NodeIter{
            next:self.next.as_deref()
        }
    }
}

pub struct NodeIter<'a>{
    next:Option<&'a Node>
}
impl<'a> Iterator for NodeIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node|{
            self.next = node.next.as_deref();
            node.data
        })
    }
}