use num_traits::{Num, one, zero};


mod operations;
mod iterators;

pub struct Matrix<T : Num> {
    nb_lines : usize,
    nb_columns : usize,
    size: usize,
    data : Box<[T]>,
}

impl<T : Num> Matrix<T> where T : Copy {
    pub fn new(nb_lines : usize, nb_columns : usize, value : T) -> Matrix<T> {
        Matrix {
            nb_lines,
            nb_columns,
            size : nb_lines * nb_columns,
            data : vec![value; nb_lines * nb_columns].into_boxed_slice(),
        }
    }

    pub fn zeros(nb_lines : usize, nb_columns : usize) -> Matrix<T> {
        Matrix {
            nb_lines,
            nb_columns,
            size : nb_lines * nb_columns,
            data : vec![zero(); nb_lines * nb_columns].into_boxed_slice(),
        }
    }

    pub fn ones(nb_lines : usize, nb_columns : usize) -> Matrix<T> {
        Matrix {
            nb_lines,
            nb_columns,
            size : nb_lines * nb_columns,
            data : vec![one(); nb_lines * nb_columns].into_boxed_slice(),
        }
    }
}