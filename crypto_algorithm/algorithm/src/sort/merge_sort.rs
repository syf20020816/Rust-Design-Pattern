// pub fn merge_sort<T: PartialOrd>(mut list: Vec<T>) -> Vec<T> {
//     if 1.ge(&list.len()) {
//         return list;
//     }
//     // 将vec分为两个部分（一半）, 再分一半 ...
//     let first_half = list.split_off(list.len() / 2);
//     let second = merge_sort(list);
//     first_half = merge_sort(first_half);
//
//     return list;
// }