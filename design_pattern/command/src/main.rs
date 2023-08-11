mod lib;

use crate::lib::{Order, Invoker, BuyOrder, SellOrder};

fn main() {
    let mut invoker = Invoker::new();
    //添加指令集
    invoker.add(Box::new(BuyOrder::new()));
    invoker.add(Box::new(SellOrder::new()));
    invoker.add(Box::new(SellOrder::new()));
    invoker.run();
}
