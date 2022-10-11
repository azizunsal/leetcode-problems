use std::time::Instant;

fn main() {
    let start = Instant::now();
    let prices: Vec<i32> = vec![7, 1, 5, 3, 6, 4];
    let max_profit = max_profit(prices);
    let duration = start.elapsed();
    println!("Max profit= {}. Elapsed time= {:?}", max_profit, duration);
}

fn max_profit(prices: Vec<i32>) -> i32 {
    println!("prices {:?}", prices);
    let max_profit = 0;

    max_profit
}

#[cfg(test)]
mod test {
    use crate::max_profit;

    #[test]
    fn test_1() {
        let prices: [i32; 6] = [7, 1, 5, 3, 6, 4];
        assert_eq!(7, max_profit(prices.to_vec()));
    }
}
