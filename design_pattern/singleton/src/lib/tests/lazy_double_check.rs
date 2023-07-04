use crate::lib::lazy::double_check_impl::{Lazy, INSTANCE};
use std::thread::spawn;

fn main() {
    let mut a: Lazy = Lazy::new();
    println!("{:?}", Lazy::get_instance());
    spawn(move || {
        let binding = Lazy::get_instance().lock().unwrap();
        a = *binding;
        println!("{:?}", a);
    }).join();

    unsafe {
        println!("{:?}", INSTANCE);
    }
}
