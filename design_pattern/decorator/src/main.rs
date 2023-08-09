mod lib;

use crate::lib::{HandGun,Gun,Grip,Parts};

fn main() {
    let mut gun = HandGun::new(Box::new(Grip::new()));
    gun.cost();
}
