//! # 工厂方法模式
//!
//! 工厂方法模式完全遵守开闭原则
//!
//! 定义一个用于创建对象的接口，让子类决定实例化哪个产品类对象，工厂方法使一个产品类的实例化延迟到其工厂的子类
//! ## 在工厂方法中包含：
//!
//! 1. 抽象工厂：提供了创建产品的接口，调用者通过它访问具体工厂的工厂方法来创建产品
//! 2. 具体工厂：主要是实现抽象工厂中的抽象方法，完成具体产品的创建
//! 3. 抽象产品：定义了产品的规范，描述了产品的主要特性和功。
//! 4. 具体产品：实现了抽象产品角色所定义的接口，由具体工厂来创建，它同具体工厂之间一一对应
//!
//! ## 工厂方法的优点
//!
//! 用户只需要知道具体工厂的名称就可得到所要的产品，无须知道产品的具体创建过程
//! 在系统增加新的产品时只需要添加具体产品类和对应的具体工厂类，无须对原工厂进行任何修改，满足开闭原则
//!
//! ## 工厂方法的缺点
//!
//! 每增加一个产品就要增加一个具体产品类和一个对应的具体工厂类，这增加了系统的复杂度
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/23
//! @version:0.0.1
//! @description:
//! ```

use crate::lib::{Product, Product1, Product2};

/// # 抽象工厂
/// 抽象产品 see : lib::mod.rs
/// 具体产品 see : lib::mod.rs
pub trait AbstractFactory {
    fn new() -> Box<dyn Product>;
}

/// # 具体工厂1
pub struct Product1Factory;

impl AbstractFactory for Product1Factory {
    fn new() -> Box<dyn Product> {
        Box::new(Product1::new("pro1 for factory method"))
    }
}

/// # 具体工厂2
pub struct Product2Factory;

impl AbstractFactory for Product2Factory {
    fn new() -> Box<dyn Product> {
        Box::new(Product2::new("pro2 for factory method"))
    }
}