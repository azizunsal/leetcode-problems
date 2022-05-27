use std::time::{Duration, Instant};

fn main() {
    let s: String = String::from("PAYPALISHIRING");
    let num_rows: i32 = 4;

    println!("{} will be formatted with ZigZag formation in {} rows.", s, num_rows);
    let st = Instant::now();
    let result = convert(s, num_rows);
    let elapsed_time: Duration = st.elapsed();
    println!("Result is {}. Elapsed time {:?}", result, elapsed_time);
}

#[allow(dead_code)]
fn print_array(arr: &[[char; 100]; 100]) {
    for (p, el) in arr.iter().enumerate() {
        println!("  >>row={} / col {:?}", p, el);
    }
}

fn convert(s: String, num_rows: i32) -> String {
    let s_len = s.len() as i32;

    if s_len == 1 || num_rows == 1 {
        return s;
    }

    let mut used_letter_count: i32 = 1;
    let mut row_index = 0;
    let mut column_index: i32 = 0;
    let mut result_str = String::new();
    let mut zigzag_table = [['-'; 100]; 100];

    while used_letter_count <= s_len {
        // println!(
        //     "Row={}, Col={} Used letter count '{}'. Mod?{}",
        //     row_index,
        //     column_index,
        //     used_letter_count,
        //     column_index % 3
        // );

        if column_index == 0 || column_index % (num_rows - 1) == 0 {
            // println!("******MAIN COL / used_letter_count ={}", used_letter_count);
            row_index += num_rows as usize - 2;

            for i in 0..(num_rows as usize) {
                let ch = s.chars().nth(used_letter_count as usize - 1 + i);
                if ch.is_none() {
                    // println!("BREAAAAAAAAAAAAAAK!!!!");
                    break;
                }
                // println!("i={}, row={} =========Ch is {}", i, row_index, ch.unwrap());
                zigzag_table[i][column_index as usize] = ch.unwrap();
            }
            // println!("?????????? - used_letter_count {}", used_letter_count);
            used_letter_count += num_rows;
        } else {
            let ch = s.chars().nth(used_letter_count as usize - 1).unwrap();
            zigzag_table[row_index][column_index as usize] = ch;
            used_letter_count += 1;

            if row_index != 0 {
                row_index -= 1;
                // println!("DECREASE ROW IDX");
            }
        }
        column_index += 1;
        // print_array(&zigzag_table);
    }

    // print_array(&zigzag_table);

    for (_, el) in zigzag_table.iter().enumerate() {
        for (_, s_el) in el.iter().enumerate() {
            if s_el != &'-' {
                // println!("[{}][{}] = {:?}", f_idx, s_idx, s_el);
                result_str.push(*s_el);
            }
        }
    }

    // let arr: String = zigzag_table.into_iter().flatten().filter(|a_chr| *a_chr != '-').collect();
    // arr

    result_str
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_with_4_rows() {
        let s: String = String::from("PAYPALISHIRING");
        let result: String = String::from("PINALSIGYAHRPI");
        let num_rows: i32 = 4;
        assert_eq!(result, convert(s, num_rows));
    }

    #[test]
    fn test_with_3_rows() {
        let s: String = String::from("PAYPALISHIRING");
        let result: String = String::from("PAHNAPLSIIGYIR");
        let num_rows: i32 = 3;
        assert_eq!(result, convert(s, num_rows));
    }

    #[test]
    fn test_with_single_char() {
        let s: String = String::from("A");
        let result: String = String::from("A");
        let num_rows: i32 = 1;
        assert_eq!(result, convert(s, num_rows));
    }

    #[test]
    fn test_with_single_row() {
        let s: String = String::from("PAYPALISHIRING");
        let num_rows: i32 = 1;
        assert_eq!(s, convert(s.clone(), num_rows));
    }
}
