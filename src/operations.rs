use std::borrow::Borrow;
use std::ops::{Add, AddAssign, BitAnd, Div, Mul, Neg};
use num_traits::Zero;
use crate::Matrix;



//OVERLOADING + OPERATOR FOR MATRIX
static ADDING_ERROR : &str = "Can't add two matrices with different shape !!";

impl<T> Add<Matrix<T>> for Matrix<T> where T : Add<Output = T> {
    type Output = Matrix<T>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape(), "{}", ADDING_ERROR);
        Matrix {
            nb_lines : self.nb_lines,
            nb_columns : self.nb_columns,
            data : {
                let data : Vec<T> = self.into_iter().zip(rhs.into_iter()).map(|(a, b)| a + b).collect();
                data
            }
        }
    }
}
impl<'a, T> Add<&'a Matrix<T>> for &'a Matrix<T> where T : Add<Output = T> + Copy {
    type Output = Matrix<T>;

    fn add(self, rhs: &'a Matrix<T>) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape(), "{}", ADDING_ERROR);
        Matrix {
            nb_lines : self.nb_lines,
            nb_columns : self.nb_columns,
            data : {
                let data : Vec<T> = self.into_iter().zip(rhs.into_iter()).map(|(a, b)| *a + *b).collect();
                data
            }
        }
    }
}
impl<T> Add<Matrix<T>> for &Matrix<T> where T : Add<Output = T> + Copy {
    type Output = Matrix<T>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape(), "{}", ADDING_ERROR);
        Matrix {
            nb_lines : self.nb_lines,
            nb_columns : self.nb_columns,
            data : {
                let data : Vec<T> = self.into_iter().zip(rhs.into_iter()).map(|(a, b)| *a + b).collect();
                data
            }
        }
    }
}
impl<T> Add<&Matrix<T>> for Matrix<T> where T : Add<Output = T> + Copy {
    type Output = Matrix<T>;

    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape(), "{}", ADDING_ERROR);
        Matrix {
            nb_lines: self.nb_lines,
            nb_columns: self.nb_columns,
            data: {
                let data: Vec<T> = self.into_iter().zip(rhs.into_iter()).map(|(a, b)| a + *b).collect();
                data
            }
        }
    }
}










//OVERLOADING += OPERATOR
impl<T> AddAssign<Matrix<T>> for Matrix<T> where T : AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        if self.nb_lines == rhs.nb_lines && self.nb_columns == rhs.nb_columns {
            for (right, left) in self.into_iter().zip(rhs.into_iter()) {
                *right += left;
            }
        }
    }
}









//ADDING ALONG COLUMNS OR LINES
impl<T> Matrix<T> where T : AddAssign + Copy {
    pub fn add_to_lines<M>(mut self, rhs : M) -> Matrix<T> where M : Borrow<Matrix<T>> {
        let matrix_line = rhs.borrow();
        assert_eq!(matrix_line.nb_lines, 1, "The matrix to add must have exactly one line");
        assert_eq!(self.nb_columns, matrix_line.nb_columns, "Both matrix must have the same number of columns");
        for i in 0..self.size() {
            self.data[i] += matrix_line.data[i % self.nb_columns];
        }
        self
    }

    pub fn add_to_columns<M>(mut self, rhs : M) -> Matrix<T> where M : Borrow<Matrix<T>> {
        let matrix_column = rhs.borrow();
        assert_eq!(matrix_column.nb_columns, 1, "The matrix to add must have exactly one column");
        assert_eq!(self.nb_lines, matrix_column.nb_lines, "Both matrix must have the same number of lines");
        for i in 0..self.nb_lines {
            for j in 0..self.nb_columns {
                self.data[i * self.nb_columns + j] += matrix_column.data[i];
            }
        }
        self
    }
}






//OVERLOADING NEG (-) OPERATOR
impl<T> Neg for Matrix<T> where T : Neg<Output = T> {
    type Output = Matrix<T>;
    fn neg(self) -> Self::Output {
        self.map(|a| -a)
    }
}








//OVERLOADING * OPERATOR FOR MATRIX
static MULTIPLICATION_ERROR : &str = "Can't multiply matrices with incompatible shapes";

impl<T> Mul<Matrix<T>> for Matrix<T> where T : Mul<Output = T> + AddAssign + Copy {
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        assert_eq!(self.nb_columns, rhs.nb_lines, "{}", MULTIPLICATION_ERROR);
        let size = self.nb_lines * rhs.nb_columns;
        let mut res = Matrix {
            nb_lines : self.nb_lines,
            nb_columns : rhs.nb_columns,
            data : {
                let mut temp = Vec::with_capacity(size);
                unsafe {temp.set_len(size)}
                temp
            }
        };
        for i in 0..self.nb_lines {
            for j in 0..rhs.nb_columns {
                res [i][j] = self[i][0] * rhs[0][j];
                for k in 1..self.nb_columns {
                    res[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        res
    }
}
impl<'a, T> Mul<&'a Matrix<T>> for &'a Matrix<T> where T : Mul<Output = T> + AddAssign + Copy {
    type Output = Matrix<T>;

    fn mul(self, rhs: &'a Matrix<T>) -> Self::Output {
        assert_eq!(self.nb_columns, rhs.nb_lines, "{}", MULTIPLICATION_ERROR);
        let size = self.nb_lines * rhs.nb_columns;
        let mut res = Matrix {
            nb_lines : self.nb_lines,
            nb_columns : rhs.nb_columns,
            data : {
                let mut temp = Vec::with_capacity(size);
                unsafe {temp.set_len(size)}
                temp
            }
        };
        for i in 0..self.nb_lines {
            for j in 0..rhs.nb_columns {
                res [i][j] = self[i][0] * rhs[0][j];
                for k in 1..self.nb_columns {
                    res[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        res
    }
}
impl<T> Mul<Matrix<T>> for &Matrix<T> where T : Mul<Output = T> + AddAssign + Copy {
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        assert_eq!(self.nb_columns, rhs.nb_lines, "{}", MULTIPLICATION_ERROR);
        let size = self.nb_lines * rhs.nb_columns;
        let mut res = Matrix {
            nb_lines : self.nb_lines,
            nb_columns : rhs.nb_columns,
            data : {
                let mut temp = Vec::with_capacity(size);
                unsafe {temp.set_len(size)}
                temp
            }
        };
        for i in 0..self.nb_lines {
            for j in 0..rhs.nb_columns {
                res [i][j] = self[i][0] * rhs[0][j];
                for k in 1..self.nb_columns {
                    res[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        res
    }
}
impl<T> Mul<&Matrix<T>> for Matrix<T> where T : Mul<Output = T> + AddAssign + Copy {
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        assert_eq!(self.nb_columns, rhs.nb_lines, "{}", MULTIPLICATION_ERROR);
        let size = self.nb_lines * rhs.nb_columns;
        let mut res = Matrix {
            nb_lines : self.nb_lines,
            nb_columns : rhs.nb_columns,
            data : {
                let mut temp = Vec::with_capacity(size);
                unsafe {temp.set_len(size)}
                temp
            }
        };
        for i in 0..self.nb_lines {
            for j in 0..rhs.nb_columns {
                res [i][j] = self[i][0] * rhs[0][j];
                for k in 1..self.nb_columns {
                    res[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        res
    }
}






//OVERLOADING * OPERATOR FOR SCALARS
impl<T> Mul<T> for Matrix<T> where T : Mul<T, Output = T> + Copy {
    type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        self.map(|a| a * rhs)
    }
}






//OVERLOADING / OPERATOR FOR SCALARS
impl<T> Div<T> for Matrix<T> where T : Div<T, Output = T> + Copy {
    type Output = Matrix<T>;
    fn div(self, rhs: T) -> Self::Output {
        self.map(|a| a / rhs)
    }
}







//OVERLOADING & OPERATOR for the Hadamard product
static HADAMAR_PRODUCT_ERROR : &str = "Can't make the Hadamard product of matrices with different shapes";

impl<T> BitAnd for Matrix<T> where T : Mul<Output = T>  {
    type Output = Matrix<T>;

    fn bitand(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape(), "{}", HADAMAR_PRODUCT_ERROR);
        Matrix {
            nb_lines : self.nb_lines,
            nb_columns : self.nb_columns,
            data : {
                let data : Vec<T> = self.into_iter().zip(rhs.into_iter()).map(|(a, b)| a * b).collect();
                data
            }
        }
    }
}

//FUNCTIONALITIES
impl<T> Matrix<T> {
    pub fn map<F>(self, f: F) -> Matrix<T> where F: Fn(T) -> T { //Apply a function to the Matrix
        Matrix {
            nb_lines : self.nb_lines,
            nb_columns : self.nb_columns,
            data : {
                let data : Vec<T> = self.into_iter().map(f).collect();
                data
            }
        }
    }
}

impl<T> Matrix<T> where T : AddAssign + Clone + Zero {
    pub fn sum_line(self) -> Matrix<T> {
        let nb_columns = self.nb_columns;
        Matrix {
            nb_columns,
            nb_lines: 1,
            data: {
                let mut data: Vec<T> = vec![num_traits::zero(); self.nb_columns];
                for (i, value) in (0..self.size()).zip(self.into_iter()) {
                    data[i % nb_columns] += value;
                }
                data
            }
        }
    }

    pub fn sum_col(self) -> Matrix<T> {
        let nb_lines = self.nb_lines;
        Matrix {
            nb_lines,
            nb_columns : 1,
            data: {
                let mut data: Vec<T> = vec![num_traits::zero(); self.nb_columns];
                for (i, value) in (0..self.size()).zip(self.into_iter()) {
                    data[i / nb_lines] += value;
                }
                data
            }
        }
    }
}