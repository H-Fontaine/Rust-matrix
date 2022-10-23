use num_traits::Num;
use crate::Matrix;


pub struct MatrixMutRefIterator<'a, T> {
    data : &'a mut [T],
}
impl<'a, T> Iterator for MatrixMutRefIterator<'a, T> {
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
impl<'a, T> IntoIterator for &'a mut Matrix<T> {
    type Item = &'a mut T;
    type IntoIter = MatrixMutRefIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        MatrixMutRefIterator {
            data : &mut self.data,
        }
    }
}


pub struct MatrixRefIterator<'a, T> {
    parent : &'a Matrix<T>,
    index : usize,

}
impl<'a, T> Iterator for MatrixRefIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.parent.size {
            let res = &self.parent.data[self.index];
            self.index += 1;
            Some(res)
        }
        else {
            None
        }
    }
}
impl<'a, T> IntoIterator for &'a Matrix<T> {
    type Item = &'a T;
    type IntoIter = MatrixRefIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        MatrixRefIterator {
            parent : self,
            index : 0,
        }
    }
}


impl<T> IntoIterator for Matrix<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_vec().into_iter()
    }
}