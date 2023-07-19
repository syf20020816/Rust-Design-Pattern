use crate::lib::{ProductEnum, SimpleFactory};

fn main() {
    let pro1 = SimpleFactory::new(ProductEnum::Product1, "pro1");
    pro1.show();
}