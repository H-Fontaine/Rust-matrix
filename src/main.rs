use matrix::{Concatenate, Matrix};

fn main() {
    let mut matrix1 : Matrix<u32> = Matrix::ones(3,3);
    let mut matrix2 : Matrix<u32> = Matrix::ones(3,3);
    matrix1[0][1] = 7;
    matrix2[1][2] = 5;
    let split = (matrix1 * matrix2).split_lines(3);
    for matrix in split.clone() {
        matrix.display();
        println!(" ");
    }
    let recomposed_matrix = split.concatenate_lines();
    recomposed_matrix.display();
}