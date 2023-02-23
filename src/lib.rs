use std::fmt::Display;
use std::ops::{Index, IndexMut};
use num_traits::{one, zero, Zero, One};
use rand::{Rng, distributions::Distribution, thread_rng};
use rand::distributions::Uniform;


mod operations;
mod iterators;
mod functionalities;

pub struct Matrix<T> {
    nb_lines : usize,
    nb_columns : usize,
    size: usize,
    data : Vec<T>,
}


//METHODS TO CREATE NEW MATRIX
impl<T> Matrix<T> {
    pub fn new() -> Matrix<T> { //create a matrix filled with the value "value"
        Matrix {
            nb_lines : 0,
            nb_columns : 0,
            size : 0,
            data : Vec::new(),
        }
    }

    pub fn zeros(nb_lines : usize, nb_columns : usize) -> Matrix<T> where T : Zero + Copy { //create a matrix filled with the value zero
        Matrix {
            nb_lines,
            nb_columns,
            size : nb_lines * nb_columns,
            data : vec![zero(); nb_lines * nb_columns],
        }
    }

    pub fn ones(nb_lines : usize, nb_columns : usize) -> Matrix<T> where T : One + Copy { //create a matrix filled with the value one
        Matrix {
            nb_lines,
            nb_columns,
            size : nb_lines * nb_columns,
            data : vec![one(); nb_lines * nb_columns],
        }
    }

    pub fn new_rand<R : ?Sized + Rng, D : Distribution<T>>(nb_lines: usize, nb_columns: usize, rng: &mut R, distribution: D) -> Matrix<T> { //create a matrix filled with random values
        let data: Vec<T> = rng.sample_iter(distribution).take(nb_lines * nb_columns).collect();
        Matrix {
            nb_lines,
            nb_columns,
            size : nb_lines * nb_columns,
            data,
        }
    }

    pub fn t(&self) -> Matrix<T> where T : Copy { //return the transposed matrix
        let mut data = Vec::<T>::with_capacity(self.size);
        for j in 0..self.nb_columns {
            for i in 0..self.nb_lines {
                data.push(self[i][j])
            }
        }
        Matrix {
            nb_lines : self.nb_columns,
            nb_columns : self.nb_lines,
            size : self.size,
            data,
        }
    }
}


//GETTERS
impl<T> Matrix<T> {
    pub fn lines(&self) -> usize {
        self.nb_lines
    }

    pub fn columns(&self) -> usize {
        self.nb_columns
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

//IMPLEMENTATION OF THE CLONE TRAIT
impl<T> Clone for Matrix<T> where T : Clone {
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
impl<T> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        let begin = index * self.nb_columns;
        let end = begin + self.nb_columns;
        &self.data[begin..end]
    }
}
impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let begin = index * self.nb_columns;
        let end = begin + self.nb_columns;
        &mut self.data[begin..end]
    }
}


//DISPLAY METHODS
impl<T> Matrix<T> where T : Display {
    pub fn display(&self) {
        for i in 0..self.nb_lines {
            for j in 0..self.nb_columns {
                print!("{} ", self[i][j])
            }
            println!();
        }
    }
}



















