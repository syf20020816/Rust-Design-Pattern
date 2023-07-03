//!在 Hunger 结构体中，使用了 pub const INSTANCE: Self = Hunger::new(); 这行代码来定义了一个常量 INSTANCE
//!
//! 它是 Hunger 结构体的单例实例。由于该常量是用 const 关键字声明的，并且在编译时就会被初始化，所以在加载时就会创建好 Hunger 的实例。
//!
//! 通过调用 Hunger::new() 方法，在编译时会将 data 字段设置为 100，并返回一个 Hunger 结构体的实例。
//!
//! 因此，无论何时访问 Hunger::INSTANCE，都会得到相同的 Hunger 实例，这符合饿汉式单例模式的特点。
//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/4
//! @version:0.0.1
//! @description: 思路参考自rust核心库`core/src/num/int_macros::int_impl!`
//! ```

#[derive(Debug)]
pub struct Hunger {
    data: u8,
}

impl Hunger {
    pub const INSTANCE: Self = Hunger::new();
    pub const fn new() -> Self {
        Hunger {
            data: 100
        }
    }
}

