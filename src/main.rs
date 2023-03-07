use matrix::{Matrix};

fn main() {
    let mut matrix1 : Matrix<u32> = Matrix::ones(3,3);
    let mut matrix2 : Matrix<u32> = Matrix::ones(3,3);
    let mut line = Matrix::ones(1, 3);
    line[0][1] = 2;
    matrix1.add_to_lines(&line);
    matrix1.display();
    matrix1[0][1] = 7;
    matrix2[1][2] = 5;
    let split = (&matrix1 + matrix2).split_lines(2);
    let mut split_iter = split.into_iter();
    let matrix1 = split_iter.next().unwrap();
    let matrix2 = split_iter.next().unwrap();
    let recomposed_matrix = matrix1.concatenate_lines(matrix2);
    recomposed_matrix.display();

    let test : String = vec!['a';3].into_iter().collect();
    println!("{}", &test);
    println!("{}", test);
}