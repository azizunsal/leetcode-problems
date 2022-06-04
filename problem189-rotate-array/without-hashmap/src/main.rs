fn main() {
    // let mut _nums = vec![-1, -100, 3, 99];
    // let _k = 2;

    let mut _nums = vec![1, 2,3];
    let _k = 2;

    rotate(&mut _nums, _k);
    println!("Rotated array is {:?}", _nums);
}

fn rotate(nums: &mut Vec<i32>, k: i32) {
    let orig_l = nums.len();
    if k == 0 || nums.len() <= 1 {
        return;
    }

    let optimized_k = k % orig_l as i32;
    println!("Array length={}, k={} and the optimized k= {}", orig_l, k, optimized_k);
    // Aslinda burada yaptigim, teknik olarak, yeni bir array yaratmak ile ayni sey.
    nums.resize(nums.len() * 2, 8888);
    for i in 0..orig_l {
        let mut new_index = i + optimized_k as usize;
        println!(" ->nex_index = {}", new_index);
        if new_index < orig_l {
            new_index = new_index + orig_l;
            println!(" -->nex_index = {}", new_index);
        }
        nums[new_index] = nums[i];
        println!(" =>arr {:?}", nums);
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
