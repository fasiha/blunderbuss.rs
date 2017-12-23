extern crate num;

use std::ops::Neg;
use num::complex::Complex;

// Create our own hand-rolled Conjugate trait, since `addv` needs to do this. This is inspired in
// part from https://docs.rs/ndarray-linalg/0.5.4/src/ndarray_linalg/types.rs.html#120-122 but
// here, the `conj` function takes the argument as a copy, rather than a reference (move), for no
// particular reason.
pub trait Conjugate {
    fn conj(self) -> Self;
}
// Implement the trait for num-complex:
impl<T: Copy + num::Num + Neg<Output = T>> Conjugate for Complex<T> {
    fn conj(self) -> Self {
        Self::conj(&self)
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
// I could move the `Conjugate` trait guard to `Y` and flip the order of `from(conj(x))`, not sure
// what the benefits of that are.
pub fn addv<X, Y>(conjx: bool, x: &[X], incx: usize, y: &mut [Y], incy: usize)
where
    X: Copy + num::Num + Conjugate,
    Y: Copy + num::Num + From<X>,
{
    let mut ix: usize = 0;
    let mut iy: usize = 0;
    if conjx {
        while ix < x.len() && iy < y.len() {
            y[iy] = Y::from(Conjugate::conj(x[ix])) + y[iy];
            ix += incx;
            iy += incy;
        }
    } else {
        while ix < x.len() && iy < y.len() {
            y[iy] = Y::from(x[ix]) + y[iy];
            ix += incx;
            iy += incy;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addv_rr() {
        // addv should work with real types like f64 and i16
        let v = vec![1i16, 2, 3, 4];
        let mut w = vec![-1i16, -1, -1, -1];
        addv(false, v.as_slice(), 1, w.as_mut_slice(), 1);
        assert_eq!(w[0], v[0] - 1);
        assert_eq!(w[1], v[1] - 1);
        assert_eq!(w[2], v[2] - 1);
        assert_eq!(w[3], v[3] - 1);
    }

    #[test]
    fn addv_cc() {
        // addv should be able to add Vec<Complex<T>> for T like i16 and f32
        let j = Complex { re: 0., im: 1. };
        let v = vec![1. + 2. * j, 3. + 4. * j];
        let mut w: Vec<Complex<f64>> = vec![10. + 20. * j, 30. + 40. * j];
        let wclone = w.clone();
        addv(true, v.as_slice(), 1, w.as_mut_slice(), 1);
        assert_eq!(w[0], Complex::conj(&v[0]) + wclone[0]);
        assert_eq!(w[1], Complex::conj(&v[1]) + wclone[1]);

        let j = Complex { re: 0i32, im: 1i32 };
        let v = vec![1 + 2 * j, 3 + 4 * j];
        let mut w = vec![10 + 20 * j, 30 + 40 * j];
        let wclone = w.clone();
        addv(true, v.as_slice(), 1, w.as_mut_slice(), 1);
        assert_eq!(w[0], Complex::conj(&v[0]) + wclone[0]);
        assert_eq!(w[1], Complex::conj(&v[1]) + wclone[1]);

    }

    #[test]
    fn addv_rc() {
        // addv should work mixed-domain: f64 + Complex<f64> etc.
        let j = Complex { re: 0., im: 1. };
        let v = vec![100f64, 200f64];
        let mut w = vec![-1. * j, -2. * j];
        let wclone = w.clone();
        addv(false, v.as_slice(), 1, w.as_mut_slice(), 1);
        assert_eq!(w[0], v[0] + wclone[0]);
        assert_eq!(w[1], v[1] + wclone[1]);
    }
}
