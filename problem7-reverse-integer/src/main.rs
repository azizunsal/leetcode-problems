fn main() {
    let num: i32 = 123;
    let result = reverse(num);
    println!("Reversed number is {}", result);
}

fn reverse(x: i32) -> i32 {
    // println!("Number is {}, zeroes = {}", x, x.trailing_zeros());
    let x_str: String = x.to_string().chars().filter(|c| c.is_digit(10)).rev().collect();
    // println!("x_str {}", x_str);

    use std::str::FromStr;
    let reversed_num = i32::from_str(&x_str);

    if reversed_num.is_err() {
        return 0;
    } else {
        if x.is_negative() {
            return -reversed_num.unwrap();
        } else {
            return reversed_num.unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::reverse;

    #[test]
    fn test_with_a_positive_number() {
        let num = 123;
        assert_eq!(321, reverse(num));
    }

    #[test]
    fn test_with_a_negative_number() {
        let num = -123;
        assert_eq!(-321, reverse(num));
    }

    #[test]
    fn test_with_trailing_zeros_and_negative() {
        let num = -901000;
        assert_eq!(-109, reverse(num));
    }

    #[test]
    fn test_with_a_trailing_zero() {
        let num = 120;
        assert_eq!(21, reverse(num));
    }

    #[test]
    fn test_with_trailing_zeros() {
        let num = 901000;
        assert_eq!(109, reverse(num));
    }

    #[test]
    fn test_with_i32_max() {
        assert_eq!(0, reverse(i32::MAX));
    }

    #[test]
    fn test_with_i32_min() {
        assert_eq!(0, reverse(i32::MIN));
    }

    #[test]
    fn test_with_zero() {
        let num = 0;
        assert_eq!(0, reverse(num));
    }
}
