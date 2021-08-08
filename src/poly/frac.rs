use std::fmt;

use super::{Polynomial, Terms};

#[derive(Debug)]
pub struct Fraction<T, U> {
    pub numerator: Polynomial<T>,
    pub denominator: Polynomial<U>,
}

impl<T, U> Fraction<T, U> {
    pub fn new(numerator: Polynomial<T>, denominator: Polynomial<U>) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

impl<T, U> fmt::Display for Fraction<T, U>
where
    T: Terms,
    U: Terms,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) / ({})", self.numerator, self.denominator)
    }
}
