use matrix::Matrix;

fn main() {
    let mut matrix : Matrix<u32> = Matrix::ones(3,3);
    matrix[0][0] = 3;
    matrix[0][1] = 2;
    matrix.display();
    println!();
    matrix.sum_line().display();
}