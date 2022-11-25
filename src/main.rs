use matrix::Matrix;

fn main() {
    let mut matrix : Matrix<u32> = Matrix::ones(3,3);
    matrix.display();
    matrix.t().display();
}