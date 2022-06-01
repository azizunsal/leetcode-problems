fn main() {
    let mut _nums = vec![1, 2, 3, 4, 5, 6, 7];
    let _k = 3;

    let mut _nums = vec![1, 2];
    let _k = 3;

    rotate(&mut _nums, _k);
    println!("Rotated array is {:?}", _nums);
}

fn rotate(nums: &mut Vec<i32>, k: i32) {
    let orig_l = nums.len();
    if k == 0 || nums.len() <= 1 {
        return;
    }

    let optimized_k = k % orig_l as i32;
    println!("Resized array is {:?}. k={} and the optimized k: {}", nums, k, optimized_k);
    nums.resize(nums.len() * 2, 0);
    for i in 0..orig_l {
        let mut new_index = i + optimized_k as usize;
        if new_index < orig_l {
            new_index = new_index + orig_l;
        }
        nums[new_index] = nums[i];
    }

    let new_vec = nums
        .iter_mut()
        .enumerate()
        .filter(|(a, _b)| *a >= orig_l)
        .map(|(_, e)| *e)
        .collect::<Vec<i32>>();

    *nums = new_vec;
}

#[cfg(test)]
mod test {
    use crate::rotate;

    #[test]
    fn test_with_three_step() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        rotate(&mut nums, k);
        assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], nums);
    }

    #[test]
    fn test_with_two_step() {
        let mut nums = vec![-1, -100, 3, 99];
        let k = 2;
        rotate(&mut nums, k);
        assert_eq!(vec![3, 99, -1, -100], nums);
    }

    #[test]
    fn test_with_single_item_array() {
        let mut nums = vec![-1];
        let k = 2;
        rotate(&mut nums, k);
        assert_eq!(vec![-1], nums);
    }

    #[test]
    fn test_when_step_is_bigger_than_array_length() {
        let mut nums = vec![1, 2];
        let k = 3;
        rotate(&mut nums, k);
        assert_eq!(vec![2, 1], nums);
    }
}
