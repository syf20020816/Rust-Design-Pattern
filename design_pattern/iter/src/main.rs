mod lib;
mod impl_iter;

// use crate::lib::{Node,NodeAggregate,NodeList,Iter,Aggregate};
use crate::impl_iter::{Node, NodeList};

fn main() {
    // let mut origin = NodeAggregate::new();
    // origin.add(Node::new("1"));
    // origin.add(Node::new("2"));
    // origin.add(Node::new("3"));
    // origin.add(Node::new("4"));
    // origin.remove(1);
    // let mut iter = origin.getIter();
    // dbg!(&iter.hasNext());
    // dbg!(&iter.next());
    // dbg!(&iter.next());
    let mut list = NodeList::new();
    list.add(1);
    list.add(2);
    list.add(3);
    list.add(4);
    dbg!(&list);
    let iter = list.iter();
    for node in iter {
        println!("{}",node);
    }
}
