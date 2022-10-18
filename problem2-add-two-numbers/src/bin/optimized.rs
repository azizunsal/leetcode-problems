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
    let l1: Option<Box<ListNode>> = create_list_from_value_array(&[2, 4, 3]);
    let l2: Option<Box<ListNode>> = create_list_from_value_array(&[9, 8, 1, 7, 4]);
    // let l1: Option<Box<ListNode>> = create_list_from_value_array(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    // let l2: Option<Box<ListNode>> = create_list_from_value_array(&[5, 6, 4]);
    let result: Option<Box<ListNode>> = add_two_numbers(l1, l2);
    println!("result= {:?}. time= {:?}", result, start.elapsed());
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let carry = 0;
    let mut arr: Vec<i32> = Vec::new();
    rec(&mut arr, carry, l1, l2);
    create_list_from_value_array(&arr)
}

fn rec(arr: &mut Vec<i32>, mut carry: i32, l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) {
    let mut current_l1 = l1;
    let mut current_l2 = l2;

    let mut total = 0;
    if let Some(a) = current_l1 {
        total += a.val;
        current_l1 = a.next;
    }

    if let Some(b) = current_l2 {
        total += b.val;
        current_l2 = b.next;
    }
    total += carry;

    // calc next carry
    carry = total / 10;
    total = total - carry * 10;
    arr.push(total);

    if current_l1.is_none() && current_l2.is_none() {
        // println!("Break!");
        if carry > 0 {
            // println!("  add an extra digit!");
            arr.push(carry);
        }
    } else {
        rec(arr, carry, current_l1, current_l2)
    }
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

// Test utility method.
#[warn(dead_code)]
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
        assert_eq!(vec![1, 3, 5, 7, 4], create_array_from_list(add_two_numbers(l1, l2)));
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