fn sum_numbers(numbers: &[u32]) -> Option<u32> {
    numbers.iter().try_fold(0, |acc:u32, &num| acc.checked_add(num))
}



mod tests {
    use super::*;

    #[test]
    fn test_sum_numbers() {
        // 测试空数组的情况
        assert_eq!(sum_numbers(&[]), Some(0));

        // 测试包含正数的数组
        assert_eq!(sum_numbers(&[1, 2, 3]), Some(6));
 
        // 测试溢出的情况
        assert_eq!(sum_numbers(&[u32::MAX, 1]), None);
    }
}