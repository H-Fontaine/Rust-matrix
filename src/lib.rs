use std::mem::MaybeUninit;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign};
use std::process::Output;
use num_traits::{Zero, zero};
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T, const N : usize, const M : usize> {
    data : Vec<T>
}

impl<T, const N : usize, const M : usize> Matrix<T, N, M> {
    fn uninitialized() -> Matrix<MaybeUninit<T>, N, M> {
        Matrix {
            data : Vec::with_capacity(N * M)
        }
    }
}

impl<T, const N : usize, const M : usize> Matrix<MaybeUninit<T>, N, M> {
    unsafe fn init(self) -> Matrix<T, N, M> {
        Matrix {
            data : self.into_iter().map(|a| unsafe{ a.assume_init() }).collect()
        }
    }
}


impl<T, const N : usize, const M : usize> Add<Matrix<T, N, M>> for Matrix<T, N, M> where T : Add<Output = T>{
    type Output = Matrix<T, N, M>;

    fn add(self, rhs: Matrix<T, N, M>) -> Self::Output {
        Matrix {
            data : self.into_iter().zip(rhs.into_iter()).map(|(a, b)| a + b).collect(),
        }
    }
}

impl<'a, T, const N : usize, const M : usize, const P : usize> Mul<Matrix<T, M, P>> for Matrix<T, N, M> where T : Mul<Output = T> + Copy + AddAssign<T> {
    type Output = Matrix<T, N, P>;

    fn mul(self, rhs: Matrix<T, M, P>) -> Self::Output {
        let mut res = Matrix::uninitialized();
        for i in 0..N {
            for j in 0..P {
                res[(i,j)].write(self[(i,0)] * rhs[(0,j)]);
                for k in 1..M {
                    unsafe {*res[(i,j)].assume_init_mut() += self[(i,k)] * rhs[(k,j)]}
                }
            }
        }
        unsafe {res.init()}
    }
}

impl<T, const N : usize, const M : usize> Zero for Matrix<T, N, M> where T : Add<Output = T> + Zero + Clone {
    fn zero() -> Self {
        Matrix {
            data : vec![T::zero(); N * M]
        }
    }

    fn set_zero(&mut self) {
        self.data = vec![T::zero(); N * M]
    }

    fn is_zero(&self) -> bool {
        for item in self.into_iter() {
            if !item.is_zero() {
                return false
            }
        }
        true
    }
}

impl<T, const N : usize, const M : usize> IntoIterator for Matrix<T, N, M> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}


impl<'a, T, const N : usize, const M : usize> IntoIterator for &'a Matrix<T, N, M> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.data).into_iter()
    }
}

impl<'a, T, const N : usize, const M : usize> IntoIterator for &'a mut Matrix<T, N, M> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.data).into_iter()
    }
}

impl<T, const N : usize, const M : usize> Index<(usize, usize)> for Matrix<T, N, M> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.1 < M, "Index {}, {} is out of bound of a {}, {} matrix", index.0, index.1, N, M);
        &self.data[index.0 * N + index.1]
    }
}

impl<T, const N : usize, const M : usize> IndexMut<(usize, usize)> for Matrix<T, N, M> {

    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.1 < M, "Index {}, {} is out of bound of a {}, {} matrix", index.0, index.1, N, M);
        &mut self.data[index.0 * N + index.1]
    }
}














