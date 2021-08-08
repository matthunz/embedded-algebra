use std::fmt;

use super::Polynomial;

#[derive(Debug)]
pub struct Fraction {
    pub numerator: Polynomial,
    pub denominator: Polynomial,
}

impl Fraction {
    pub fn new(numerator: Polynomial, denominator: Polynomial) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) / ({})", self.numerator, self.denominator)
    }
}
