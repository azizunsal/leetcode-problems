fn main() {
    let mut _nums = vec![1, 1, 2];
    // let mut _nums = vec![1, 2, 3, 4, 5, 6];
    let res = remove_duplicates(&mut _nums);
    println!("resulst len {} of the result array {:?}", res, _nums);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] == nums[j] {
                nums[j] = 500;
            }
        }
    }
    nums.retain(|k| *k != 500);
    nums.len() as i32
}

#[cfg(test)]
mod test {

    use crate::remove_duplicates;

    #[test]
    fn test_1() {
        let mut _nums = vec![1, 1, 2];
        assert_eq!(2, remove_duplicates(&mut _nums));
    }

    #[test]
    fn test_2() {
        let mut _nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(5, remove_duplicates(&mut _nums));
    }
}
