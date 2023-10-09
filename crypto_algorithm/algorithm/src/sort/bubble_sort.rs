/// 冒泡排序 O(n^2)
/// 循环进行排序，检测元素1与元素2进行比较，若元素1>元素2，元素1与元素2进行交换，后面继续
/// ```code
///   |———|
///   |   ↓
/// |————————————————————|
/// | 4 | 1 | 2 | 10 | 7 |
/// |————————————————————|
///           ↓ first
/// |————————————————————|
/// | 1 | 4 | 2 | 10 | 7 |   ......
/// |————————————————————|
/// ```
pub fn bubble_sort<T: PartialOrd>(eles: &mut [T]) {
    for o in 0..eles.len() {
        let mut sorted = true;
        for i in 0..(eles.len() - 1) - o {
            if eles[i] > eles[i + 1] {
                eles.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble() {
        let mut origin = vec![4, 1, 2, 10, 7];
        let _ = bubble_sort(&mut origin);
        assert_eq!(vec![1, 2, 4, 7, 10], origin);
    }
}