//! # 懒汉式static实现
//!
//! 其中使用unsafe对static进行修改
//!
//! 仅当调用Lazy::get_instance()才会生成实例
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/4
//! @version:0.0.1
//! @description:
//! ```


pub static mut INSTANCE: Option<Lazy> = None;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Lazy {
    data: u8,
}

impl Lazy {
    pub fn get_instance() -> &'static Lazy {
        unsafe {
            match INSTANCE {
                None => {
                    let _ = INSTANCE.replace(Lazy { data: 110 });
                    Lazy::get_instance()
                }
                Some(ref instance) => instance
            }
        }
    }
}