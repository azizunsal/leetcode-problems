fn main() {
    let number = 10;
    println!("Is {} a palindrome ? {}", number, is_palindrome(number));
}

pub fn is_palindrome(x: i32) -> bool {
    let x_str = x.to_string();
    let reverse_str = x_str.chars().rev().collect::<String>();
    return x_str == reverse_str;
}
