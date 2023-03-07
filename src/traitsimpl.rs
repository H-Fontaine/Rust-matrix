use crate::Matrix;

impl<T> PartialEq<Self> for Matrix<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        if self.shape() != other.shape() {
            return false
        }
        for (a, b) in self.into_iter().zip(other.into_iter()) {
            if a != b {
                return false
            }
        }
        true
    }
}

impl<T> Eq for Matrix<T> where T: Eq {

}