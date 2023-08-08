//! # 简单工厂模式
//! 并非一种设计模式而是一种编程习惯
//!
//! 简单工厂包含如下角色：
//!
//! 1. 抽象产品︰定义了产品的规范，描述了产品的主要特性和功能。
//! 2. 具体产品︰实现或者继承抽象产品的子类
//! 3. 具体工厂︰提供了创建产品的方法，调用者通过该方法来创建产品。
//!
//! ## 简单工厂的优点
//!
//! 封装了创建对象的过程，可以通过参数直接获取对象。把对象的创建和业务逻辑层分开，这样以后就避免了修改客户代码，
//! 如果要实现新产品直接修改工厂类，而不需要在原代码中修改，这样就降低了客户代码修改的可能性，更加容易扩展
//!
//! ### 简单工厂的缺点
//!
//! 增加新产品时还是需要修改工厂类的代码，依然违背了"开闭原则"
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/18
//! @version:0.0.1
//! @description:
//! ```

use crate::lib::{Product, Product1, Product2, ProductEnum};
use crate::product_impl;


/// 具体工厂
pub struct SimpleFactory;

impl SimpleFactory {
    pub fn new(name: ProductEnum, pro_name: &str) -> Box<dyn Product> {
        match name {
            ProductEnum::Product1 => Box::new(Product1::new(pro_name)),
            ProductEnum::Product2 => Box::new(Product2::new(pro_name))
        }
    }
}