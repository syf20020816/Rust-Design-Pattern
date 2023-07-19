//! # 工厂模式
//! 在获取对象时我们常使用new，这样对对象的耦合十分严重，如果我们使用工厂来生产对象，
//! 我们就只和工厂打交道就可以了，彻底和对象解耦，如果要更换对象，
//! 直接在工厂里更换该对象即可，达到了与对象解耦的目的
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/18
//! @version:0.0.1
//! @description:
//! ```

///简单工厂
mod simple_factory;
///工厂方法
mod factory_method;
///抽象工厂
mod abstract_factory;


pub use simple_factory::*;