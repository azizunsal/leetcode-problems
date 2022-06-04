use std::collections::HashMap;

fn main() {
    let _single_num1 = vec![2, 2, 1];
    let _single_num2 = vec![4, 1, 2, 1, 2];
    let s = single_number(_single_num1);
    println!("Single num in the arr is {}", s);
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    println!("**nums {:?}", nums);
    if nums.len() == 1 {
        return nums[0];
    }

    let mut m: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    for i in nums {
        let a = m.entry(i).or_insert(0);
        *a += 1;
    }
    *m.iter_mut().filter(|(_, v)| *v == &1).map(|(k, _)| k).nth(0).unwrap()
}

#[cfg(test)]
mod test {
    use crate::single_number;

    #[test]
    fn test_1() {
        let _single_num = vec![4, 1, 2, 1, 2];
        assert_eq!(4, single_number(_single_num));
    }
    #[test]
    fn test_2() {
        let _single_num = vec![2, 2, 1];
        assert_eq!(1, single_number(_single_num));
    }
}
