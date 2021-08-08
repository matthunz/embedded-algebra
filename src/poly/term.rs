use std::{
    fmt,
    iter::Product,
    ops::{DivAssign, Mul, MulAssign},
};

#[derive(Clone, Copy, Debug)]
pub struct Term {
    pub coefficient: i64,
    pub exponents: [u16; 4],
}

impl Term {
    pub const fn new(coefficient: i64, exponents: [u16; 4]) -> Self {
        Self {
            coefficient,
            exponents,
        }
    }

    pub fn gcd(mut self, rhs: Self) -> Self {
        self.coefficient = gcd(self.coefficient, rhs.coefficient);
        for (a, b) in self.exponents.iter_mut().zip(rhs.exponents.iter().copied()) {
            *a = b.min(*a)
        }
        self
    }
}

impl Default for Term {
    fn default() -> Self {
        Self::new(1, [0; 4])
    }
}

impl DivAssign for Term {
    fn div_assign(&mut self, rhs: Self) {
        self.coefficient /= rhs.coefficient;
        for (dest, src) in self.exponents.iter_mut().zip(rhs.exponents.iter()) {
            *dest -= src;
        }
    }
}

impl MulAssign for Term {
    fn mul_assign(&mut self, rhs: Self) {
        self.coefficient *= rhs.coefficient;
        for (dest, src) in self.exponents.iter_mut().zip(rhs.exponents.iter()) {
            *dest += src;
        }
    }
}

impl Mul for Term {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Product for Term {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Term::default(), |acc, term| acc * term)
    }
}

#[inline]
fn gcd(mut m: i64, mut n: i64) -> i64 {
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }
    n.abs()
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.coefficient)?;

        for (pos, exp) in self.exponents.iter().copied().enumerate() {
            if exp != 0 {
                write!(f, "{}^{}", char::from((97 + pos) as u8), exp)?;
            }
        }

        Ok(())
    }
}
