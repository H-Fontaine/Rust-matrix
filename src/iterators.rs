use num_traits::Num;
use crate::Matrix;


struct MatrixMutIterator<'a, T : Num> {
    data : &'a mut [T],
}

impl<'a, T : Num> Iterator for MatrixMutIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let data = std::mem::replace(&mut self.data, &mut []);
        match data.split_first_mut() {
            Some((head, tail)) => {
                self.data = tail;
                Some(head)
            }
            None => None,
        }
    }
}

impl<'a, T : Num> IntoIterator for &'a mut Matrix<T> {
    type Item = &'a mut T;
    type IntoIter = MatrixMutIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        MatrixMutIterator {
            data : &mut self.data,
        }
    }
}

impl<T : Num> IntoIterator for Matrix<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_vec().into_iter()
    }
}