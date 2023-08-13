//! # 发布订阅模式（观察者模式）
//!
//! 又被称为发布-订阅(Publish/Subscribe)模式，它定义了一种一对多的依赖关系，让多个观察者对象同时监听某一个主题对象。这个主题对象在状态变化时，会通知所有的观察者对象，使他们能够自动更新自己。
//!
//! ## 结构
//!
//! 1. subject:抽象主题（抽象被观察者），抽象主题角色把所有观察者对象保存在一个集合里，每个主题都可以有任意数量的观察者，抽象主题提供一个接口，可以增加和删除观察者对象。
//! 2. concreteSubscribe:具体主题（具体被观察者)，该角色将有关状态存入具体观察者对象，在具体主题的内部状态发生改变时，给所有注册过的观察者发送通知。
//! 3. Observer:抽象观察者，是观察者的抽象类，它定义了一个更新接口，使得在得到主题更改通知时更新自己。
//! 4. ConcreteObserver:具体观察者，实现抽象观察者定义的更新接口，以便在得到主题更改通知时更新自身的状态。
//!
//! ## 优点
//!
//! 1. 降低了目标与观察者之间的耦合关系，两者之间是抽象耦合关系。
//! 2. 被观察者发送通知，所有注册的观察者都会收到信息【可以实现广播机制】
//!
//! ## 缺点
//!
//! 1. 如果观察者非常多的话，那么所有的观察者收到被观察者发送的通知会耗时
//! 2. 如果被观察者有循环依赖的话，那么被观察者发送通知会使观察者循环调用，会导致系统崩溃
//!
//! ## 使用场景
//!
//! 1. 对象间存在一对多关系，一个对象的状态发生改变会影响其他对象。
//! 2. 当一个抽象模型有两个方面，其中一个方面依赖于另一方面时。
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/13
//! @version:0.0.1
//! @description:
//! ```

/// 抽象观察者
pub trait Observer {
    fn update(&self, msg: &str) -> ();
}

/// 具体观察者
pub struct ChildrenSystem {
    name: String,
}

impl ChildrenSystem {
    pub fn new(name: &str) -> Self {
        ChildrenSystem {
            name: String::from(name)
        }
    }
}

impl Observer for ChildrenSystem {
    fn update(&self, msg: &str) -> () {
        println!("{} - msg from sys : {}", &self.name, msg);
    }
}

/// 抽象主题
pub trait Subject {
    fn add(&mut self, observer: Box<dyn Observer>) -> ();
    fn remove(&mut self, index: usize) -> ();
    fn notify(&self, msg: &str) -> ();
}

pub struct MainSystem {
    sys_list: Vec<Box<dyn Observer>>,
}

impl MainSystem {
    pub fn new() -> Self where Self: Sized {
        MainSystem {
            sys_list: vec![]
        }
    }
}

impl Subject for MainSystem {
    fn add(&mut self, observer: Box<dyn Observer>) -> () {
        self.sys_list.push(observer)
    }

    fn remove(&mut self, index: usize) -> () {
        self.sys_list.remove(index);
    }

    fn notify(&self, msg: &str) -> () {
        for sys in &self.sys_list {
            sys.update(msg)
        }
    }
}