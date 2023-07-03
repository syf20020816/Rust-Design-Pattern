mod lib;

use lib::hunger::Hunger;

fn main() {
    let hunger = Hunger::INSTANCE;
    println!("{:?}",hunger);
}
