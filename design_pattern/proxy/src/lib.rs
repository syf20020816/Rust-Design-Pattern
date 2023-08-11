//! # 代理模式
//!
//! 由于某些原因需要给某对象提供一个代理以控制对该对象的访问。这时，访问对象不适合或者不能直接引用目标对象，代理对象作为访问对象和目标对象之间的中介。
//!
//! ## 代理模式结构
//!
//! 代理（Proxy)模式分为三种角色:
//!
//! 1. 抽象主题(subject)类:通过接口或抽象类声明真实主题和代理对象实现的业务方法。
//! 2. 真实主题(Real subject)类: 实现了抽象主题中的具体业务，是代理对象所代表的真实对象，是最终要引用的对象。
//! 3. 代理(Proxy)类︰提供了与真实主题相同的接口，其内部含有对真实主题的引用，它可以访问、控制或扩展真实主题的功能。
//!
//! ## 代理的优点
//!
//! 1. 代理模式在客户端与目标对象之间起到一个中介作用和保护目标对象的作用
//! 2. 代理对象可以扩展目标对象的功能
//! 3. 代理模式能将客户端与目标对象分离，在一定程度上降低了系统的耦合度
//!
//! ## 代理的缺点
//!
//! 增加了系统的复杂度
//!
//!  ## 使用场景
//!
//!  ### 远程(Remote) 代理
//!
//! 本地服务通过网络请求远程服务。为了实现本地到远程的通信，我们需要实现网络通信，处理其中可能的异常。为良好的代码设计和可维护性，我们将网络通信部分隐藏起来，只暴露给本地服务一个接口， 通过该接口即可访问远程服务提供的功能，而不必过多关心通信部分的细节。
//!
//! ### 防火墙(Firewall) 代理
//!
//! 当你将浏览器配置成使用代理功能时，防火墙就将你的浏览器的请求转给互联网;当互联网返回响应时，代理服务器再把它转给你的浏览器。
//!
//! ### 保护(Protect or Access) 代理
//!
//! 控制对一个对象的访问，如果需要，可以给不同的用户提供不同级别的使用权限。
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/11
//! @version:0.0.1
//! @description:
//! ```

use std::fmt::Debug;

pub trait Ref {
    fn show(&self) -> ();
}

/// 真实值
#[derive(Debug)]
pub struct RealData {
    data: String,
}

impl RealData {
    pub fn new(data: &str) -> Self {
        RealData {
            data: String::from(data)
        }
    }
    pub fn show(&self) -> () {
        println!("{:?}", &self)
    }
}

/// 值的代理对象
pub struct DataProxy {
    name: String,
    data: Option<RealData>,
}

impl DataProxy {
    pub fn new(name: &str) -> DataProxy {
        DataProxy {
            name: String::from(name),
            data: None,
        }
    }
    /// 获取真实值的引用
    pub fn get(&mut self) -> Option<&RealData> {
        if self.data.is_none() {
            self.data = Some(RealData::new(&self.name));
        }
        self.data.as_ref()
    }
}

impl Ref for DataProxy {
    fn show(&self) -> () {
        match self.data {
            None => (),
            Some(ref res) => res.show()
        }
    }
}