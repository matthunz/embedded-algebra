use std::fmt;

use super::Polynomial;

#[derive(Debug)]
pub struct Fraction<'n, 'd> {
    pub numerator: Polynomial<'n>,
    pub denominator: Polynomial<'d>,
}

impl<'n, 'd> Fraction<'n, 'd> {
    pub fn new(numerator: Polynomial<'n>, denominator: Polynomial<'d>) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

impl fmt::Display for Fraction<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) / ({})", self.numerator, self.denominator)
    }
}
