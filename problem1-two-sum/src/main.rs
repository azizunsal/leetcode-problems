use std::{collections::HashMap, fs, time::Instant, vec};
// https://leetcode.com/problems/two-sum/
fn main() {
    // let _v2 = vec![3, 2, 4];
    // let _t2 = 6;
    // let mut _v2 = vec![2, 7, 11, 15, 17, 18, 19, 20, 21, 22, 23];
    // let _t2 = 45;
    // let _v2 = vec![3, 2, 3];
    // let _t2 = 6;
    // println!("items={:?}", _v2);
    let _v2 = get_10000_items();
    let _t2 = 19999;
    let s1 = Instant::now();
    let result = two_sum(&_v2, _t2);
    println!("Result-1= {:?}. Elapsed time= {:?}", result, s1.elapsed());

    println!("--");
    let s2 = Instant::now();
    let result2 = two_sum_alternate(&_v2, _t2);
    println!("Result-2= {:?}. Elapsed time= {:?}", result2, s2.elapsed());

    println!("--");
    let s3 = Instant::now();
    let result3 = two_sum_with_hash_table(&_v2, _t2);
    println!("Result-3= {:?}. Elapsed time= {:?}", result3, s3.elapsed());
}

fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
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

fn two_sum_alternate(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    // println!("nums= {:?}, target= {}", nums, target);
    let mut s = 0;
    let mut e = nums.len() - 1;

    while s < e {
        let a = nums[s];
        let b = nums[e];
        // println!("{a}+{b}={}", a + b);
        if a + b == target {
            // println!("OK");
            return vec![s as i32, e as i32];
        }
        e -= 1;
        if e == s {
            s += 1;
            e = nums.len() - 1;
        }
    }
    vec![0, 0]
}

fn call_me(map: &mut HashMap<i32, (i32, i32)>, inc: usize, nums: &Vec<i32>, target: i32) -> bool {
    for i in 0..nums.len() - inc {
        let keyy = nums[i] + nums[i + inc];
        let val = (i as i32, i as i32 + inc as i32);
        map.insert(keyy, val);
    }

    if map.get(&target).is_some() {
        return true;
    }
    return false;
}

fn two_sum_with_hash_table(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, (i32, i32)> = HashMap::new();
    let mut inc: usize = 1;
    let mut found: bool = false;
    while !found {
        found = call_me(&mut map, inc, nums, target);
        inc += 1;
    }
    let rezz = map.get(&target);

    vec![rezz.unwrap().0, rezz.unwrap().1]
}

pub fn two_sum_POSTED(nums: &Vec<i32>, target: i32) -> Vec<i32> {
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
        assert_eq!(vec![1, 2], two_sum(&vec![3, 2, 4], 6));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![0, 1], two_sum(&vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![0, 1], two_sum(&vec![3, 3], 6));
    }

    #[test]
    fn test_4() {
        assert_eq!(vec![0, 2], two_sum(&vec![3, 2, 3], 6));
    }

    #[test]
    fn test_perf() {
        assert_eq!(vec![9998, 9999], two_sum(&get_10000_items(), 19999));
    }
}
