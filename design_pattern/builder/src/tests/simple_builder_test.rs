use crate::lib::{Computer};
use crate::lib::simple_builder::{Builder, IOSBuilder, Director};

fn main() {
    let mut director = Director::new(IOSBuilder::new());
    let computer = director.construct();
    dbg!(computer);
}