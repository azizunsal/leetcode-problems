fn main() {
    // let mut vec: Vec<i32> = vec![1, 0, 2, 3, 7, 4, 5, 9];
    // let mut vec: Vec<i32> = vec![1, 8, 4, 3, 9];
    let mut vec: Vec<i32> = vec![1, 0, 2, 3, 0, 4, 5, 0];
    duplicate_zeros(&mut vec);
    println!("result= {:?}", vec);
}

fn duplicate_zeros(arr: &mut Vec<i32>) {
    // println!("started= {:?}", arr);
    let mut s = 0;
    let e = arr.len() - 1;

    while s < e {
        if arr[s] == 0 {
            shift_array(arr, s);
            s += 1;
        }
        s += 1;
    }
}

fn shift_array(arr: &mut Vec<i32>, from_index: usize) {
    // println!("array= {:?}, index= {}", arr, from_index);
    let e = arr.len() - 1;
    for i in from_index..e {
        let tmp = arr[i];
        if i != from_index {
            arr[i] = arr[e];
        }
        arr[e] = tmp;
    }
    // println!("array shifted\n{:?}", arr);
}

#[cfg(test)]
mod test {
    use crate::duplicate_zeros;

    #[test]
    fn test_1() {
        let mut vec: Vec<i32> = vec![1, 2, 3];
        duplicate_zeros(&mut vec);
        assert_eq!(vec![1, 2, 3], vec);
    }

    #[test]
    fn test_2() {
        let mut vec: Vec<i32> = vec![1, 0, 2, 3, 0, 4, 5, 0];
        duplicate_zeros(&mut vec);
        assert_eq!(vec![1, 0, 0, 2, 3, 0, 0, 4], vec);
    }
}
