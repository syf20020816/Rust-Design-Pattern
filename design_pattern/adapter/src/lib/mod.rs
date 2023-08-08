//!# 适配器模式
//!
//! 将一个类的接口转换成客户希望的另外一个接口，使得原本由于接口不兼容而不能一起工作的那些类能一起工作。
//! 适配器模式分为类适配器模式和对象适配器模式，前者类之间的耦合度比后者高，且要求程序员了解现有组件库中的相关组件的内部结构，所以应用相对较少些。
//!
//! ## 适配器模式的结构
//!
//! 1. 目标(Target) 接口：当前系统业务所期待的接口，它可以是抽象类或接口
//! 2. 适配者(Adaptee) 类：它是被访问和适配的现存组件库中的组件接口
//! 3. 适配器(Adapter) 类：它是一个转换器 ，通过继承或弓|用适配者的对象,把适配者接口转换成目标接口，让客户按目标接口的格式访问适配者
//!
//! ## 使用场景
//!
//! 1. 以前开发的系统存在满足新系统功能需求的类，但其接口同新系统的接口不一致。
//! 2. 使用第三方提供的组件，但组件接口定义和自己要求的接口定义不同。
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/9
//! @version:0.0.1
//! @description:
//! ```
/// API trait
pub trait ApiImpl{
    fn api1(&self)->();
}
/// 新API
pub struct NewApiImpl;

impl ApiImpl for NewApiImpl {
    fn api1(&self) -> () {
        println!("{}","new api running...")
    }
}
/// 旧API
pub struct OldApiImpl;

impl ApiImpl for OldApiImpl {
    fn api1(&self) -> () {
        println!("{}","old api running...")
    }
}
/// 适配器
/// 在适配器中借旧API
pub struct Adapter;

impl ApiImpl for Adapter {
    fn api1(&self) -> () {
        let old = OldApiImpl;
        old.api1();
    }
}

pub struct System;

impl System {
    pub fn run_api(api:impl ApiImpl)->(){
        api.api1()
    }
}