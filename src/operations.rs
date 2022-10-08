use std::ops::{Add, AddAssign, BitAnd, Div, Mul, Neg};
use num_traits::{Num, zero};
use crate::Matrix;

//OVERLOADING + OPERATOR
impl<T : Num> Add<Matrix<T>> for Matrix<T> where T : AddAssign + Copy {
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

//OVERLOADING += OPERATOR
impl<T : Num> AddAssign<Matrix<T>> for Matrix<T> where T : AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        if self.nb_lines == rhs.nb_lines && self.nb_columns == rhs.nb_columns {
            for (right, left) in self.into_iter().zip(rhs.into_iter()) {
                *right += left;
            }
        }
    }
}


//OVERLOADING NEG (-) OPERATOR
impl<T : Num> Neg for Matrix<T> where T : Neg<Output = T> {
    type Output = Matrix<T>;
    fn neg(self) -> Self::Output {
        self.map(|a| -a)
    }
}

//OVERLOADING * OPERATOR FOR MATRIX
impl<T : Num> Mul<Matrix<T>> for Matrix<T> where  T : Mul + AddAssign + Copy {
    type Output = Matrix<T>;
    
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res : Matrix<T> = Matrix::zeros(self.nb_lines, rhs.nb_columns);
        if self.nb_columns == rhs.nb_lines {
            for i in 0..self.nb_lines {
                for j in 0..rhs.nb_columns {
                    let mut sum : T = zero();
                    for k in 0..self.nb_columns {
                        sum += self[i][k] * rhs[k][j];
                    }
                    res[i][j] = sum;
                }
            }
            res
        }
        else {
            panic!("Can't multiply to matrix with no compatible shapes")
        }
    }
}

//OVERLOADING * OPERATOR FOR SCALARS
impl<T : Num> Mul<T> for Matrix<T> where T : Mul<T, Output = T> + Copy {
    type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        self.map(|a| a * rhs)
    }
}

//OVERLOADING / OPERATOR FOR SCALARS
impl<T : Num> Div<T> for Matrix<T> where T : Div<T, Output = T> + Copy {
    type Output = Matrix<T>;
    fn div(self, rhs: T) -> Self::Output {
        self.map(|a| a / rhs)
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

impl<T : Num> Matrix<T> where T : AddAssign + Clone {
    pub fn sum_line(self) -> Matrix<T> {
        let nb_columns = self.nb_columns;
        Matrix {
            nb_columns,
            nb_lines: 1,
            size: nb_columns,
            data: {
                let mut data: Vec<T> = vec![zero(); self.nb_columns];
                for (i, value) in (0..self.size).zip(self.into_iter()) {
                    data[i % nb_columns] += value;
                }
                data.into_boxed_slice()
            }
        }
    }

    pub fn sum_col(self) -> Matrix<T> {
        let nb_lines = self.nb_lines;
        Matrix {
            nb_lines,
            nb_columns : 1,
            size: nb_lines,
            data: {
                let mut data: Vec<T> = vec![zero(); self.nb_columns];
                for (i, value) in (0..self.size).zip(self.into_iter()) {
                    data[i / nb_lines] += value;
                }
                data.into_boxed_slice()
            }
        }
    }
}





