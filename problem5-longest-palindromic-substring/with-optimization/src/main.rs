fn main() {
    // let s = "babad".to_string(); // bab
    // let s = "cbbd".to_string(); // bb
    //  let s = "baba".to_string(); // ?
    // let s = "ac".to_string(); // a
    let s="jhgtrclvzumufurdemsogfkpzcwgyepdwucnxrsubrxadnenhvjyglxnhowncsubvdtftccomjufwhjupcuuvelblcdnuchuppqpcujernplvmombpdttfjowcujvxknzbwmdedjydxvwykbbamfnsyzcozlixdgoliddoejurusnrcdbqkfdxsoxxzlhgyiprujvvwgqlzredkwahexewlnvqcwfyahjpeiucnhsdhnxtgizgpqphunlgikogmsffexaeftzhblpdxrxgsmeascmqngmwbotycbjmwrngemxpfakrwcdndanouyhnnrygvntrhcuxgvpgjafijlrewfhqrguwhdepwlxvrakyqgstoyruyzohlvvdhvqmzdsnbtlwctetwyrhhktkhhobsojiyuydknvtxmjewvssegrtmshxuvzcbrabntjqulxkjazrsgbpqnrsxqflvbvzywzetrmoydodrrhnhdzlajzvnkrcylkfmsdode".to_string();
    // let s = "jhgtrclvzumufurdemsogfkpz".to_string();
    // let s = "racecar".to_string(); // racecar
    let str = longest_palindrome(s);
    println!("Longest palindrome substring is '{}'.", str);
}

fn longest_palindrome(s: String) -> String {
    // println!("Checking the string '{}'", s);
    let (mut longest, mut longest_length) = (String::new(), 0);
    for i in 0..s.len() {
        (longest, longest_length) = check(i as i32, i as i32, &s, longest.clone(), longest_length);
        (longest, longest_length) = check(i as i32, i as i32 + 1, &s, longest.clone(), longest_length);
    }

    longest.to_string()
}

fn check(
    mut left: i32,
    mut right: i32,
    s: &String,
    mut longest: String,
    mut longest_length: i32,
) -> (String, i32) {
    while left >= 0
        && right < s.len() as i32
        && &s[left as usize..left as usize + 1] == &s[right as usize..right as usize + 1]
    {
        println!(
            " -- inside while for l={}, r={}, new length={}",
            left,
            right,
            (right - left + 1)
        );
        if (right - left + 1) > longest_length {
            longest_length = right - left + 1;
            longest = s[left as usize..right as usize + 1].to_string();
            println!("    Longest changed to {}", &longest);
        }
        left -= 1;
        right += 1;
    }
    (longest.to_string(), longest_length)
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
    fn test_non_recurring() {
        let s = "ac".to_string();
        assert_eq!("a", longest_palindrome(s));
    }

    #[test]
    fn test_with_a_loooooong_string() {
        let s="jhgtrclvzumufurdemsogfkpzcwgyepdwucnxrsubrxadnenhvjyglxnhowncsubvdtftccomjufwhjupcuuvelblcdnuchuppqpcujernplvmombpdttfjowcujvxknzbwmdedjydxvwykbbamfnsyzcozlixdgoliddoejurusnrcdbqkfdxsoxxzlhgyiprujvvwgqlzredkwahexewlnvqcwfyahjpeiucnhsdhnxtgizgpqphunlgikogmsffexaeftzhblpdxrxgsmeascmqngmwbotycbjmwrngemxpfakrwcdndanouyhnnrygvntrhcuxgvpgjafijlrewfhqrguwhdepwlxvrakyqgstoyruyzohlvvdhvqmzdsnbtlwctetwyrhhktkhhobsojiyuydknvtxmjewvssegrtmshxuvzcbrabntjqulxkjazrsgbpqnrsxqflvbvzywzetrmoydodrrhnhdzlajzvnkrcylkfmsdode".to_string();
        longest_palindrome(s);
    }
}
