//! # 策略模式
//!
//! 该模式定义了一系列算法，并将每个算法封装起来，使它们可以相互替换，且算法的变化不会影响使用算法的客户。策略模式属于对象行为模式，它通过对算法进行封装，把使用算法的责任和算法的实现分割开来，并委派给不同的对象对这些算法进行管理。
//!
//! ## 结构
//!
//! 1. 抽象策略(strategy)类:这是一个抽象角色，通常由一个接口或抽象类实现。此角色给出所有的具体策略类所需的接口。
//! 2. 具体策略(Concrete strategy)类:实现了抽象策略定义的接口，提供具体的算法实现或行为。
//! 3. 环境(context)类:持有一个策略类的引用，最终给客户端调用。
//!
//! ## 优点
//!
//! 1. 策略类之间可以自由切换:由于策略类都实现同一个接口，所以使它们之间可以自由切换。
//! 2. 易于扩展:增加一个新的策略只需要添加一个具体的策略类即可，基本不需要改变原有的代码，符合"开闭原则"
//! 3. 避免使用多重条件选择语句(if else)，充分体现面向对象设计息想。
//!
//! ## 缺点
//!
//! 1. 客户端必须知道所有的策略类，并自行决定使用哪一个策略类。
//! 2. 策略模式将造成产生很多策略类，可以通过使用享元模式在一定程度上减少对象的数量。
//!
//! ## 使用场景
//!
//! 1. 一个系统需要动态地在几种算法中选择一种时，可将每个算法封装到策略类中。
//! 2. 一个类定义了多种行为，并且这些行为在这个类的操作中以多个条件语句的形式出现，可将每个条件分支移入它们各自的策略类中以代替这些条件语句。
//! 3. 系统中各算法彼此完全独立，且要求对客户隐藏具体算法的实现细节时。
//! 4. 系统要求使用算法的客户不应该知道其操作的数据时，可使用策略模式来隐藏与算法相关的数据结构。
//! 5. 多个类只区别在表现行为不同，可以使用策略模式，在运行时动态选择具体要执行的行为
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/13
//! @version:0.0.1
//! @description:
//! ```

/// 抽象策略
pub trait Strategy {
    fn show(&self) -> ();
}

/// 具体策略
pub struct ShowStrategy1;

impl Strategy for ShowStrategy1 {
    fn show(&self) -> () {
        println!("{}", "show strategy 1")
    }
}

pub struct ShowStrategy2;

impl Strategy for ShowStrategy2 {
    fn show(&self) -> () {
        println!("{}", "show strategy 2")
    }
}

/// 环境角色
pub struct Context<T: ?Sized + Strategy> {
    strategy: Box<T>,
}

impl<T: ?Sized + Strategy> Context<T> {
    pub fn new(strategy: Box<T>) -> Self {
        Context {
            strategy
        }
    }
    pub fn show(&self) -> () {
        &self.strategy.show();
    }
    pub fn replace(&mut self, strategy: Box<T>) -> () {
        self.strategy = strategy;
    }
}