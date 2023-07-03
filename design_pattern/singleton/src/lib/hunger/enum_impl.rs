//! # 👎基于Enum的一种实现
//!
//! 虽然使用了关联常量 INSTANCE 来表示 Hunger 枚举的一个实例，但是由于枚举类型的特性，Hunger::Data(16) 只是一个可能的取值而不是真正的单例实例。
//!
//! 每次访问 Hunger::INSTANCE 时，都会创建一个新的 Hunger::Data(16) 实例。所以，第二种代码并不符合饿汉式单例模式的定义。
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/4
//! @version:0.0.1
//! @description:
//! ```

#[derive(Debug)]
pub enum Hunger {
    Data(u8)
}

impl Hunger {
    pub const INSTANCE: Hunger = Hunger::Data(16);
    pub const INNER: u8 = Hunger::Data(32).match_instance();

    const fn match_instance(self) -> u8 {
        match self { Hunger::Data(data) => data }
    }
}