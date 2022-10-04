use std::ops::{Add, AddAssign, BitAnd, Mul};
use num_traits::{Num, zero};
use crate::Matrix;

//OVERLOADING + OPERATOR
impl<T : Num> Add for Matrix<T> where T : AddAssign + Copy {
    type Output = Matrix<T>;

    fn add(mut self, rhs: Self) -> Self::Output {
        if self.nb_lines == rhs.nb_lines && self.nb_columns == rhs.nb_columns {
            Matrix {
                nb_lines : self.nb_lines,
                nb_columns : self.nb_columns,
                size : self.size,
                data : {
                    let data : Vec<T> = self.into_iter().zip(rhs.into_iter()).map(|(a, b)| a + b).collect();
                    data.into_boxed_slice()
                }
            }
        }
        else if self.nb_columns == rhs.nb_columns && rhs.nb_lines == 1 {
            for i in 0..self.size {
                self.data[i] += rhs.data[i % self.nb_columns];
            }
            self
        }
        else if self.nb_lines == rhs.nb_lines && rhs.nb_columns == 1 {
            for i in 0..self.nb_lines {
                for j in 0..self.nb_columns {
                    self.data[i * self.nb_columns + j] += rhs.data[i];
                }
            }
            self
        }
        else {
            panic!("Can't add two matrix with different shape !!")
        }
    }
}

//OVERLOADING * OPERATOR
impl<T : Num> Mul for Matrix<T> where  T : Mul + AddAssign + Copy {
    type Output = Matrix<T>;
    
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res : Matrix<T> = Matrix::zeros(self.nb_lines, rhs.nb_columns);
        if self.nb_columns == rhs.nb_lines {
            for i in 0..self.nb_lines {
                for j in 0..rhs.nb_columns {
                    let mut sum : T = zero();
                    for k in 0..self.nb_columns {
                        sum += self[i][k] * rhs[k][j]
                    }
                    res[i][j] = sum
                }
            }
            res
        }
        else {
            panic!("Can't multiply to matrix with no compatible shapes")
        }
    }
}

//OVERLOADING & OPERATOR for the Hadamard product
impl<T : Num> BitAnd for Matrix<T> {
    type Output = Matrix<T>;

    fn bitand(self, rhs: Self) -> Self::Output {
        if self.nb_lines == rhs.nb_lines && self.nb_columns == rhs.nb_columns {
            Matrix {
                nb_lines : self.nb_lines,
                nb_columns : self.nb_columns,
                size : self.size,
                data : {
                    let data : Vec<T> = self.into_iter().zip(rhs.into_iter()).map(|(a, b)| a * b).collect();
                    data.into_boxed_slice()
                }
            }
        }
        else {
            panic!("Can't add two matrix with different shape !!")
        }
    }
}



//FUNCTIONALITIES
impl<T : Num> Matrix<T> {
    pub fn map<F>(self, f: F) -> Matrix<T> where F: Fn(T) -> T { //Apply a function to the Matrix
        Matrix {
            nb_lines : self.nb_lines,
            nb_columns : self.nb_columns,
            size : self.size,
            data : {
                let data : Vec<T> = self.into_iter().map(f).collect();
                data.into_boxed_slice()
            }
        }
    }
}