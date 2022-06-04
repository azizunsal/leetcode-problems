use core::num;

fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    // let mut nums = vec![0];
    // let mut nums = vec![0, 0, 1];
    // let mut nums = vec![1, 2, 3, 4];
    // let mut nums = vec![7, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 3, 0, 0, 0, 0, 0, 0];

    move_zeroes(&mut nums);
    println!("result {:?}", nums);
}

fn move_zeroes(nums: &mut Vec<i32>) {
    let l = nums.len();
    if l == 1 {
        return;
    }

    if !nums.contains(&0) {
        return;
    }
    let mut i: usize = 0;
    let mut counter = 0;
    while counter < l {
        let c = nums[i];
        if c == 0 {
            nums.push(c);
            nums.remove(i);
        } else {
            i += 1;
        }
        counter += 1;
    }
}
