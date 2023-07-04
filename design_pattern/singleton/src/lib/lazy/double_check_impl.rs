//! # 双重检查锁实现
//!
//!在 get_instance() 方法中，我们首先通过 call_once() 函数来执行初始化，确保只有第一个调用该函数的线程会执行初始化代码。使用 Mutex 来保护 Lazy 结构体的访问。
//!
//! 然后，我们检查 INSTANCE 是否存在，如果存在，我们使用 lock() 方法来获取互斥锁的可变引用
//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/4
//! @version:0.0.1
//! @description:
//! ```

use std::ops::Deref;
use std::sync::{Arc, Mutex, Once};

pub static mut INSTANCE: Option<Arc<Mutex<Lazy>>> = None;
///创建了一个静态变量 INIT，它是 Once 类型，用于确保只有一个线程执行初始化过程。
static INIT: Once = Once::new();

#[derive(Debug, PartialEq, Copy, Clone,Eq)]
pub struct Lazy {
    data: u8,
}

impl Lazy {
    pub fn new() -> Self {
        Lazy {
            data: 0
        }
    }
    pub fn get_instance() -> &'static Arc<Mutex<Lazy>> {
        unsafe {
            match INSTANCE {
                None => {
                    INIT.call_once(|| {
                        let instance = Arc::new(Mutex::new(Lazy { data: 120 }));
                        INSTANCE = Some(instance);
                    });
                    Self::get_instance()
                }
                Some(ref instance) => {
                    instance
                }
            }
        }
    }
}