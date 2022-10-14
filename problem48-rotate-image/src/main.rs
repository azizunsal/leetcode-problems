use std::ops::Index;

fn main() {
    let mut matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("Original matrix= {:?}", matrix);
    rotate_in_place(&mut matrix);
    println!("Rotated matrix= {:?}", matrix);
}

fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let mut new_matrix: Vec<Vec<i32>> = Vec::new();
    for i in 0..matrix.len() {
        print!("[ ");
        let mut col: Vec<i32> = Vec::new();
        for j in (0..matrix.len()).rev() {
            let orig_val = matrix[j][i];

            // print!("[{j}][{i}]={orig_val}, ");

            let old_index = (j, i);
            print!("{:?}={}, ", old_index, orig_val);
            // new_matrix[j][i] = orig_val;
            col.push(orig_val);
        }
        println!("]");
        new_matrix.push(col);
    }

    println!("new matrix= {:?}", new_matrix);
}

fn rotate_in_place(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    for i in 0..n {
        for j in 0..n {
            let old_index = find_old_index(i, j, n).unwrap();
            println!("New index= {:?} / Old index= {:?}", (i, j), old_index);
        }
        println!("]");
    }
}

fn find_old_index(i: usize, j: usize, n: usize) -> Option<(usize, usize)> {
    // println!("Trying to find old index for index= {:?} for '{n}x{n} array.'", new_index);
    let mut new_i:usize=i;
    let mut new_j = j + (n - 1);
    println!("  -new initial values  / new_i= {new_i}, new_j= {new_j}");

    if new_j > (n - 1) {
        new_j = n - 1;
        new_i = i + 1;
    } else {
        new_i = i;
    }

    Some((new_i, new_j))
}
