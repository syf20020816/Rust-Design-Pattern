//! # 饿汉式
//! 加载就会导致该单实例对象被创建
//!
//! >note:若该实例长时间不使用则会长时间占据内存空间，导致内存浪费
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/4
//! @version:0.0.1
//! @description:
//! ```

/// const 实现
pub mod const_impl;
/// static 实现
pub mod static_impl;
/// enum 实现
/// >note:这种写法是错误的！
pub mod enum_impl;