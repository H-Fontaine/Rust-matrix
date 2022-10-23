use matrix::Matrix;

fn main() {
    let mut matrix : Matrix<u32> = Matrix::ones(3,3);
    matrix[0][2] = 3;
    matrix[0][1] = 5;
    matrix[2][0] = 2;
    matrix[2][1] = 4;
    matrix.display();
    println!();
    matrix.t().display();
}