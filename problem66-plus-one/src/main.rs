fn main() {
    let digits = vec![1, 2, 9];
    let res_digits = plus_one(digits);
    println!("Result digits {:?}", res_digits);
}

fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    println!("New _Plus One_ for {:?}", digits);
    let mut r: Vec<i32> = digits.clone();
    let l = digits.len();

    let mut pointer: i32 = l as i32 - 1;

    while pointer >= 0 {
        let last_digit = r[pointer as usize];
        // println!("Last digit={}, pointer={}", last_digit, pointer);
        if last_digit == 9 {
            r[pointer as usize] = 0;
            if pointer == 0 {
                r.insert(0, 1);
            }
        } else {
            r[pointer as usize] += 1;
            // println!("BREAK");
            break;
        }
        pointer -= 1;
    }
    r
}

#[cfg(test)]
mod test {
    use crate::plus_one;

    #[test]
    fn test_1() {
        let _digits = vec![1, 2, 3];
        assert_eq!(vec![1, 2, 4], plus_one(_digits));
    }
    #[test]
    fn test_2() {
        let _digits = vec![9];
        assert_eq!(vec![1, 0], plus_one(_digits));
    }
    #[test]
    fn test_3() {
        let _digits = vec![1, 2, 9];
        assert_eq!(vec![1, 3, 0], plus_one(_digits));
    }

    #[test]
    fn test_4() {
        let _digits = vec![4, 3, 2, 1];
        assert_eq!(vec![4, 3, 2, 2], plus_one(_digits));
    }
}
