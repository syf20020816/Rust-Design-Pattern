//! # 选择排序
//! 选择第一位开始比较，遍历剩下的元素，找到比第一位小的与第一位交换
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/9
//! @version:0.0.1
//! @description:
//! ```

use std::fmt::Debug;

fn select_sort(list: &mut Vec<u32>) -> () {
    for i in 0..list.len() - 1 {
        let mut smaller_index = i;
        let mut smaller = list[i];
        for j in (i + 1)..list.len() {
            if smaller >= list[j] {
                // 记录更小的元素下标
                smaller_index = j;
                smaller = list[j];
            }
        }
        //交换元素
        list.swap(i, smaller_index);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut list = vec![4, 2, 6, 7, 1, 9];
        select_sort(&mut list);
        assert_eq!(list, vec![1, 2, 4, 6, 7, 9]);
    }

}