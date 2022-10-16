use std::time::Instant;

// #[derive(Debug, Clone)]
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { val, next }
    }
}

fn main() {
    let start = Instant::now();
    let l1: Option<Box<ListNode>> = create_list_from_value_array(&[2, 4, 3]);
    let l2: Option<Box<ListNode>> = create_list_from_value_array(&[5, 6, 4]);
    let l1: Option<Box<ListNode>> = create_list_from_value_array(&[9, 9, 9, 9, 9, 9, 9]);
    let l2: Option<Box<ListNode>> = create_list_from_value_array(&[9, 9, 9, 9]);

    let result: Option<Box<ListNode>> = add_two_numbers(l1, l2);
    println!("result= {:?}. time= {:?}", result, start.elapsed());
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let number_arr_1 = create_array_from_list(l1);
    println!("number_arr_1= {:?}", number_arr_1);
    let number_arr_2 = create_array_from_list(l2);
    println!("number_arr_2= {:?}", number_arr_2);

    let num1 = create_number_from_digit_array_in_rev_order(&number_arr_1);
    let num2 = create_number_from_digit_array_in_rev_order(&number_arr_2);

    let sum_of_numbers = num1 + num2;
    println!("num1= {}, num2= {}, sum= {}", num1, num2, sum_of_numbers);

    let arr: Vec<i32> = create_array_from_number(sum_of_numbers);

    println!("sum_of_numbers to arr= {:?}", arr);


    create_list_from_value_array(arr.as_slice())
}

fn create_list_from_value_array(values: &[i32]) -> Option<Box<ListNode>> {
    let mut node: Option<Box<ListNode>> = Some(Box::new(ListNode::new(*values.last().unwrap(), None)));
    for i in (0..values.len() - 1).rev() {
        let val = values[i];
        let current_node = Some(Box::new(ListNode::new(val, node.clone())));
        node = current_node;
    }
    node
}

fn create_array_from_list(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let mut node = list;
    res.push((node.as_ref()).unwrap().val);

    while node.is_some() {
        let current_node = node.unwrap().next;
        node = current_node;
        if node.is_none() {
            break;
        }
        res.push(node.as_ref().unwrap().val);
    }

    res
}

fn create_number_from_digit_array_in_rev_order(arr: &[i32]) -> i32 {
    let mut number_str: String = String::new();
    for i in (0..arr.len()).rev() {
        let digit_str: String = arr[i].to_string();
        number_str.push(digit_str.chars().last().unwrap());
    }
    number_str.parse::<i32>().unwrap()
}

fn create_array_from_number(number: i32) -> Vec<i32> {
    let mut digits: Vec<char> = number.to_string().chars().collect();
    let mut nums: Vec<i32> = vec![-1; digits.len()];

    let mut cnt: usize = digits.len() - 1;
    for d in digits.iter_mut().rev() {
        // nums.push(d.to_string().parse::<i32>().unwrap());
        nums[cnt] = d.to_string().parse::<i32>().unwrap();
        if cnt == 0 { break; }
        cnt -= 1;
    }

    nums.reverse();
    let sli: &[i32] = nums.as_slice().try_into().unwrap();
    sli.to_owned()
}


#[cfg(test)]
mod test {
    use crate::{add_two_numbers, create_list_from_value_array, ListNode};

    #[test]
    fn test_1() {
        let l1: Option<Box<ListNode>> = create_list_from_value_array(&[2, 4, 3]);
        let l2: Option<Box<ListNode>> = create_list_from_value_array(&[5, 6, 4]);
        assert_eq!(create_list_from_value_array(&[7, 0, 8]), add_two_numbers(l1, l2));
    }

    #[test]
    fn test_2() {
        let l1: Option<Box<ListNode>> = create_list_from_value_array(&[9, 9, 9, 9, 9, 9, 9]);
        let l2: Option<Box<ListNode>> = create_list_from_value_array(&[9, 9, 9, 9]);
        assert_eq!(create_list_from_value_array(&[8, 9, 9, 9, 0, 0, 0, 1]), add_two_numbers(l1, l2));
    }

    #[test]
    fn test_3() {
        let l1: Option<Box<ListNode>> = create_list_from_value_array(&[0]);
        let l2: Option<Box<ListNode>> = create_list_from_value_array(&[0]);
        assert_eq!(create_list_from_value_array(&[0]), add_two_numbers(l1, l2));
    }
}