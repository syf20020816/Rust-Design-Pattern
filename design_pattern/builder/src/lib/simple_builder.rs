//! # 简单构造者
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/8
//! @version:0.0.1
//! @description:
//! ```
use super::Computer;

/// 抽象构造者
pub trait Builder {
    fn new() -> Box<dyn Builder> where Self: Sized;
    fn build_screen(&mut self) -> ();
    fn build_battery(&mut self) -> ();
    fn build_mouse(&mut self) -> ();
    fn back_computer(&self)->Computer;
}

/// 具体构造者
#[derive(Debug)]
pub struct IOSBuilder {
    obj: Computer,
}

impl Builder for IOSBuilder {
    fn new() -> Box<dyn Builder> where Self: Sized, {
        Box::new(IOSBuilder {
            obj: Computer::new()
        })
    }

    fn build_screen(&mut self) -> () {
        self.obj.set_screen("ios screen");
    }

    fn build_battery(&mut self) -> () {
        self.obj.set_battery("ios battery")
    }

    fn build_mouse(&mut self) -> () {
        self.obj.set_mouse("ios mouse")
    }

    fn back_computer(&self) -> Computer {
        Computer{
            screen: String::from(&self.obj.screen),
            battery: String::from(&self.obj.battery),
            mouse: String::from(&self.obj.mouse)
        }
    }
}

/// 指挥者
pub struct Director {
    builder: Box<dyn Builder>,
}

impl Director {
    pub fn new(builder: Box<dyn Builder>) -> Director {
        Director {
            builder
        }
    }
    pub fn construct(&mut self) -> Computer {
        self.builder.build_screen();
        self.builder.build_battery();
        self.builder.build_mouse();
        self.builder.back_computer()
    }
}
