//! # 外观模式
//!
//! 又名门面模式，是一种通过为多个复杂的子系统提供一个一致的接口, 而使这些子系统更加容易被访问的模式。
//! 该模式对外有一个统一接口， 外部应用程序不用关心内部子系统的具体的细节，这样会大大降低应用程序的复杂度,提高了程序的可维护性。
//! 外观(Facade) 模式是“迪米特法则"的典型应用
//!
//! ## 外观模式结构
//!
//! 1. 外观(Facade) 角色:为多个子系统对外提供一个共同的接口。
//! 2. 子系统(Sub System) 角色:实现系统的部分功能，客户可以通过外观角色访问它。
//!
//! ## 外观模式优点
//!
//! 1. 降低了子系统与客户端之间的耦合度,使得子系统的变化不会影响调用它的客户类。
//! 2. 对客户屏蔽了子系统组件,减少了客户处理的对象数目,并使得子系统使用起来更加容易。
//!
//! ## 外观模式缺点
//!
//! 不符合开闭原则，修改起来很麻烦
//!
//! ## 使用场景
//!
//! 1. 对分层结构系统构建时，使用外观模式定义子系统中每层的入口点可以简化子系统之间的依赖关系。
//! 2. 当一个复杂系统的子系统很多时,外观模式可以为系统设计一个简单的接口供外界访问。
//! 3. 当客户端与多个子系统之间存在很大的联系时，引入外观模式可将它们分离，从而提高子系统的独立性和可移植性。
//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/11
//! @version:0.0.1
//! @description:
//! ```

pub trait ElectricalEquipment {
    fn new(name: &str) -> Self where Self: Sized;
    fn on(&self) -> ();
    fn off(&self) -> ();
    fn name(&self)->&str;
}
/// 子系统
pub struct RiceCooker {
    name: String,
}

impl ElectricalEquipment for RiceCooker {
    fn new(name: &str) -> Self where Self: Sized{
        RiceCooker {
            name: String::from(name)
        }
    }

    fn on(&self) -> () {
        println!("{}", "rice_cooker on...")
    }

    fn off(&self) -> () {
        println!("{}", "rice_cooker off...")
    }

    fn name(&self) -> &str {
        &self.name
    }
}
/// 子系统
pub struct Light {
    name: String,
}

impl ElectricalEquipment for Light {
    fn new(name: &str) -> Self where Self: Sized{
        Light {
            name: String::from(name)
        }
    }

    fn on(&self) -> () {
        println!("{}", "light on...")
    }

    fn off(&self) -> () {
        println!("{}", "light off...")
    }

    fn name(&self) -> &str {
        &self.name
    }
}
/// # 外观角色
/// 用语音助手控制电器
pub struct AIHelper {
    workers: Vec<Box<dyn ElectricalEquipment>>,
}

impl AIHelper {
    pub fn new() -> Self where Self: Sized {
        AIHelper {
            workers: vec![]
        }
    }
    pub fn push(&mut self, work: Box<dyn ElectricalEquipment>) -> () {
        self.workers.push(work);
    }
    pub fn ctrl(&self,worker_name:&str,on:bool)->(){
        for worker in &self.workers {
            if worker_name.eq(worker.name()){
                if on {
                    worker.on();
                }else {
                    worker.off();
                }
            }
        }
    }
}

