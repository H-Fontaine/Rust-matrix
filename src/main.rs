use matrix::{Concatenate, Matrix};

fn main() {
    let mut matrix1 : Matrix<u32> = Matrix::ones(3,3);
    let mut matrix2 : Matrix<u32> = Matrix::ones(3,3);
    matrix1[0][1] = 7;
    matrix2[1][2] = 5;
    let split = (&matrix1 * &matrix2).split_lines(2);
    let mut split_iter = split.into_iter();
    let matrix1 = split_iter.next().unwrap();
    let matrix2 = split_iter.next().unwrap();
    let recomposed_matrix = matrix1.concatenate_lines(matrix2);
    recomposed_matrix.display();
}