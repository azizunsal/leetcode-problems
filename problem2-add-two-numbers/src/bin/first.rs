use std::time::Instant;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn main() {
    let start = Instant::now();
    // let l1: Option<Box<ListNode>> = create_list_from_value_array(&[9, 9, 9, 9, 9, 9, 9]);
    // let l2: Option<Box<ListNode>> = create_list_from_value_array(&[9, 9, 9, 9]);
    let l1: Option<Box<ListNode>> = create_list_from_value_array(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    let l2: Option<Box<ListNode>> = create_list_from_value_array(&[5, 6, 4]);
    let result: Option<Box<ListNode>> = add_two_numbers(l1, l2);
    println!("result= {:?}. time= {:?}", result, start.elapsed());
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut number_arr_1 = create_array_from_list(l1);
    let mut number_arr_2 = create_array_from_list(l2);
    let sum: Vec<i32> = sum_two_digit_arrays(&mut number_arr_1, &mut number_arr_2);
    create_list_from_value_array(sum.as_slice())
}

fn create_list_from_value_array(values: &[i32]) -> Option<Box<ListNode>> {
    let mut node: Option<Box<ListNode>> = Some(Box::new(ListNode::new(*values.last().unwrap())));
    for i in (0..values.len() - 1).rev() {
        let val = values[i];
        let mut current_node = Some(Box::new(ListNode::new(val)));
        current_node.as_mut().map(|a| a.next = node.clone());
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

fn sum_two_digit_arrays(arr1: &mut Vec<i32>, arr2: &mut Vec<i32>) -> Vec<i32> {
    let arr1_len = arr1.len();
    let arr2_len = arr2.len();

    if arr1_len > arr2_len {
        fix_array_length(arr2, arr1_len);
    } else if arr2_len > arr1_len {
        fix_array_length(arr1, arr2_len);
    }

    arr1.reverse();
    arr2.reverse();
    let mut sum_arr: Vec<i32> = Vec::with_capacity(arr1_len);

    let mut carry = 0;
    for i in (0..arr1.len()).rev() {
        let digit1 = arr1[i];
        let digit2 = arr2[i];
        let mut sum = digit1 + digit2 + carry;

        if sum >= 10 {
            carry = sum / 10;
            sum = sum - 10;
        } else { carry = 0 }

        println!("{} + {} = {}, carry= {}", digit1, digit2, sum, carry);
        sum_arr.push(sum);
    }

    if carry > 0 {
        println!("The carry (={}) must be ZERO! Adding 1 extra digit to the number.", carry);
        sum_arr.push(carry);
    }
    println!("Resulting array= {:?}", sum_arr);
    sum_arr
}

fn fix_array_length(arr: &mut Vec<i32>, limit: usize) {
    println!("Fixing array= {:?}, limit= {}", arr, limit);
    if arr.len() < limit {
        for _i in 0..(limit - arr.len()) {
            arr.push(0);
        }
    }
    println!("  --> fixed array= {:?}", arr);
}

#[cfg(test)]
mod test {
    use crate::{add_two_numbers, create_list_from_value_array, create_array_from_list, ListNode};

    #[test]
    fn test_1() {
        let l1: Option<Box<ListNode>> = create_list_from_value_array(&[2, 4, 3]);
        let l2: Option<Box<ListNode>> = create_list_from_value_array(&[5, 6, 4]);
        assert_eq!(vec![7, 0, 8], create_array_from_list(add_two_numbers(l1, l2)));
    }

    #[test]
    fn test_2() {
        let l1: Option<Box<ListNode>> = create_list_from_value_array(&[9, 9, 9, 9, 9, 9, 9]);
        let l2: Option<Box<ListNode>> = create_list_from_value_array(&[9, 9, 9, 9]);
        assert_eq!(vec![8, 9, 9, 9, 0, 0, 0, 1], create_array_from_list(add_two_numbers(l1, l2)));
    }

    #[test]
    fn test_3() {
        let l1: Option<Box<ListNode>> = create_list_from_value_array(&[0]);
        let l2: Option<Box<ListNode>> = create_list_from_value_array(&[0]);
        assert_eq!(vec![0], create_array_from_list(add_two_numbers(l1, l2)));
    }

    #[test]
    fn test_4() {
        let l1: Option<Box<ListNode>> = create_list_from_value_array(&[9]);
        let l2: Option<Box<ListNode>> = create_list_from_value_array(&[1, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], create_array_from_list(add_two_numbers(l1, l2)));
    }

    #[test]
    fn test_5() {
        let l1: Option<Box<ListNode>> = create_list_from_value_array(&[2, 4, 3]);
        let l2: Option<Box<ListNode>> = create_list_from_value_array(&[9, 8, 1, 7, 4]);
        assert_eq!(vec![1,3,5,7,4], create_array_from_list(add_two_numbers(l1, l2)));
    }

    #[test]
    fn test_6() {
        let l1: Option<Box<ListNode>> = create_list_from_value_array(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        let l2: Option<Box<ListNode>> = create_list_from_value_array(&[5, 6, 4]);
        let mut res: Vec<i32> = vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 6, 6];
        res.reverse();
        assert_eq!(res, create_array_from_list(add_two_numbers(l1, l2)));
    }
}