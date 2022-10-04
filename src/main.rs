use matrix::Matrix;

fn main() {
    let mut matrix : Matrix<u32> = Matrix::zeros(3,3);
    matrix[0][0] = 3;
    matrix.display();
}