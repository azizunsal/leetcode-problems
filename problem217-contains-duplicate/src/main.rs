use std::collections::HashMap;
use std::time::Instant;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let s = Instant::now();
    let _nums1 = vec![1, 2, 3, 4];
    let _nums2 = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
    let _nums3 = get_long_vector();
    let result = contains_duplicate(_nums2);
    println!("Contains duplicate item? {}. Elapsed time {:?}", result, s.elapsed());
}

// With standard brute force
fn _contains_duplicate(nums: Vec<i32>) -> bool {
    println!("Array length: {}", nums.len());
    if nums.len() <= 1 {
        return false;
    }

    for (p, el) in nums.iter().enumerate() {
        for (pp, ell) in nums.iter().enumerate() {
            if p == pp {
                continue;
            }

            if el == ell {
                return true;
            }
        }
    }

    false
}
// with a hash map
fn contains_duplicate(nums: Vec<i32>) -> bool {
    println!("Array len: {:?}", nums.len());
    if nums.len() <= 1 {
        return false;
    }

    let mut map: HashMap<i32, i32> = HashMap::new();

    for (_, el) in nums.iter().enumerate() {
        if map.contains_key(el) {
            // No need to collect recurring items. Just 1 is OK to return the result!
            // *map.get_mut(el).unwrap() += 1;
            return true;
        } else {
            map.insert(*el, 1);
        }
    }

    false

    
    // let a = map.iter().filter(|(_, v)| *v > &1).map(|(_, b)| b).collect::<Vec<&i32>>();
    // println!("There are {} items which repeats", a.len());
    // a.len() > 0
}

#[cfg(test)]
mod test {
    use crate::{contains_duplicate, get_long_vector};

    #[test]
    fn test_with_a_recurring_array() {
        let nums = vec![1, 2, 3, 1];
        assert!(contains_duplicate(nums));
    }

    #[test]
    fn test_with_a_non_recurring_array() {
        let nums = vec![1, 2, 3, 4];
        assert!(!contains_duplicate(nums));
    }

    #[test]
    fn test_with_another_recurring_array() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert!(contains_duplicate(nums));
    }

    #[test]
    fn test_with_a_big_array() {
        contains_duplicate(get_long_vector());
    }
}

fn get_long_vector() -> Vec<i32> {
    let v = Vec::new();
    let filename = "/Users/azizunsal/projects/leetcode-problems/problem217-contains-duplicate/large_array.txt";
    let lines = lines_from_file(filename);
    if lines.len() > 1 {
        println!("There should be only 1 line. Check the source txt file's format!");
        return v;
    }
    let line: &String = lines.get(0).unwrap();

    // Split string by `,`
    let split_res = line.split(',');

    let res = split_res.map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    println!("Big array created from file '{filename}'. Array size is {}", res.len());
    res
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}
