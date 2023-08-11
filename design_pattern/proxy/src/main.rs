mod lib;

use crate::lib::{DataProxy, RealData, Ref};

fn main() {
    let mut instance = DataProxy::new("hello world");
    let real = instance.get().unwrap();
    real.show();
}
