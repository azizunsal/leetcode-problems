use std::time::Instant;
use std::{fs, vec};

fn main() {
    let prices: Vec<i32> = vec![7, 1, 5, 3, 6, 4];
    // let prices: Vec<i32> = vec![3, 10, 2, 4, 1, 5];
    let start = Instant::now();
    // let max_profit = max_profit(get_100000_items());
    // let max_profit = max_profit(get_67552_items());
    // let max_profit = max_profit(get_53347_items());
    // let max_profit = max_profit_with_brute_force(prices);
    let max_profit = max_profit(prices);
    let duration = start.elapsed();
    println!("Max profit= {}. Elapsed time= {:?}", max_profit, duration);
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_diff = 0;
    let mut memo: Vec<i32> = vec![-1; prices.len()];

    for i in (0..prices.len()).rev() {
        let buying_price = prices[i];
        // println!("buying price= {}", buying_price);
        let remainings = &prices[i + 1..];

        // println!(" {} --> {:?}", buying_price, remainings);
        // let res = find_max(remainings);
        let res = find_max_with_memoization(i, remainings, &mut memo);

        let diff = res - buying_price;
        // println!("i= {}, item= {}, res= {}, diff= {}", i, &prices[i], res, diff);
        if diff > max_diff {
            max_diff = diff;
        }
    }

    max_diff
}

fn find_max_with_memoization(index: usize, arr: &[i32], memo: &mut Vec<i32>) -> i32 {
    if arr.len() == 0 {
        return 0;
    }
    if arr.len() == 1 {
        return arr[0];
    }

    if memo[index] != -1 {
        return memo[index];
    } else {
        let arr1 = &arr[..1];
        let arr2 = &arr[1..];
        let res1 = find_max_with_memoization(index, arr1, memo);

        let res2 = find_max_with_memoization(index + 1, arr2, memo);
        let res_max;
        if res1 > res2 {
            res_max = res1;
        } else {
            res_max = res2;
        }
        memo[index] = res_max;
        return memo[index];
    }
}

#[allow(dead_code)]
fn find_max(arr: &[i32]) -> i32 {
    // println!("Trying to find max value in {:?}", arr);
    let mut max = 0;
    for i in arr {
        if max < *i {
            max = *i;
        }
    }
    max
}

#[allow(dead_code)]
fn max_profit_with_brute_force(prices: Vec<i32>) -> i32 {
    let mut max_diff = 0;
    println!("prices= {:?}", prices);

    for buy_i in 0..prices.len() {
        let buy_p = prices[buy_i];
        for sell_i in buy_i + 1..prices.len() {
            let sell_p = prices[sell_i];
            let diff = sell_p - buy_p;
            if buy_p > sell_p {
                continue;
            }
            if diff > max_diff {
                max_diff = diff;
            }
        }
    }
    max_diff
}

#[allow(dead_code)]
fn get_67552_items() -> Vec<i32> {
    let file_path = "/Users/azizunsal/projects/leetcode-problems/problem121-best-time-to-buy-and-sell-stock/big_array.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let big_arr: Vec<i32> = contents.split(",").map(|a| a.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    println!("Length= {}", big_arr.len());
    big_arr
}

#[allow(dead_code)]
fn get_53347_items() -> Vec<i32> {
    let file_path = "/Users/azizunsal/projects/leetcode-problems/problem121-best-time-to-buy-and-sell-stock/big_array-2.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let big_arr: Vec<i32> = contents.split(",").map(|a| a.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    println!("Length= {}", big_arr.len());
    big_arr
}

#[allow(dead_code)]
fn get_100000_items() -> Vec<i32> {
    let file_path = "/Users/azizunsal/projects/leetcode-problems/problem121-best-time-to-buy-and-sell-stock/big_array-3.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let big_arr: Vec<i32> = contents.split(",").map(|a| a.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    println!("Length= {}", big_arr.len());
    big_arr
}

#[cfg(test)]
mod test {
    use crate::{get_100000_items, get_53347_items, get_67552_items, res_start as max_profit};

    #[test]
    fn test_1() {
        let prices: [i32; 6] = [7, 1, 5, 3, 6, 4];
        assert_eq!(5, max_profit(prices.to_vec()));
    }

    #[test]
    fn test_no_profit() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(0, max_profit(prices));
    }

    #[test]
    fn test_3() {
        let prices = vec![2, 1, 4];
        assert_eq!(3, max_profit(prices));
    }

    #[test]
    fn test_4() {
        let prices = vec![2, 4, 1];
        assert_eq!(2, max_profit(prices));
    }

    #[test]
    fn test_perf_with_53347_items() {
        assert_eq!(999, max_profit(get_53347_items()));
    }

    #[test]
    fn test_perf_with_67552_items() {
        assert_eq!(999, max_profit(get_67552_items()));
    }

    #[test]
    fn test_perf_with_100000_items() {
        assert_eq!(0, max_profit(get_100000_items()));
    }
}
