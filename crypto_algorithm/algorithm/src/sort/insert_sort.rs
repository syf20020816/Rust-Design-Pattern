//! # 插入排序
//! 从第一个元素开始向后遍历，遍历一个选择该元素和它之前的比较找到比它小的元素放在该元素的后面
//! 然后其他元素以此向后移动一位
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/9
//! @version:0.0.1
//! @description:
//! ```

fn insert_sort(list: &mut Vec<u32>) -> () {
    for i in 1..list.len() {
        let mut flag = i;
        while flag > 0 && list[flag] < list[flag - 1] {
            list.swap(flag, flag - 1);
            flag -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut list = vec![4, 2, 6, 7, 1, 9];
        insert_sort(&mut list);
        assert_eq!(list, vec![1, 2, 4, 6, 7, 9]);
    }
}