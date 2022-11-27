use std::fmt::Display;
use std::ops::{Index, IndexMut};
use num_traits::{one, zero, Zero, One};
use rand::{Rng, distributions::Distribution, thread_rng};
use rand::distributions::Uniform;


mod operations;
mod iterators;

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

    pub fn chose_lines(&self, chosen_lines : &Vec<usize>) -> Matrix<T>  where T : Copy {
        let nb_of_chosen_lines = chosen_lines.len();
        Matrix {
            nb_lines : nb_of_chosen_lines,
            nb_columns : self.nb_columns,
            size : self.nb_columns * nb_of_chosen_lines,
            data : {
                let mut data = Vec::<T>::with_capacity(self.nb_columns * nb_of_chosen_lines);
                for i in 0..nb_of_chosen_lines {
                    data.extend_from_slice(&self[chosen_lines[i]]);
                }
                data
            }
        }
    }

    pub fn chose_rnd_lines(&self, nb_of_selected_line : usize) -> Matrix<T> where T : Copy {
        let range = Uniform::new(0, self.nb_lines);
        self.chose_lines(&thread_rng().sample_iter(range).take(nb_of_selected_line).collect())
    }

    pub fn split_lines(self, nb_of_parts : usize) -> Vec<Matrix<T>> {
        assert!(self.nb_lines >= nb_of_parts, "You asked to split this matrix in a number of part greater than the number of lines");
        let main_part = self.nb_lines / nb_of_parts;
        let mut remainder = self.nb_lines - main_part * nb_of_parts;
        let mut sizes = vec![main_part; nb_of_parts];
        let mut i = 0;
        while remainder != 0 {
            sizes[i] += 1;
            i += 1;
            remainder -= 1;
        }
        let mut res = Vec::with_capacity(nb_of_parts);
        let nb_columns = self.nb_columns;
        let mut matrice_iter = self.into_iter();
        for i in 0..nb_of_parts {
            res.push(Matrix {
                nb_lines: sizes[i],
                nb_columns,
                size: sizes[i] * nb_columns,
                data: matrice_iter.by_ref().take(sizes[i] * nb_columns).collect(),
            })
        }
        res
    }

    pub fn concatenate_lines(self, other : Matrix<T>) -> Matrix<T> {
        assert_eq!(self.nb_columns, other.nb_columns, "Error can't concatenate matrix with different number of columns");
        let nb_lines = self.nb_lines + other.nb_lines;
        let nb_columns = self.nb_columns;
        let size = nb_lines * self.nb_columns;
        let mut data = Vec::with_capacity(size);
        data.extend(self.into_iter());
        data.extend(other.into_iter());
        Matrix {
            nb_lines,
            nb_columns,
            size,
            data,
        }
    }
}

pub trait Concatenate {
    type Item;
    fn concatenate_lines(self) -> Self::Item;
}

impl<T, I> Concatenate for I where I : IntoIterator<Item = Matrix<T>> {
    type Item = Matrix<T>;

    fn concatenate_lines(self) -> Self::Item {
        let mut iterator = self.into_iter();
        let first_matrix = iterator.next().unwrap();
        let nb_columns = first_matrix.nb_columns;
        let mut nb_lines = first_matrix.nb_lines;
        let mut data : Vec<T> = first_matrix.into_iter().collect();

        for matrix in iterator {
            assert_eq!(nb_columns, matrix.nb_columns, "Error can't concatenate matrix with different number of columns");
            nb_lines += matrix.nb_lines;
            data.extend(matrix.into_iter());
        }
        Matrix {
            nb_lines,
            nb_columns,
            size: nb_lines * nb_columns,
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



















