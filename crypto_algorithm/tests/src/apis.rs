use structure::{Stack, Queue};

pub fn test_stack() -> () {
    let mut stack = Stack::new();
    dbg!(&stack.is_empty());
    stack.push(56_u32);
    stack.push(1_u32);
    stack.push(89_u32);
    stack.push(106_u32);
    dbg!(&stack);
    dbg!(&stack.is_empty());
    dbg!(stack.pop());
    dbg!(&stack.peek());
    dbg!(&stack);
}

pub fn test_queue() -> () {
    let mut queue = Queue::new(5_usize);
    dbg!(&queue.is_empty());
    queue.push(10_u32);
    queue.push(101_u32);
    queue.push(46_u32);
    queue.push(2_u32);
    dbg!(&queue.is_empty());
    dbg!(&queue.len());
    dbg!(&queue.pop());
    dbg!(&queue);
}