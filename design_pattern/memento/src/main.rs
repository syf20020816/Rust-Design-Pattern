mod lib;

use crate::lib::{Memento, CareTaker, Originator};

fn main() {
    let mut caretaker = CareTaker::new(Memento::new());
    caretaker.store(Originator::new(1, "version1"));
    dbg!(&caretaker);
    let res = caretaker.recover(0);
    dbg!(res);
}
