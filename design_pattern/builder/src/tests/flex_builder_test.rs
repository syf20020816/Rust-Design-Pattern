use crate::lib::{Computer};
use crate::lib::flex_builder::{Phone};

fn main() {
    let mut builder = Phone::BUILDER;
    let phone = builder
        .screen("MI screen")
        .battery("MI battery")
        .build();
    dbg!(phone);
}
