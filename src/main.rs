use matrix::Matrix;

fn main() {
    let mut matrix1 : Matrix<u32> = Matrix::ones(3,3);
    let mut matrix2 : Matrix<u32> = Matrix::ones(3,3);
    matrix1[0][1] = 7;
    matrix2[1][2] = 5;
    (matrix1 * matrix2).display();
    //matrix.t().display();
}