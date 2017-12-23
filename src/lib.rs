use std::ops::Neg;

extern crate num;

// Create our own hand-rolled Conjugate trait, since addv can do this. This is inspired in part
// from https://docs.rs/ndarray-linalg/0.5.4/src/ndarray_linalg/types.rs.html#120-122 but here, the
// `conj` function takes the argument as a copy, rather than a reference (move), for no particular
// reason.
pub trait Conjugate {
    fn conj(self) -> Self;
}
// Implement the trait for num-complex:
impl<T: Copy + num::Num + Neg<Output = T>> Conjugate for num::complex::Complex<T> {
    fn conj(self) -> Self {
        num::complex::Complex::conj(&self)
    }
}
// And make a macro to implement it for all the usual real types.
macro_rules! reals_conj {
    ($real:ty) => {
        impl Conjugate for $real {
            fn conj(self) -> Self{
                self
            }
        }
    }
}
// I just invoke the macro to add Conjugate trait to the signed integers, even though it appears
// the compiler lets me do it to unsigned, which shouldn't implement `Neg`?
reals_conj!(i8);
reals_conj!(i16);
reals_conj!(i32);
reals_conj!(i64);
reals_conj!(isize);
reals_conj!(f32);
reals_conj!(f64);

// See https://github.com/flame/blis/wiki/BLISAPIQuickReference#addv
pub fn addv<T>(conjx: bool, x: &[T], incx: usize, y: &mut [T], incy: usize)
where
    T: Copy + num::Num + Conjugate,
{
    let mut ix: usize = 0;
    let mut iy: usize = 0;
    if conjx {
        while ix < x.len() && iy < y.len() {
            y[iy] = Conjugate::conj(x[iy]) + y[ix];
            ix += incx;
            iy += incy;
        }
    } else {
        while ix < x.len() && iy < y.len() {
            y[iy] = x[iy] + y[ix];
            ix += incx;
            iy += incy;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use num::complex::Complex;

        assert_eq!(2 + 2, 4);

        // addv works for complex floats and ints!
        let j = Complex { re: 0., im: 1. };
        let cv: Vec<Complex<f64>> = vec![1. + 2. * j, 3. + 4. * j];
        let mut cw = vec![10. + 20. * j, 30. + 40. * j];
        addv(true, cv.as_slice(), 1, cw.as_mut_slice(), 1);
        println!("complex result: {:?}", cw);

        let v = vec![1, 2, 3, 4];
        let mut w = vec![0, 0, 0, 0];
        addv(true, v.as_slice(), 1, w.as_mut_slice(), 2);
        println!("result: {:?}", w);

    }

}
