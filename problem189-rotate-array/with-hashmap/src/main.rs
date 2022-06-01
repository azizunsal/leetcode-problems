use std::collections::HashMap;

fn main() {
    let mut _nums = vec![1, 2, 3, 4, 5, 6, 7];
    let _k = 3;

    let mut _nums = vec![1, 2];
    let _k = 3;

    rotate(&mut _nums, _k);
    println!("Rotated array is {:?}", _nums);
}

fn rotate(nums: &mut Vec<i32>, k: i32) {
    if k == 0 || k > nums.len() as i32 || nums.len() <= 1 {
        return;
    }

    let mut map: HashMap<usize, i32> = HashMap::with_capacity(nums.len());
    for i in 0..nums.len() {
        let mut new_index = i + k as usize;
        if new_index >= nums.len() {
            new_index -= nums.len();
        }
        map.insert(new_index, nums[i]);
    }

    for (k, v) in map {
        nums[k] = v;
    }
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
