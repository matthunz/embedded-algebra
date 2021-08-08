use std::{
    fmt,
    iter::Product,
    ops::{DivAssign, Mul, MulAssign},
    str::FromStr,
};

use crate::parse::Parser;

pub trait Monomials: AsRef<[Monomial]> + AsMut<[Monomial]> {}

impl<T> Monomials for T where T: AsRef<[Monomial]> + AsMut<[Monomial]> + ?Sized {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Monomial {
    pub coefficient: i64,
    pub exponents: [u16; 4],
}

impl Monomial {
    pub const fn new(coefficient: i64, exponents: [u16; 4]) -> Self {
        Self {
            coefficient,
            exponents,
        }
    }

    pub const fn coefficient(coefficient: i64) -> Self {
        Self::new(coefficient, [0; 4])
    }

    pub fn exponent(mut self, index: usize, degree: u16) -> Self {
        self.exponents[index] = degree;
        self
    }

    /// Compute the greatest common divisor for this and another monomial
    pub fn gcd(mut self, rhs: Self) -> Self {
        #[inline]
        fn gcd(mut m: i64, mut n: i64) -> i64 {
            while m != 0 {
                let old_m = m;
                m = n % m;
                n = old_m;
            }
            n.abs()
        }

        self.coefficient = gcd(self.coefficient, rhs.coefficient);
        for (a, b) in self.exponents.iter_mut().zip(rhs.exponents.iter().copied()) {
            *a = b.min(*a)
        }
        self
    }
}

impl Default for Monomial {
    fn default() -> Self {
        Self::coefficient(1)
    }
}

impl From<i64> for Monomial {
    fn from(coefficient: i64) -> Self {
        Self::coefficient(coefficient)
    }
}

impl DivAssign for Monomial {
    fn div_assign(&mut self, rhs: Self) {
        self.coefficient /= rhs.coefficient;
        for (dest, src) in self.exponents.iter_mut().zip(rhs.exponents.iter()) {
            *dest -= src;
        }
    }
}

impl MulAssign for Monomial {
    fn mul_assign(&mut self, rhs: Self) {
        self.coefficient *= rhs.coefficient;
        for (dest, src) in self.exponents.iter_mut().zip(rhs.exponents.iter()) {
            *dest += src;
        }
    }
}

impl Mul for Monomial {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Product for Monomial {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Monomial::default(), |acc, monomial| acc * monomial)
    }
}

impl fmt::Display for Monomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.coefficient)?;

        for (pos, exp) in self.exponents.iter().copied().enumerate() {
            if exp != 0 {
                write!(f, "{}", char::from((97 + pos) as u8))?;

                if exp > 1 {
                    write!(f, "^{}", exp)?;
                }
            }
        }
        Ok(())
    }
}

impl FromStr for Monomial {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Parser::new(s).next().ok_or(())
    }
}

impl From<&str> for Monomial {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}
