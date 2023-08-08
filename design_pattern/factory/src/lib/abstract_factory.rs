//! # 抽象工厂模式
//!
//! 抽象工厂模式是工厂方法模式的升级版本，工厂方法模式只生产一个等级的产品，而抽象工厂模式可生产多个等级的产品。
//!
//! ## 产品级别
//!
//! 同类型产品，比如：苹果手机，华为手机，小米手机，属于一个等级
//!
//! ## 产品族
//!
//! 同厂家生产的产品，如：小米手机，小米手环，小米电脑属于一个产品族
//!
//! ## 抽象工厂优点
//!
//! 当一个产品族中的多个对象被设计成一起工作时，它能保证客户端始终只使用同一个产品族中的对象
//!
//! ## 抽象工厂缺点
//!
//! 当产品族中需要增加一个新的产品时，所有的工厂类都需要进行修改
//!
//! ## 使用场景
//!
//! 1. 当需要创建的对象是一系列相互关联或相互依赖的产品族时，如电器工厂中的电视机、洗衣机等
//! 2. 系统中有多个产品族，但每次只使用其中的某一族产品。如有人只喜欢穿某一个品牌的衣服和鞋
//! 3. 系统中提供了产品的类库，且所有产品的接口相同，客户端不依赖产品实例的创建细节和内部结构。
//!    如:输入法换皮肤，一整套一起换。生成不同操作系统的程序
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/23
//! @version:0.0.1
//! @description:
//! ```

use crate::lib::{Product1, Product};

pub trait AbstractProductFactory {
    fn create_pro1(name:&str) -> Box<dyn Product>;
}

pub struct Product1Factory1;

impl AbstractProductFactory for Product1Factory1 {
    fn create_pro1(name:&str) -> Box<dyn Product> {
        Box::new(Product1::new(name))
    }
}