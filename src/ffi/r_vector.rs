use std::any::Any;
use std::ops::Index;
use crate::ffi::CWrapperFFI;

#[repr(C)]
pub struct RVector<T> {
    data: *const T,
    len: usize,
}

impl<T> RVector<T> {
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.data, self.len)
        }
    }

    pub fn to_rust(&self) -> Vec<T>
    where T: Clone
    {
        self.as_slice().to_vec()
    }
}

impl<T> From<Vec<T>> for RVector<T> {
    fn from(v: Vec<T>) -> Self {
        let len = v.len();
        let data = v.as_ptr();
        std::mem::forget(v);
        Self { data, len }
    }
}

impl<T> Index<usize> for RVector<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}


#[cfg(test)]
mod tests {
    use crate::ffi::r_vector::RVector;

    #[test]
    fn index_test() {
        let rvec = RVector::from(vec![1,2,3,4,5]);

        assert_eq!(rvec[0], 1);
        assert_eq!(rvec[1], 2);
        assert_eq!(rvec[2], 3);
        assert_eq!(rvec[3], 4);
        assert_eq!(rvec[4], 5);

    }
}