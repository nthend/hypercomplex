use std::ops::{
    Neg, Add, Sub, Mul, Div, Rem, 
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
};
use num_traits::{Num, Float, Zero, One, Inv};
//use std::fmt::{Display, Formatter, Result as FmtResult};

use std::marker::PhantomData;

use crate::traits::{Conj, SqrAbs, Algebra};


/// Cayley–Dickson construction, a basic building block
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Construction<T: Float, A: Algebra<T>>(pub A, pub A, PhantomData<T>);

impl<T: Float, A: Algebra<T>> Construction<T, A> {
    /// Create from two parts
    pub fn new2(re: A, im: A) -> Self {
        Self(re, im, PhantomData)
    }
}

impl<T: Float, A: Algebra<T>> Conj for Construction<T, A> {
    fn conj(self) -> Self {
        Self::new2(self.0.conj(), -self.1)
    }
}

impl<T: Float, A: Algebra<T>> Neg for Construction<T, A> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new2(-self.0, -self.1)
    }
}

impl<T: Float, A: Algebra<T>> Add for Construction<T, A> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new2(self.0 + other.0, self.1 + other.1)
    }
}

impl<T: Float, A: Algebra<T>> Sub for Construction<T, A> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new2(self.0 - other.0, self.1 - other.1)
    }
}

impl<T: Float, A: Algebra<T>> Mul<T> for Construction<T, A> {
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        Self::new2(self.0*other, self.1*other)
    }
}

impl<T: Float, A: Algebra<T>> Div<T> for Construction<T, A> {
    type Output = Self;
    fn div(self, other: T) -> Self::Output {
        Self::new2(self.0/other, self.1/other)
    }
}

impl<T: Float, A: Algebra<T>> SqrAbs for Construction<T, A> {
    type Output = T;
    fn sqr_abs(self) -> T {
        self.0.sqr_abs() + self.1.sqr_abs()
    }
}

impl<T: Float, A: Algebra<T>> Construction<T, A> {
    pub fn abs(self) -> T {
        self.sqr_abs().sqrt()
    }
}

impl<T: Float, A: Algebra<T>> Inv for Construction<T, A> {
    type Output = Self;
    fn inv(self) -> Self {
        self.conj()/self.sqr_abs()
    }
}

macro_rules! rmul {
    ($F:ident) => (
        /// Workaround for reverse multiplication
        impl<A: Algebra<$F>> Mul<Construction<$F, A>> for $F {
            type Output = Construction<$F, A>;
            fn mul(self, other: Construction<$F, A>) -> Self::Output {
                other*self
            }
        }
    )
}
rmul!(f32);
rmul!(f64);

macro_rules! rdiv {
    ($F:ident) => (
        /// Workaround for reverse division
        impl<A: Algebra<$F>> Div<Construction<$F, A>> for $F {
            type Output = Construction<$F, A>;
            fn div(self, other: Construction<$F, A>) -> Self::Output {
                other.inv()*self
            }
        }
    )
}
rdiv!(f32);
rdiv!(f64);

impl<T: Float, A: Algebra<T>> Mul for Construction<T, A> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self::new2(
            self.0*other.0 - other.1.conj()*self.1,
            other.1*self.0 + self.1*other.0.conj(),
        )
    }
}

impl<T: Float, A: Algebra<T>> Div for Construction<T, A> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        self*other.inv()
    }
}

impl<T: Float, A: Algebra<T>> Zero for Construction<T, A> {
    fn zero() -> Self {
        Self::new2(A::zero(), A::zero())
    }
    fn is_zero(&self) -> bool {
        self.0.is_zero() && self.1.is_zero()
    }
}

impl<T: Float, A: Algebra<T>> One for Construction<T, A> {
    fn one() -> Self {
        Self::new2(A::one(), A::zero())
    }
}

/// Not implemented yet
impl<T: Float, A: Algebra<T>> Rem for Construction<T, A> {
    type Output = Self;
    fn rem(self, _other: Self) -> Self::Output {
        unimplemented!()
    }
}

/// Not implemented yet
impl<T: Float, A: Algebra<T>> Num for Construction<T, A> {
    type FromStrRadixErr = ();
    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        unimplemented!()
    }
}

impl<T: Float, A: Algebra<T>> Algebra<T> for Construction<T, A> {}


impl<T: Float, A: Algebra<T>> AddAssign for Construction<T, A> {
    fn add_assign(&mut self, other: Self) -> () {
        *self = *self + other;
    }
}

impl<T: Float, A: Algebra<T>> SubAssign for Construction<T, A> {
    fn sub_assign(&mut self, other: Self) -> () {
        *self = *self - other;
    }
}

impl<T: Float, A: Algebra<T>> MulAssign for Construction<T, A> {
    fn mul_assign(&mut self, other: Self) -> () {
        *self = *self*other;
    }
}

impl<T: Float, A: Algebra<T>> DivAssign for Construction<T, A> {
    fn div_assign(&mut self, other: Self) -> () {
        *self = *self/other;
    }
}

impl<T: Float, A: Algebra<T>> RemAssign for Construction<T, A> {
    fn rem_assign(&mut self, other: Self) -> () {
        *self = *self % other;
    }
}

impl<T: Float, A: Algebra<T>> MulAssign<T> for Construction<T, A> {
    fn mul_assign(&mut self, other: T) -> () {
        *self = *self*other;
    }
}

impl<T: Float, A: Algebra<T>> DivAssign<T> for Construction<T, A> {
    fn div_assign(&mut self, other: T) -> () {
        *self = *self/other;
    }
}


impl<T: Float, A: Algebra<T>> Construction<T, A> {
    #[inline]
    pub fn re(self) -> A {
        self.0
    }
    #[inline]
    pub fn im(self) -> A {
        self.1
    }
}

impl<T: Float, A: Algebra<T>> Construction<T, Construction<T, A>> {
    /// Create from four parts
    pub fn new4(w: A, x: A, y: A, z: A) -> Self {
        Self::new2(Construction::new2(w, x), Construction::new2(y, z))
    }

    #[inline]
    pub fn w(self) -> A {
        self.re().re()
    }
    #[inline]
    pub fn x(self) -> A {
        self.re().im()
    }
    #[inline]
    pub fn y(self) -> A {
        self.im().re()
    }
    #[inline]
    pub fn z(self) -> A {
        self.im().im()
    }
}


/// Dummy test to force codecov look at this file
#[cfg(test)]
#[test]
fn dummy() {
    assert_eq!(1, 1);
}