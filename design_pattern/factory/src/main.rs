mod lib;
mod tests;

use crate::lib::{SimpleFactory, ProductEnum};

fn main() {
    let pro1 = SimpleFactory::new(ProductEnum::Product1, "pro1");
    pro1.show();
}
