//! # 桥接模式
//!
//! 将抽象与实现分离，使它们可以独立变化。它是用组合关系代替继承关系来实现，从而降低了抽象和实现这两个可变维度的耦合度。
//!
//! ## 桥接模式的结构
//!
//! 1. 抽象化(Abstraction) 角色：定义抽象类,并包含一个对实现化对象的引用。
//! 2. 扩展抽象化(Refined Abstraction) 角色： 是抽象化角色的子类,实现父类中的业务方法，并通过组合关系调用实现化角色中的业务方法。
//! 3. 实现化(Implementor) 角色：定义实现化角色的接口，供扩展抽象化角色调用。
//! 4. 具体实现化(Concrete Implementor) 角色：给出实现化角色接口的具体实现。
//!
//! ## 桥接的优点
//!
//! 1. 桥接模式提高了系统的可扩充性,在两个变化维度中任意扩展一个维度, 都不需要修改原有系统。
//! 2. 实现细节对客户透明
//!
//! ## 桥接模式的适用场景
//!
//! 1. 当一个类存在两个独立变化的维度,且这两个维度都需要进行扩展时。
//! 2. 当一个系统不希望使用继承或因为多层次继承导致系统类的个数急剧增加时。
//! 3. 当一个系统需要在构件的抽象化角色和具体化角色之间增加更多的灵活性时。避免在两个层次之间建立静态的继承联系,通过桥接模式可以使它们在抽象层建立一个关联关系。
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/9
//! @version:0.0.1
//! @description:
//! ```

/// 实现化角色
pub trait VideoPlayer {
    fn new() -> Self where Self: Sized;
    fn decode(&self, file: &str) -> ();
}

/// 具体实现化角色
pub struct MP4;

impl VideoPlayer for MP4 {
    fn new() -> Self where Self: Sized, {
        MP4
    }

    fn decode(&self, file: &str) -> () {
        println!("{} - mp4 decode", file);
    }
}

/// 具体实现化角色
pub struct Avi;

impl VideoPlayer for Avi {
    fn new() -> Self {
        Avi
    }

    fn decode(&self, file: &str) -> () {
        println!("{} - avi decode", file);
    }
}

/// 抽象化角色
/// 存在两个独立变化的维度
pub trait System {
    fn new(player: Box<dyn VideoPlayer>) -> Self;
    fn play(&self, file: &str) -> ();
}

/// 扩展的抽象化角色
pub struct Linux {
    player: Box<dyn VideoPlayer>,
}

impl System for Linux {
    fn new(player: Box<dyn VideoPlayer>) -> Self {
        Linux {
            player
        }
    }

    fn play(&self, file: &str) -> () {
        self.player.decode(file);
    }
}

pub struct Mac {
    player: Box<dyn VideoPlayer>,
}

impl System for Mac {
    fn new(player: Box<dyn VideoPlayer>) -> Self {
        Mac {
            player
        }
    }

    fn play(&self, file: &str) -> () {
        self.player.decode(file);
    }
}