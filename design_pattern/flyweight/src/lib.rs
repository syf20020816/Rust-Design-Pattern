//! # 享元模式
//!
//! 运用共享技术来有效地支持大量细粒度对象的复用。它通过共享已经存在的对象来大幅度减少需要创建的对象数量、避免大量相似对象的开销，从而提高系统资源的利用率。
//!
//! ## 享元模式的状态
//!
//! 1. 内部状态：不会随着环境的改变而改变的可共享部分
//! 2. 外部状态：随着环境改变而改变的不可以共享的部分
//!
//! > 享元模式的实现要领就是区分应用中的这两种状态，并将外部状态外部化
//!
//! ## 享元模式的结构
//!
//! 1. 抽象享元角色(Flyweight) :通常是一个接口或抽象类,在抽象享元类中声明了具体享元类公共的方法，这些方法可以向外界提供享元对象的内部数据(内部状态)，同时也可以通过这些方法来设置外部数据(外部状态)。
//! 2. 具体享元(Concrete Flyweight) 角色:它实现了抽象享元类，称为享元对象;在具体享元类中为内部状态提供了存储空间。通常我们可以结合单例模式来设计具体享元类,为每一个具体享元类提供唯一的享元对象。
//! 3. 非享元(Unsharable Flyweight)角色:并不是所有的抽象享元类的子类都需要被共享，不能被共享的子类可设计为非共享具体享元类;当需要一个非共享具体享元类的对象时可以直接通过实例化创建。
//! 4. 享元工厂(Flyweight Factory) 角色:负责创建和管理享元角色。当客户对象请求一个享元对象时，享元工厂检查系统中是否存在符合要求的享元对象，如果存在则提供给客户:如果不存在的话，则创建一 个新的享元对象。
//!
//! ## 优点
//!
//! 1. 极大减少内存中相似或相同对象数量，节约系统资源，提供系统性能
//! 2. 享元模式中的外部状态相对独立，且不影响内部状态
//!
//! ## 缺点
//!
//! 为了使对象可以共享，需要将享元对象的部分状态外部化，分离内部状态和外部状态，使程序逻辑复杂
//!
//! ## 使用场合
//!
//! 1. 一个系统有大量相同或者相似的对象，造成内存的大量耗费。
//! 2. 对象的大部分状态都可以外部化，可以将这些外部状态传入对象中。
//! 3. 在使用享元模式时需要维护一个存储享元对象的享元池，而这需要耗费一 定的系统资源，因此，应当在需要多次重复使用享元对象时才值得使用享元模式。
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/11
//! @version:0.0.1
//! @description:
//! ```

use std::collections::HashMap;
use std::fmt::Debug;

/// 形状（抽象享元）
pub trait Shape {
    fn new() -> Self where Self: Sized;
    /// 外部化状态，可修改，不可共享
    fn add_style(&mut self, k: &str, v: &str) -> ();
    /// 内部化状态，不可修改，可共享
    fn shape(&self) -> &str;
    /// 通过内部状态，获取一个新Shape,切记外部化状态不可共享！
    fn from(&self) -> Box<dyn Shape>;
    fn show(&self) -> ();
}

/// 具体享元
#[derive(Debug)]
pub struct Rectangle {
    shape: String,
    styles: HashMap<String, String>,
}

impl Shape for Rectangle {
    fn new() -> Self where Self: Sized {
        Rectangle {
            shape: String::from("rect"),
            styles: HashMap::new(),
        }
    }

    fn add_style(&mut self, k: &str, v: &str) -> () {
        self.styles.insert(String::from(k), String::from(v));
    }

    fn shape(&self) -> &str {
        &self.shape
    }

    fn from(&self) -> Box<dyn Shape> {
        Box::new(Rectangle {
            shape: String::from(self.shape()),
            styles: HashMap::new(),
        })
    }

    fn show(&self) -> () {
        println!("{:?}", &self);
    }
}

/// 具体享元
#[derive(Debug)]
pub struct Circle {
    shape: String,
    styles: HashMap<String, String>,
}

impl Shape for Circle {
    fn new() -> Self where Self: Sized {
        Circle {
            shape: String::from("circle"),
            styles: HashMap::new(),
        }
    }

    fn add_style(&mut self, k: &str, v: &str) -> () {
        self.styles.insert(String::from(k), String::from(v));
    }
    fn shape(&self) -> &str {
        &self.shape
    }

    fn from(&self) -> Box<dyn Shape> {
        Box::new(Circle {
            shape: String::from(self.shape()),
            styles: HashMap::new(),
        })
    }

    fn show(&self) -> () {
        println!("{:?}", &self);
    }
}

/// 享元工厂
pub struct ShapeFactory {
    shapes: HashMap<String, Box<dyn Shape>>,
}

impl ShapeFactory {
    pub fn new() -> Self {
        let mut shapes: HashMap<String, Box<dyn Shape>> = HashMap::new();
        shapes.insert(String::from("rect"), Box::new(Rectangle::new()));
        shapes.insert(String::from("circle"), Box::new(Circle::new()));
        ShapeFactory {
            shapes
        }
    }
    pub fn get(&self, shape: &str) -> Box<dyn Shape> {
        let instance = self.shapes.get(shape).unwrap();
        instance.from()
    }
}
