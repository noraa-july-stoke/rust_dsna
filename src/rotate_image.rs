fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();

    for i in 0..n {
        let row: Vec<i32> = matrix.iter().map(|r| r[i]).collect();
        matrix[i] = row.into_iter().rev().collect();
    }

}


pub fn main() {
    let mut matrix = vec![
        vec![1,2,3],
        vec![4,5,6],
        vec![7,8,9]
    ];

    rotate(&mut matrix);

    for row in &matrix {
        println!("{:?}", row)
    }
}
