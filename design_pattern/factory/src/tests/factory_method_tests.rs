use crate::lib::{AbstractFactory, Product1Factory, Product2Factory};

fn main() {
    let pro1 = Product1Factory::new();
    pro1.show();
    let pro2 = Product2Factory::new();
    pro2.show();
}
