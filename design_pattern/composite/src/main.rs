mod lib;

use crate::lib::{Component, Leaf, Branch};

fn main() {
    let mut root = Branch::new("根节点", 1);
    let mut branch1 = Branch::new("树枝1", 2);
    let mut branch2 = Branch::new("树枝2", 2);
    let mut branch1_1 = Branch::new("树枝1-1", 3);
    let leaf1 = Leaf::new("叶子1", 4);
    let leaf2 = Leaf::new("叶子2", 4);
    let leaf3 = Leaf::new("叶子3", 3);
    let leaf4 = Leaf::new("叶子4", 4);
    let leaf5 = Leaf::new("叶子5", 2);
    let leaf6 = Leaf::new("叶子6", 3);

    branch1_1.add(Box::new(leaf1));
    branch1_1.add(Box::new(leaf2));
    branch1_1.add(Box::new(leaf4));
    branch1.add(Box::new(leaf3));
    branch2.add(Box::new(leaf6));
    root.add(Box::new(branch1));
    root.add(Box::new(branch2));
    root.add(Box::new(leaf5));

    let res = root.get_child(0);


}
