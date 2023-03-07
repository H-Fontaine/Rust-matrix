use rand::distributions::Uniform;
use rand::{Rng, thread_rng};
use crate::Matrix;

impl<T> Matrix<T> {
    /*
    Create a new Matrix by selection the lines according to the provided index
    - chosen_lines : &Vec<usize>            contains the index of the lines to select
     */
    pub fn chose_lines_by_index(&self, chosen_lines : &Vec<usize>) -> Matrix<T>  where T : Copy {
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

    /*
    Select the provided number of lines using a uniform distribution
     */
    pub fn chose_rnd_lines(&self, nb_of_selected_line : usize) -> Matrix<T> where T : Copy {
        let range = Uniform::new(0, self.nb_lines);
        self.chose_lines_by_index(&thread_rng().sample_iter(range).take(nb_of_selected_line).collect())
    }

    /*
    Select the lines according to the provided boolean vector, if true the line is selected, not otherwise
     */
    pub fn chose_lines_by_bool(&self, bools : &Vec<bool>) -> Matrix<T> where T : Copy {
        assert_eq!(bools.len(), self.nb_lines, "The length of the provided vector must be the same as the number of line of the matrix");
        let mut data = Vec::new();
        let mut nb_lines = 0;
        for i in 0..self.nb_lines {
            if bools[i] {
                data.extend_from_slice(&self[i]);
                nb_lines +=1;
            }
        }
        Matrix {
            size: self.nb_columns * nb_lines,
            nb_columns : self.nb_columns,
            nb_lines,
            data,
        }
    }

    /*
    Split the Matrix in the provided number of parts, if the number of parts asked is greater than the number of lines in the Matrix the program panic
     */
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

    /*
    Concatenate two Matrix putting the second one under the first one
    - self : Matrix<T>          The first matrix
    - other : Matrix<T>         The second matrix
     */
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

/*
impl<T, I> Concatenate for I where I : IntoIterator<Item = Matrix<T>> {
    type Item = Matrix<T>;

    /*
    Concatenate multiples Matrix by putting them under each other in the order of the iterator
    - self : I                  It is an object that can be iterate over and that returns Matrix<T>
     */
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
*/