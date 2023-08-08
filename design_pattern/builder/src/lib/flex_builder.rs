//! # 以一种更灵活的方式让用户决定构建
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/9
//! @version:0.0.1
//! @description:
//! ```


#[derive(Debug)]
pub struct Phone {
    screen: String,
    battery: String,
}

impl Phone {
    pub const BUILDER: Builder = Builder::new();
    fn new(screen: &str, battery: &str) -> Phone {
        Phone { screen: String::from(screen), battery: String::from(battery) }
    }
}

pub struct Builder {
    screen: String,
    battery: String,
}

impl Builder {
    pub const fn new() -> Builder {
        Builder {
            screen: String::new(),
            battery: String::new(),
        }
    }
    pub fn screen(&mut self, name: &str) -> &mut Self {
        self.screen = String::from(name);
        self
    }
    pub fn battery(&mut self, name: &str) -> &mut Self {
        self.battery = String::from(name);
        self
    }
    pub fn build(&mut self) -> Phone {
        Phone::new(&self.screen, &self.battery)
    }
}