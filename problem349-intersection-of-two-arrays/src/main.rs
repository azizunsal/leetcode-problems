use std::{cmp::min, time::Instant};

fn main() {
    let s = Instant::now();
    let _nums1 = vec![1, 2, 2, 1];
    let _nums2 = vec![2, 2];

    let _nums1 = vec![4, 9, 5];
    let _nums2 = vec![9, 4, 9, 8, 4];

    let r = intersection(_nums1, _nums2);
    println!("Intersection array is {:?}. Elapse time {:?}", r, s.elapsed());
}

fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut intersection: Vec<i32> = Vec::with_capacity(min(nums1.len(), nums2.len()));
    for i in nums1 {
        if nums2.contains(&i) {
            if intersection.contains(&i) {
                continue;
            }

            intersection.push(i);
        }
    }

    intersection
}

#[cfg(test)]
mod test {
    use crate::intersection;

    #[test]
    fn test_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(vec![2], intersection(nums1, nums2));
    }
    #[test]
    fn test_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        assert_eq!(vec![4, 9], intersection(nums1, nums2));
    }
    #[test]
    fn test_3() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![1, 2];
        assert_eq!(vec![1, 2, 1], intersection(nums1, nums2));
    }
}
