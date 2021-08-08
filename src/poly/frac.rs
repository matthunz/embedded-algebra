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
