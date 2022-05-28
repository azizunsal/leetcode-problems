// use std::collections::HashMap;
use std::collections::BTreeMap;

fn main() {
    let num = 58;
    let res = int_to_roman(num);
    println!("Res is {}", res);
}

fn int_to_roman(num: i32) -> String {
    println!("Number to be converted to roman letters is {}", num);
    // 1000 =>  M
    // 500  =>  D
    // 100  =>  C
    // 50   =>  L
    // 10   =>  X
    // 5    =>  V
    // 1    =>  I
    // println!("Letter is {}", letter);
    let mut my_num = num.clone();
    let mut map = BTreeMap::new();
    let romans: [i32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    for i in &romans {
        let res = (my_num / i) as usize;
        let remainder = my_num % i;
        map.insert(i, res);
        my_num = remainder;
    }
    println!("mynum={}, Map is {:?}", my_num, map);
    let mut in_roman = String::new();
    for (k, v) in map.iter().rev() {
        if v > &0 {
            let roman = get_roman_by_number(k, *v);

            if roman.is_some() {
                let r_v = roman.unwrap();
                println!("k={}, v={}, roman={}", k, v, r_v);
                in_roman.push_str(&r_v);
            }
        }
    }

    in_roman
}

fn get_roman_by_number(num: &i32, times: usize) -> Option<String> {
    let a = match num {
        1000 => String::from("M"),
        900 => String::from("CM"),
        500 => String::from("D"),
        400 => String::from("CD"),
        100 => String::from("C"),
        90 => String::from("XC"),
        50 => String::from("L"),
        40 => String::from("XL"),
        10 => String::from("X"),
        9 => String::from("IX"),
        5 => String::from("V"),
        4 => String::from("IV"),
        1 => String::from("I"),
        _ => String::from("NOOOOO"),
    };

    Some(a.repeat(times))
}

#[cfg(test)]
mod test {
    use crate::int_to_roman;
    #[test]
    fn test_with_10() {
        let num = 10;
        assert_eq!("X", int_to_roman(num));
    }

    #[test]
    fn test_with_128() {
        let num = 128;
        assert_eq!("CXXVIII", int_to_roman(num));
    }
    #[test]
    fn test_with_58() {
        let num = 58;
        assert_eq!("LVIII", int_to_roman(num));
    }

    #[test]
    fn test_with_163() {
        let num = 163;
        assert_eq!("CLXIII", int_to_roman(num));
    }

    #[test]
    fn test_with_1994() {
        let num = 1994;
        assert_eq!("MCMXCIV", int_to_roman(num));
    }
    
}
