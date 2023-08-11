mod lib;

use crate::lib::{Handler, Manager, GroupLeader};

fn main() {
    /// 向上级提交
    let leader = GroupLeader::new();
    leader.handle(3);
    leader.handle(5);
    leader.handle(1);
}
