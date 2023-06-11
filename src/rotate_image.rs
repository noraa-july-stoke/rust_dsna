fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();

    for i in 0..n {
        for j in i..n {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }

    for i in 0..n {
        matrix[i].reverse();
    }
}


pub fn main() {
    let mut matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    rotate(&mut matrix);

    println!("{:?}", matrix);
}
