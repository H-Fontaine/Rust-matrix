use std::ops::Add;
use num_traits::Num;
use crate::Matrix;


impl<T : Num> Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        
    }
}