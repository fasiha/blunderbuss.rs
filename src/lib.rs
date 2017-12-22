extern crate num;

pub fn addv<T>(conjx: bool, x: &[T], incx: usize, y: &mut [T], incy: usize)
where
    T: num::Num + Copy,
{
    let mut ix: usize = 0;
    let mut iy: usize = 0;
    while ix < x.len() && iy < y.len() {
        // println!("{} + ¡!", xi + yi);
        y[iy] = y[iy] + x[ix];
        ix += incx;
        iy += incy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        let v = vec![1, 2, 3, 4];
        let mut w = vec![0, 0, 0, 0];
        addv(true, v.as_slice(), 1, w.as_mut_slice(), 2);
        println!("result: {:?}", w);

    }

}
