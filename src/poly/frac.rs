use std::fmt;

use super::{Monomial, Monomials, Polynomial};

#[derive(Debug)]
pub struct Fraction<T, U> {
    pub numerator: Polynomial<T>,
    pub denominator: Polynomial<U>,
}

impl<T, U> Fraction<T, U>
where
    T: Monomials,
    U: Monomials,
{
    pub fn new(numerator: Polynomial<T>, denominator: Polynomial<U>) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn gcd(&self) -> Monomial {
        self.numerator.gcd().gcd(self.denominator.gcd())
    }

    pub fn simplify(&mut self) {
        let gcd = self.gcd();

        for monomial in self
            .numerator
            .monomials_mut()
            .iter_mut()
            .chain(self.denominator.monomials_mut().iter_mut())
        {
            *monomial /= gcd;
        }
    }

    pub fn into_simplified(mut self) -> Self {
        self.simplify();
        self
    }
}

impl<T, U> fmt::Display for Fraction<T, U>
where
    T: Monomials,
    U: Monomials,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) / ({})", self.numerator, self.denominator)
    }
}
