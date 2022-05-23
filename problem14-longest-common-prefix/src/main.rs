// https://leetcode.com/problems/longest-common-prefix/
fn main() {
    // let strs_1: Vec<String> = vec![
    //     "flower".to_string(),
    //     "flow".to_string(),
    //     "flight".to_string(),
    // ];
    let strs_2: Vec<String> = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    // let strs_3: Vec<String> = vec![
    //     "reflower".to_string(),
    //     "flow".to_string(),
    //     "flight".to_string(),
    // ];

    let result = longest_common_prefix(strs_2);
    println!("Output :{}", result);
}
// Write a function to find the longest common prefix string amongst an array of strings.
// If there is no common prefix, return an empty string "".
fn longest_common_prefix(strs: Vec<String>) -> String {
    // Get the shortest str
    let shortest_str = get_shortest_word(&strs);
    if shortest_str.len() == 0 {
        return shortest_str;
    }

    let mut new_vector: Vec<String> = Vec::new();
    for str in &strs {
        if str != &shortest_str {
            new_vector.push(str.to_string());
        }
    }

    let mut common_word = String::from("");
    for i in (1..shortest_str.len() + 1).rev() {
        let w = &shortest_str[0..i];
        println!("W is {}", w);
        let rex = test_word(&new_vector, &w.to_string());
        if rex {
            common_word = w.to_string();
            break;
        }
    }
    common_word
}

fn test_word(new_str: &Vec<String>, shortest_str: &String) -> bool {
    for array_str in new_str {
        if !array_str.starts_with(shortest_str) {
            return false;
        }
    }
    true
}

fn get_shortest_word(strs: &Vec<String>) -> String {
    if let Some(mut shortest_str) = strs.get(0) {
        for str in strs {
            if str.len() < shortest_str.len() {
                shortest_str = str;
            }
        }
        return shortest_str.to_owned();
    } else {
        return String::from("");
    }
}
