fn main() {
    // let roman = String::from("III");
    // let roman = String::from("LV");
    let roman_num = String::from("LVIII");
    // let roman = String::from("LIV");
    // let roman = String::from("MCMXCIV");
    let res = roman_to_int(roman_num);

    println!("Integer representation is {}", res);
}

pub fn roman_to_int(s: String) -> i32 {
    println!("Trying to convert roman {} to interger.", s);
    let mut letters: Vec<i32> = Vec::with_capacity(15);
    let mut prev_letter: char = '\0';

    for i in s.chars() {
        let mut rep: String = get_int_rep_by_roman_letter(&(i.to_string()));
        let mut int_rep: i32 = rep.clone().parse::<i32>().unwrap();

        if prev_letter.is_control() {
            letters.push(int_rep);
        } else {
            let previous_int_rep = get_int_rep_by_roman_letter(&prev_letter.to_string()).parse::<i32>().unwrap();
            if previous_int_rep < int_rep {
                // We have to consider the previous and the current letters as a single one.
                let mut double_letter_entry: String = String::with_capacity(2);
                double_letter_entry.push(prev_letter);
                double_letter_entry.push(i);
                println!("**Double letters entry is {}", double_letter_entry);
                rep = get_int_rep_by_roman_letter(&double_letter_entry);
                int_rep = rep.clone().parse::<i32>().unwrap();
                println!("---------previous_int_rep: {}", previous_int_rep);
                for (_, el) in letters.iter_mut().enumerate() {
                    if *el == previous_int_rep {
                        *el = int_rep;
                    }
                }
            } else {
                letters.push(int_rep);
            }
        }
        prev_letter = i;
    }

    println!("Overall str result is {:?}", letters);
    letters.iter().sum()
}

pub fn get_int_rep_by_roman_letter(roman_char: &String) -> String {
    let integer_rep: &str = match roman_char.as_str() {
        "M" => "1000",
        "CM" => "900",
        "D" => "500",
        "CD" => "400",
        "C" => "100",
        "XC" => "90",
        "L" => "50",
        "XL" => "40",
        "X" => "10",
        "IX" => "9",
        "V" => "5",
        "IV" => "4",
        "I" => "1",
        _ => "!!!",
    };
    integer_rep.to_string()
}

#[cfg(test)]
mod test {
    use crate::roman_to_int;

    #[test]
    fn test_with_roman_letter_v() {
        let letter = String::from("V");
        let result = roman_to_int(letter);

        assert_eq!(5, result);
    }

    #[test]
    fn test_with_roman_letter_lv() {
        let letter = String::from("LV");
        let result = roman_to_int(letter);

        assert_eq!(55, result);
    }
    #[test]
    fn test_with_roman_letter_mcmxciv() {
        let letter = String::from("MCMXCIV");
        let result = roman_to_int(letter);

        assert_eq!(1994, result);
    }

    #[test]
    fn test_with_roman_letter_lvii() {
        let letter = String::from("LVIII");
        let result = roman_to_int(letter);

        assert_eq!(58, result);
    }

    #[test]
    fn test_with_roman_letter_iii() {
        let letter = String::from("III");
        let result = roman_to_int(letter);

        assert_eq!(3, result);
    }

    #[test]
    fn test_with_roman_letter_liv() {
        let letter = String::from("LIV");
        let result = roman_to_int(letter);

        assert_eq!(54, result);
    }
}
