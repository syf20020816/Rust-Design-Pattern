use crate::lib::hunger::const_impl::Hunger;

fn main() {
    let hunger = Hunger::INSTANCE;
    println!("{:?}",hunger);
}