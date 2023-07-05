//! # Array Impl
//! depend vector
//! ## Structure
//!
//! ``` code
//!
//!          ——————
//!          |    |
//!        |ele|  |   ...
//!          ↓️    ↓
//!        ---------------------------------
//! data:  | 9 | 5 | 2 | 1 | 4 | 0 | 6 | 6 |
//!        ---------------------------------
//! index: | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
//!        ---------------------------------
//!        |->           len             <-|
//!          ↑
//!       current
//! ```
//!
//! ## Functions
//!
//! 1. new (init an empty array)
//! 2. new_bind (init an array with binding length)
//! 3. push (add a new element into array)
//! 4. pop (remove the end element from array)
//! 5. remove (remove an array element with a specified index)
//! 6. is_empty (judge array is empty or not)
//! 7. is_full (judge if the array is full)
//!
//!
//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/4
//! @version:0.0.1
//! @description:
//! ```


use std::ops::{Deref, DerefMut};

/// 数组
#[derive(Debug, Clone)]
pub struct Array<T> {
    elements: Vec<Element<T>>,
    length: u32,
    current: u32,
}

/// # 数组的元素
/// ## Function
/// 1. new
/// 2. set_data
/// 3. get_data (get data use clone)
/// 4. get_data_ref (get ref)
/// 5. get_index (index ref)
/// 6. mount_next (set next element)
#[derive(Debug, Clone)]
pub struct Element<T> {
    data: T,
    index: u32,
}

macro_rules! element_impl {
    ($($el:ty)*) => ($(
    impl Element<$el>{
        pub fn new(data: $el, index: u32) -> Self {
           Element {
                data,
                index
           }
        }
        pub fn set_data(&mut self, data: $el) -> () {
            self.data = data
        }
        pub fn get_data(&self) -> $el {
            self.data.clone()
        }
        pub fn get_data_ref(&self) -> &$el {
            &self.data
        }
        pub fn get_index(&self) -> &u32 {
            &self.index
        }
    }
    )*);
}

element_impl! {u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 bool f32 f64 &'static str String}


impl Array<u8> {
    pub fn new() -> Self {
        //empty
        // if use new_bind : self.max = Some(u32)
        // new : self.max = None
        Array { elements: Vec::new(), length: 0, current: 0 }
    }
    /// 预先分配
    pub fn new_bind(length: u32) -> Self {
        let mut elements = Vec::new();
        for i in 0..length {
            elements.push(Element::<u8>::new(0 as u8, i as u32))
        };
        Array {
            elements,
            length,
            current: 0,
        }
    }
    fn get_current(&self) -> &u32 {
        &self.current
    }
    fn get_length(&self) -> &u32 {
        &self.length
    }
    pub fn is_empty(&self) -> bool {
        0_u32.eq(self.get_length())
    }
    pub fn is_full(&self) -> bool {
        self.get_current().eq(self.get_length())
    }
    pub fn push(&mut self, data: u8) -> () {
        let _ = self.elements.push(Element::<u8>::new(data, self.get_length().clone()));
        self.length += 1;
    }
    pub fn pop(&mut self) -> () {
        let _ = self.elements.pop();
        self.length -= 1;
    }
    pub fn remove(&mut self, index: u32) -> () {
        let _ = self.elements.remove(index as usize);
        self.length -= 1;
    }
    pub fn get(&self, index: u32) -> &Element<u8> {
        &self.elements.get(index as usize).unwrap()
    }
    pub fn set(&mut self, index: u32, value: u8) -> () {
        let _ = self.elements.get_mut(index as usize).unwrap().set_data(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element() -> () {
        let mut e = Element::<String>::new("你好".to_string(), 0);
        println!("{:?}", e);
        println!("{}", e.get_data());
        let _ = e.set_data("hello".to_string());
        println!("{}", e.get_data_ref());
    }


    #[test]
    fn test_array_new() -> () {
        let mut arr = Array::<u8>::new();
        println!("{:?}", &arr);
        println!("{}", arr.is_empty());
        println!("{}", arr.is_full());
        let _ = arr.push(56);
        let _ = arr.push(99);
        let _ = arr.push(110);
        println!("{:?}", &arr);
        println!("{}", arr.is_empty());
        println!("{}", arr.is_full());
    }

    #[test]
    fn test_array_bind() -> () {
        let mut arr = Array::<u8>::new_bind(5);
        arr.pop();
        arr.remove(1);
        println!("{:?}", &arr);
        println!("{}", arr.is_empty());
        println!("{}", arr.is_full());
        let _ = arr.push(56);
        arr.set(2,110);
        println!("{:?}", &arr);
        println!("{}", arr.is_empty());
        println!("{}", arr.is_full());
        println!("{:?}", arr.get(1));
    }
}