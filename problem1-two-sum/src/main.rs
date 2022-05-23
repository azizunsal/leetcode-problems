// https://leetcode.com/problems/two-sum/
fn main() {
    let v1 = vec![3, 2, 4];
    let result = two_sum(v1, 6);
    println!("The result is {:?}", result);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (p1, e1) in nums.iter().enumerate() {
        for (p2, e2) in nums.iter().enumerate() {
            if p1 == p2 {
                continue;
            }
            if e1 + e2 == target {
                return vec![p1 as i32, p2 as i32];
            }
        }
    }
    vec![0, 0]
}
