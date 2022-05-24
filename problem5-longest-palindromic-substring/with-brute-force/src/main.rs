use std::time::Instant;

fn main() {
    // let s = "cbbd".to_string();
    let s="jhgtrclvzumufurdemsogfkpzcwgyepdwucnxrsubrxadnenhvjyglxnhowncsubvdtftccomjufwhjupcuuvelblcdnuchuppqpcujernplvmombpdttfjowcujvxknzbwmdedjydxvwykbbamfnsyzcozlixdgoliddoejurusnrcdbqkfdxsoxxzlhgyiprujvvwgqlzredkwahexewlnvqcwfyahjpeiucnhsdhnxtgizgpqphunlgikogmsffexaeftzhblpdxrxgsmeascmqngmwbotycbjmwrngemxpfakrwcdndanouyhnnrygvntrhcuxgvpgjafijlrewfhqrguwhdepwlxvrakyqgstoyruyzohlvvdhvqmzdsnbtlwctetwyrhhktkhhobsojiyuydknvtxmjewvssegrtmshxuvzcbrabntjqulxkjazrsgbpqnrsxqflvbvzywzetrmoydodrrhnhdzlajzvnkrcylkfmsdode".to_string();
    longest_palindrome(s);
}

fn longest_palindrome(s: String) -> String {
    // println!("Testing the string '{}'", s);
    let start = Instant::now();

    let l = s.len();
    let mut palindromes: Vec<&str> = Vec::new();

    for i in 0..l + 1 {
        for j in i + 1..l + 1 {
            let c = &s[i..j];
            let is_p = is_palindrome(c);
            // println!("'{}' is a palindrome ?{}", c, is_p);
            if is_p {
                palindromes.push(c);
            }
        }
    }
    // let step1_duration = start.elapsed();
    // let step2_start = Instant::now();

    println!("All palindromes for string '{}' are {:?}.", &s, palindromes,);
    let mut max_l = 0;
    let mut longest_palindrome = String::new();
    for p in palindromes {
        // println!("Testing {}. Len={}, test={}", p, p.len(), p.len() > max_l);
        if p.len() > max_l {
            longest_palindrome = p.to_string();
            max_l = longest_palindrome.len() as usize;
        }
    }
    let duration = start.elapsed();
    println!(
        "The longest palindrome is '{}'. Elapsed time is {:?}",
        longest_palindrome, duration
    );
    longest_palindrome
}

fn is_palindrome(s: &str) -> bool {
    let reverse_letters = s.chars().rev().collect::<String>();
    return s == &reverse_letters[0..];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_a_valid_string() {
        let s = "babad".to_string();
        assert_eq!("bab", longest_palindrome(s));
    }

    #[test]
    fn test_with_another_valid_string() {
        let s = "cbbd".to_string();
        assert_eq!("bb", longest_palindrome(s));
    }

    #[test]
    fn test_with_a_loooooong_string() {
        let s="jhgtrclvzumufurdemsogfkpzcwgyepdwucnxrsubrxadnenhvjyglxnhowncsubvdtftccomjufwhjupcuuvelblcdnuchuppqpcujernplvmombpdttfjowcujvxknzbwmdedjydxvwykbbamfnsyzcozlixdgoliddoejurusnrcdbqkfdxsoxxzlhgyiprujvvwgqlzredkwahexewlnvqcwfyahjpeiucnhsdhnxtgizgpqphunlgikogmsffexaeftzhblpdxrxgsmeascmqngmwbotycbjmwrngemxpfakrwcdndanouyhnnrygvntrhcuxgvpgjafijlrewfhqrguwhdepwlxvrakyqgstoyruyzohlvvdhvqmzdsnbtlwctetwyrhhktkhhobsojiyuydknvtxmjewvssegrtmshxuvzcbrabntjqulxkjazrsgbpqnrsxqflvbvzywzetrmoydodrrhnhdzlajzvnkrcylkfmsdode".to_string();
        longest_palindrome(s);
    }

    #[test]
    fn test_is_palindrome_with_a_palindrome_digit() {
        let s = "121";
        assert_eq!(true, is_palindrome(s));
    }

    #[test]
    fn test_is_palindrome_with_a_not_palindrome_digit() {
        let s = "-121";
        assert_eq!(false, is_palindrome(s));
    }

    #[test]
    fn test_is_palindrome_with_just_letters() {
        let s = "bb";
        assert_eq!(true, is_palindrome(s));
    }
}
