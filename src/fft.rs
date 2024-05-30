use std::f64::consts::PI;

use num::{cast::AsPrimitive, complex::Complex, Float, Zero};

pub fn fft<T>(input: &[Complex<T>]) -> Vec<Complex<T>>
where
    T: Float + 'static,
    usize: AsPrimitive<T>,
    f64: AsPrimitive<T>,
{
    fft_inner::<T, false>(input)
}

pub fn ifft<T>(input: &[Complex<T>]) -> Vec<Complex<T>>
where
    T: Float + 'static,
    usize: AsPrimitive<T>,
    f64: AsPrimitive<T>,
{
    let mut ret = fft_inner::<T, true>(input);

    let d: T = ret.len().as_();
    for c in &mut ret {
        *c = *c / d;
    }

    ret
}

pub fn fft_inner<T, const INVERSE: bool>(input: &[Complex<T>]) -> Vec<Complex<T>>
where
    T: Float + 'static,
    usize: AsPrimitive<T>,
    f64: AsPrimitive<T>,
{
    let n = input.len();

    if n == 1 {
        return vec![input[0]];
    }

    let w: Complex<T> = Complex::from_polar(
        T::one(),
        if INVERSE { -(2.as_()) } else { 2.as_() } * PI.as_() / n.as_(),
    );

    let (pe, po) = (
        input.iter().copied().step_by(2).collect::<Vec<_>>(),
        input.iter().copied().skip(1).step_by(2).collect::<Vec<_>>(),
    );

    let (ye, yo) = (
        fft_inner::<T, INVERSE>(&pe[..]),
        fft_inner::<T, INVERSE>(&po[..]),
    );

    let mut y = vec![Complex::zero(); n];

    for j in 0..(n / 2) {
        let yowj = w.powi(j as i32) * yo[j];

        y[j] = ye[j] + yowj;
        y[j + n / 2] = ye[j] - yowj;
    }

    y
}
