mod apis;

use crate::apis::{test_stack,test_queue};
use structure::{LinkedList};

fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    dbg!(&list);
    list.pop();
    dbg!(list.peek());
    let mut iter = list.into_iter();
    dbg!(&iter.next());
    dbg!(&iter.next());
    dbg!(&iter.next());
}
