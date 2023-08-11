//!# 命令模式
//!
//! 将一个请求封装为一个对象，使发出请求的责任和执行请求的责任分割开。这样两者之间通过命令对象进行沟通，这样方便将命令对象进行存储、传递、调用、增加与管理。
//!
//! ## 结构
//!
//! 1. 抽象命令类(Command)角色:定义命令的接口，声明执行的方法。
//! 2. 具体命令(Concrete Command)角色:具体的命令，实现命令接口;通常会持有接收者，并调用接收者的功能来完成命令要执行的操作。
//! 3. 实现者/接收者(Receiver)角色:接收者，真正执行命令的对象。任何类都可能成为一个接收者，只要它能够实现命令要求实现的相应功能。
//! 4. 调用者/请求者（Invoker)角色:要求命令对象执行请求，通常会持有命令对象，可以持有很多的命令对象。这个是客户端真正触发命令并要求命令执行相应操作的地方，也就是说相当于使用命令对象的入口。
//!
//! ## 优点
//!
//! 1. 降低系统的耦合度。命令模式能将调用操作的对象与实现该操作的对象解耦。
//! 2. 增加或删除命令非常方便。采用命令模式增加与删除命令不会影响其他类，它满足"开闭原则”，对扩展比较灵活。
//! 3. 可以实现宏命令。命令模式可以与组合模式结合，将多个命令装配成一个组合命令，即宏命令。
//! 4. 方便实现 Undo和Redo操作。命令模式可以与后面介绍的备忘录模式结合，实现命令的撤销与恢复。
//!
//! ## 缺点
//!
//! 1. 使用命令模式可能会导致某些系统有过多的具体命令类。
//! 2. 系统结构更加复杂。
//!
//! ## 使用场景
//!
//! 1. 系统需要将请求调用者和请求接收者解耦，使得调用者和接收者不直接交互。
//! 2. 系统需要在不同的时间指定请求、将请求排队和执行请求。
//! 3. 系统需要支持命令的撤销(Undo)操作和恢复(Redo)操作。
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/11
//! @version:0.0.1
//! @description:
//! ```

/// 抽象命令
pub trait Order {
    fn new()->Self where Self:Sized;
    fn run(&self) -> ();
}

/// 实现者
pub struct Worker;

impl Worker {
    fn new()->Self{
        Worker
    }
    fn sell(&self) -> () {
        println!("{}", "buy")
    }
    fn buy(&self) -> () {
        println!("{}", "sell")
    }
}

/// 具体命令
pub struct BuyOrder {
    worker: Worker,
}

impl Order for BuyOrder {
    fn new() -> Self where Self: Sized {
        BuyOrder{
            worker: Worker::new()
        }
    }

    fn run(&self) -> () {
        self.worker.buy()
    }
}

///具体命令
pub struct SellOrder {
    worker: Worker,
}

impl Order for SellOrder {
    fn new() -> Self where Self: Sized {
        SellOrder{
            worker: Worker::new()
        }
    }

    fn run(&self) -> () {
        self.worker.sell()
    }
}

/// 调用者
pub struct Invoker {
    orders: Vec<Box<dyn Order>>,
}

impl Invoker {
    pub fn new() -> Self {
        Invoker {
            orders: vec![]
        }
    }
    pub fn add(&mut self, order: Box<dyn Order>) -> () {
        self.orders.push(order);
    }
    pub fn run(&mut self) -> () {
        for order in &self.orders {
            order.run();
        }
        //执行结束后清空命令
        self.orders.clear();
    }
}
