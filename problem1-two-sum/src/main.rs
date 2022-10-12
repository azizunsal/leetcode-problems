use std::{collections::HashMap, fs, time::Instant, vec};
// https://leetcode.com/problems/two-sum/
fn main() {
    let start = Instant::now();
    // let nums = vec![2, 7, 11, 15];
    // let target = 9;
    // let nums = vec![3, 2, 4];
    // let target = 6;
    // let nums = vec![2, 7, 11, 15, 17, 18, 19, 20, 21, 22, 23, 12, 1, 11, 13, 23, 11, 22];
    // let target = 45;
    let nums = get_10000_items();
    let target = 19999;

    // let result = two_sum_with_hash_table(nums, target);
    let result = two_sum_without_hash_table(nums, target);
    println!("Result= {:?}. Elapsed time= {:?}", result, start.elapsed());
}

fn iterate(map: &mut HashMap<i32, (i32, i32)>, inc: usize, nums: &Vec<i32>, target: i32) -> bool {
    for i in 0..nums.len() - inc {
        let keyy = nums[i] + nums[i + inc];
        let val = (i as i32, i as i32 + inc as i32);
        map.insert(keyy, val);

        if keyy == target {
            return true;
        }
    }

    return false;
}

fn two_sum_with_hash_table(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, (i32, i32)> = HashMap::new();
    let mut inc: usize = 1;
    let mut found: bool = false;
    while !found {
        found = iterate(&mut map, inc, &nums, target);
        inc += 1;
    }
    let result = map.get(&target).unwrap();

    vec![result.0, result.1]
}

fn iterate_wo(inc: usize, nums: &Vec<i32>, target: i32) -> Option<(i32, i32)> {
    for i in 0..nums.len() - inc {
        let keyy = nums[i] + nums[i + inc];
        let val = (i as i32, i as i32 + inc as i32);

        if keyy == target {
            return Some(val);
        }
    }

    return None;
}

fn two_sum_without_hash_table(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut inc: usize = 1;
    let mut found: Option<(i32, i32)> = None;
    while found.is_none() {
        found = iterate_wo(inc, &nums, target);
        inc += 1;
    }
    vec![found.unwrap().0, found.unwrap().1]
}

fn two_sum_first_posted_solution(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    for (p1, e1) in nums.iter().enumerate() {
        for (p2, e2) in nums.iter().enumerate() {
            if p1 == p2 {
                continue;
            }
            // println!("{e1}+{e2}={}", e1 + e2);
            if e1 + e2 == target {
                return vec![p1 as i32, p2 as i32];
            }
        }
    }
    vec![0, 0]
}

fn two_sum_second_posted_solution(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut s = 0;
    let limit = nums.len() - 1;
    let mut e = limit;

    while s < e {
        if nums[s as usize] + nums[e as usize] == target {
            return vec![s as i32, e as i32];
        }
        e -= 1;
        if e == s {
            s += 1;
            e = limit;
        }
    }
    vec![]
}

#[allow(dead_code)]
fn get_10000_items() -> Vec<i32> {
    let file_path = "/Users/azizunsal/projects/leetcode-problems/problem1-two-sum/src/big-array.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let big_arr: Vec<i32> = contents.split(",").map(|a| a.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    println!("Length= {}", big_arr.len());
    big_arr
}

#[cfg(test)]
mod test {
    use crate::{get_10000_items, two_sum_with_hash_table as two_sum};

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 2], two_sum(vec![3, 2, 4], 6));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![0, 1], two_sum(vec![3, 3], 6));
    }

    #[test]
    fn test_4() {
        assert_eq!(vec![0, 2], two_sum(vec![3, 2, 3], 6));
    }

    #[test]
    fn test_perf() {
        assert_eq!(vec![9998, 9999], two_sum(get_10000_items(), 19999));
    }
}
