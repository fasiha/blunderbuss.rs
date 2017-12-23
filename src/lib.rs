use std::ops::Neg;

extern crate num;

pub trait Conjugate {
    fn conj(self) -> Self;
}

impl Conjugate for f64 {
    fn conj(self) -> Self {
        self
    }
}

impl<T: Copy + num::Num + Neg<Output = T>> Conjugate for num::complex::Complex<T> {
    fn conj(self) -> Self {
        num::complex::Complex::conj(&self)
    }
}

macro_rules! reals_conj {
    ($real:ty) => {
        impl Conjugate for $real {
            fn conj(self) -> Self{
                self
            }
        }
    }
}
reals_conj!(i32);

pub fn addv<T>(conjx: bool, x: &[T], incx: usize, y: &mut [T], incy: usize)
where
    T: Copy + num::Num + Conjugate,
{
    let mut ix: usize = 0;
    let mut iy: usize = 0;
    if conjx {
        while ix < x.len() && iy < y.len() {
            // println!("{} + ¡!", xi + yi);
            y[iy] = Conjugate::conj(y[iy]) + x[ix];
            ix += incx;
            iy += incy;
        }
    } else {
        while ix < x.len() && iy < y.len() {
            // println!("{} + ¡!", xi + yi);
            y[iy] = y[iy] + x[ix];
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

        let j = Complex { re: 0., im: 1. };
        let cv: Vec<Complex<f64>> = vec![1. + 2. * j, 3. + 4. * j];
        let mut cw: Vec<Complex<f64>> = vec![10. + 20. * j, 30. + 40. * j];
        addv(true, cv.as_slice(), 1, cw.as_mut_slice(), 1);
        println!("complex result: {:?}", cw);

        let v = vec![1, 2, 3, 4];
        let mut w = vec![0, 0, 0, 0];
        addv(true, v.as_slice(), 1, w.as_mut_slice(), 2);
        println!("result: {:?}", w);

    }

}
