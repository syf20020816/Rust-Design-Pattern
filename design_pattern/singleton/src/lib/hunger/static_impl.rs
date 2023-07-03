//! # 基于static的一种饿汉式实现
//!
//! static 关键字用于声明静态变量，它们位于静态存储区，生命周期贯穿整个程序执行过程。
//!
//! 与 const 不同，static 变量的值可以是在运行时计算的，并且可以在任何作用域内使用，甚至可以在跨线程共享。静态变量的值在第一次访问时初始化，并且以后的访问都会使用相同的值。
//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/4
//! @version:0.0.1
//! @description:
//! ```

pub static INSTANCE: Hunger = Hunger::new();

#[derive(Debug)]
pub struct Hunger {
    data: u8,
}

impl Hunger {
    pub const fn new() -> Self {
        Hunger {
            data: 101
        }
    }
}