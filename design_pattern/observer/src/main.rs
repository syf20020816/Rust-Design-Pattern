mod lib;

use crate::lib::{MainSystem, ChildrenSystem, Observer, Subject};

fn main() {
    let mut main_sys = MainSystem::new();
    main_sys.add(Box::new(ChildrenSystem::new("children 1")));
    main_sys.add(Box::new(ChildrenSystem::new("children 2")));
    main_sys.add(Box::new(ChildrenSystem::new("children 3")));
    main_sys.notify("update version v0.1.1");
}
