fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];

    let median: f64 = find_median_sorted_arrays(nums1, nums2);
    println!("The median is {}", median);
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut single_vec: Vec<i32> = Vec::new();
    // Merge two lists
    for i in nums1 {
        single_vec.push(i);
    }
    for j in nums2 {
        single_vec.push(j);
    }
    single_vec.sort();

    // Sort the list
    let n = single_vec.len();

    if n % 2 == 0 {
        let a = n / 2;
        let b = n / 2 + 1;
        return (single_vec[a - 1] as f64 + single_vec[b - 1] as f64) / 2.0;
    } else {
        let index = (n + 1) / 2;
        return single_vec[index - 1] as f64;
    }
}

#[test]
fn test_with_two_array_odd() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    assert_eq!(2.0, find_median_sorted_arrays(nums1, nums2));
}

#[test]
fn test_with_two_array_event() {
    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    assert_eq!(2.5, find_median_sorted_arrays(nums1, nums2));
}

#[test]
fn test_with_two_array_zero() {
    let nums1 = vec![0, 0];
    let nums2 = vec![0, 0];
    assert_eq!(0.0, find_median_sorted_arrays(nums1, nums2));
}
