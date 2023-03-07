#[cfg(test)]

use crate::Matrix;

#[test]
fn multiplication() {
    let matrix1 = Matrix {
        nb_lines : 3,
        nb_columns : 2,
        data : vec![2,   5,
                    3,  -7,
                    5,   0]
    };

    let matrix2 = Matrix {
        nb_lines : 2,
        nb_columns : 5,
        data : vec![5,   4,  7,  0, -1,
                    3, -10,  5,  0,  4]
    };

    let result = Matrix {
        nb_lines : 3,
        nb_columns : 5,
        data : vec![25, -42,  39, 0,  18,
                    -6,  82, -14, 0, -31,
                    25,  20,  35, 0,  -5]

    };

    assert_eq!(&matrix1 * &matrix2, result, "Testing multiplication with &Matrix<u32> and &Matrix<u32>");
    assert_eq!(matrix1.clone() * &matrix2, result, "Testing multiplication with Matrix<u32> and &Matrix<u32>");
    assert_eq!(&matrix1 * matrix2.clone(), result, "Testing multiplication with &Matrix<u32> and Matrix<u32>");
    assert_eq!(matrix1 * matrix2, result, "Testing multiplication with Matrix<u32> and Matrix<u32>");
}

#[test]
fn addition() {
    let matrix1 = Matrix {
        nb_lines : 3,
        nb_columns : 2,
        data : vec![2,   5,
                    3,  -7,
                    5,   0]
    };

    let matrix2 = Matrix {
        nb_lines : 3,
        nb_columns : 2,
        data : vec![ 3,   5,
                    -1,  -6,
                     5,   2]
    };

    let result = Matrix {
        nb_lines : 3,
        nb_columns : 2,
        data : vec![ 5,  10,
                     2, -13,
                    10,   2]
    };

    assert_eq!(&matrix1 + &matrix2, result, "Testing addition with &Matrix<u32> and &Matrix<u32>");
    assert_eq!(&matrix1 + matrix2.clone(), result, "Testing addition with &Matrix<u32> and &Matrix<u32>");
    assert_eq!(matrix1.clone() + &matrix2, result, "Testing addition with &Matrix<u32> and Matrix<u32>");
    assert_eq!(matrix1 + matrix2, result, "Testing addition with Matrix<u32> and Matrix<u32>");
}

#[test]
fn addition_lines() {
    let matrix = Matrix {
        nb_lines : 3,
        nb_columns : 2,
        data : vec![ 3,   5,
                    -1,  -6,
                     5,   2]
    };

    let line = Matrix {
        nb_lines : 1,
        nb_columns : 2,
        data : vec![-1, 2]
    };

    let result = Matrix {
        nb_lines : 3,
        nb_columns : 2,
        data : vec![ 2,   7,
                    -2,  -4,
                     4,   4]
    };

    assert_eq!(matrix.add_to_lines(line), result, "Testing addition on lines of Matrix<u32>");
}


























