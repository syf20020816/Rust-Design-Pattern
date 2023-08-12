//!# 迭代器模式
//!
//! 提供一个对象来顺序访问聚合对象中的一系列数据，而不暴露聚合对象的内部表示。
//!
//! ## 结构
//!
//! 1. 抽象聚合(Aggregate)角色:定义存储、添加、删除聚合元素以及创建迭代器对象的接口。
//! 2. 具体聚合(concreteAggregate)角色:实现抽象聚合类，返回一个具体迭代器的实例。
//! 3. 抽象迭代器(Iterator)角色:定义访问和遍历聚合元素的接口，通常包含 hasNext(),next()等方法
//! 4. 具体迭代器(ConcreteIterator)角色:实现抽象迭代器接口中所定义的方法，完成对聚合对象的遍历，记录遍历的当前位置。
//!
//! ## 优点
//!
//! 1. 它支持以不同的方式遍历一个聚合对象，在同一个聚合对象上可以定义多种遍历方式。在迭代器模式中只需要用一个不同的迭代器来替换原有迭代器即可改变遍历算法，我们也可以自己定义迭代器的子类以支持新的遍历方式。
//! 2. 迭代器简化了聚合类。由于引入了迭代器，在原有的聚合对象中不需要再自行提供数据遍历等方法，这样可以简化聚合类的设计。
//! 3. 迭代器模式中，由于引入了抽象层，增加新的聚合类和迭代器类都很方便，无须修改原有代码，满足“开闭原则”的要求。
//!
//! ## 缺点
//!
//! 增加了类的个数，这在一定程度上增加了系统的复杂性。
//!
//! ## 使用场景
//!
//! 1. 当需要为聚合对象提供多种遍历方式时。
//! 2. 当需要为遍历不同的聚合结构提供一个统一的接口时。
//! 3. 当访问一个聚合对象的内容而无须暴露其内部细节的表示时。
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/12
//! @version:0.0.1
//! @description:
//! ```

use std::ops::Deref;

/// 节点
#[derive(Clone,Debug)]
pub struct Node {
    name: String,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: String::from(name)
        }
    }
}

/// 抽象迭代器
pub trait Iter {
    fn hasNext(&self) -> bool;
    fn next(&mut self) -> &Node;
}

#[derive(Clone)]
pub struct NodeList {
    nodes: Vec<Node>,
    index: usize,
}

impl NodeList {
    fn new(nodes: Vec<Node>) -> Self {
        NodeList {
            nodes,
            index: 0,
        }
    }
}

impl Iter for NodeList {
    fn hasNext(&self) -> bool {
        let len = self.nodes.len();
        len.gt(&self.index)
    }

    fn next(&mut self) -> &Node {
        let index = self.index;
        let node = &self.nodes[index];
        self.index += 1;
        node
    }
}

/// 抽象存储器
pub trait Aggregate {
    fn add(&mut self, node: Node) -> ();
    fn remove(&mut self, index: usize) -> ();
    fn getIter(&self) -> Box<dyn Iter>;
}

/// 具体存储器
pub struct NodeAggregate {
    nodes: Vec<Node>,
}

impl NodeAggregate {
    pub fn new() -> Self {
        NodeAggregate {
            nodes: vec![]
        }
    }
}


impl Aggregate for NodeAggregate {
    fn add(&mut self, node: Node) -> () {
        self.nodes.push(node);
    }

    fn remove(&mut self, index: usize) -> () {
        self.nodes.remove(index);
    }

    fn getIter(&self) -> Box<dyn Iter> {
        let mut nodes = Vec::new();
        for node in &self.nodes {
            nodes.push(node.clone());
        }
        Box::new(NodeList::new(nodes))
    }
}