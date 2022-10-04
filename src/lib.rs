use std::fmt::Display;
use std::ops::{Index, IndexMut};
use num_traits::{Num, one, zero};
use rand::{Rng, distributions::Distribution};


mod operations;
mod iterators;

pub struct Matrix<T : Num> {
    nb_lines : usize,
    nb_columns : usize,
    size: usize,
    data : Box<[T]>,
}


//METHODS TO CREATE NEW MATRIX
impl<T : Num> Matrix<T> where T : Copy {
    pub fn new(nb_lines : usize, nb_columns : usize, value : T) -> Matrix<T> { //create a matrix filled with the value "value"
        Matrix {
            nb_lines,
            nb_columns,
            size : nb_lines * nb_columns,
            data : vec![value; nb_lines * nb_columns].into_boxed_slice(),
        }
    }

    pub fn zeros(nb_lines : usize, nb_columns : usize) -> Matrix<T> { //create a matrix filled with the value zero
        Matrix {
            nb_lines,
            nb_columns,
            size : nb_lines * nb_columns,
            data : vec![zero(); nb_lines * nb_columns].into_boxed_slice(),
        }
    }

    pub fn ones(nb_lines : usize, nb_columns : usize) -> Matrix<T> { //create a matrix filled with the value one
        Matrix {
            nb_lines,
            nb_columns,
            size : nb_lines * nb_columns,
            data : vec![one(); nb_lines * nb_columns].into_boxed_slice(),
        }
    }

    pub fn new_rand<R : ?Sized + Rng, D : Distribution<T>>(nb_lines: usize, nb_columns: usize, rng: &mut R, distribution: D) -> Matrix<T> { //create a matrix filled with random values
        let data: Vec<T> = rng.sample_iter(distribution).take(nb_lines * nb_columns).collect();
        Matrix {
            nb_lines,
            nb_columns,
            size : nb_lines * nb_columns,
            data : data.into_boxed_slice(),
        }
    }

    pub fn t(&self) -> Matrix<T> { //return the transposed matrix
        let mut matrix : Matrix<T> = Matrix::zeros(self.nb_columns, self.nb_lines);
        for i in  0..self.nb_lines {
            for j in 0..self.nb_columns {
                matrix[j][i] = self[i][j];
            }
        }
        matrix
    }
}

//IMPLEMENTATION OF THE CLONE TRAIT
impl<T : Num> Clone for Matrix<T> where T : Clone {
    fn clone(&self) -> Self {
        Matrix {
            nb_lines : self.nb_lines,
            nb_columns : self.nb_columns,
            size : self.size,
            data : self.data.clone(),
        }
    }
}

//OVERLOADING [.] operator
impl<T : Num> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        let begin = index * self.nb_columns;
        let end = begin + self.nb_columns;
        &self.data[begin..end]
    }
}
impl<T : Num> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let begin = index * self.nb_columns;
        let end = begin + self.nb_columns;
        &mut self.data[begin..end]
    }
}


//DISPLAY METHODS
impl<T : Num> Matrix<T> where T : Display{
    pub fn display(&self) {
        for i in 0..self.nb_lines {
            for j in 0..self.nb_columns {
                print!("{} ", self[i][j])
            }
            println!();
        }
    }
}



















