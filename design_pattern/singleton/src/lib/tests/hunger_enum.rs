use crate::lib::hunger::enum_impl::{Hunger};

fn main() {
    println!("{:?}", Hunger::INSTANCE);
    println!("{:?}", Hunger::INNER);
}