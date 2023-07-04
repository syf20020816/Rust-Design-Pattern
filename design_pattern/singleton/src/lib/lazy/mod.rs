//! # 懒汉式
//! 类加载不会导致该单实例对象被创建
//!
//! 而是首次使用该对象时才会创建
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/4
//! @version:0.0.1
//! @description:
//! ```

///普通的static实现
pub mod static_impl;
///双重检查锁
pub mod double_check_impl;
