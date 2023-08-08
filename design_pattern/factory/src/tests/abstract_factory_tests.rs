use crate::lib::{Product1Factory, Product1Factory1,AbstractProductFactory};

fn main() {
    let factory = Product1Factory1::create_pro1("abs");
    factory.show();
}
