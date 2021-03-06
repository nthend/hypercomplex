use rand::{prelude::*};
use rand_xorshift::XorShiftRng;
use approx::*;
use crate::{prelude::*, transform::*, Complex, Quaternion, random::Unit};


const SAMPLE_ATTEMPTS: usize = 256;


#[test]
fn chaining() {
    let mut rng = XorShiftRng::seed_from_u64(0xBEEF0);
    for _ in 0..SAMPLE_ATTEMPTS {
        let a: Moebius<Complex<f64>> = rng.sample(&Normalized);
        let b: Moebius<Complex<f64>> = rng.sample(&Normalized);
        let c: Quaternion<f64> = rng.sample(&StandardNormal);
        assert_abs_diff_eq!(
            a.chain(b).apply(c),
            a.apply(b.apply(c)),
            epsilon=1e-12,
        );
    }
}

#[test]
fn complex_derivation() {
    let mut rng = XorShiftRng::seed_from_u64(0xBEEF1);
    const EPS: f64 = 1e-8;

    for _ in 0..SAMPLE_ATTEMPTS {
        let a: Moebius<Complex<f64>> = rng.sample(&Normalized);
        let p: Complex<f64> = rng.sample(&StandardNormal);
        let v: Complex<f64> = rng.sample(&Unit);

        let deriv = a.deriv(p);
        let dabs = deriv.abs();
        assert_abs_diff_eq!(
            (a.apply(p + EPS * v) - a.apply(p)) / (EPS * v),
            deriv,
            epsilon=1e-4*dabs
        );
    }
}

#[test]
fn quaternion_directional_derivation() {
    let mut rng = XorShiftRng::seed_from_u64(0xBEEF2);
    const EPS: f64 = 1e-8;

    for _ in 0..SAMPLE_ATTEMPTS {
        let a: Moebius<Complex<f64>> = rng.sample(&Normalized);
        let p: Quaternion<f64> = rng.sample(&StandardNormal);
        let v: Quaternion<f64> = rng.sample(&Unit);

        let deriv = a.deriv_dir(p, v);
        let dabs = deriv.abs();
        assert_abs_diff_eq!(
            (a.apply(p + EPS * v) - a.apply(p)) / EPS,
            deriv,
            epsilon=1e-4*dabs
        );
    }
}
