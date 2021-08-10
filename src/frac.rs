use crate::Gcd;
use core::fmt;
use std::{fmt::Display, ops::DivAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fraction<T, U> {
    pub numerator: T,
    pub denominator: U,
}

impl<T, U> Fraction<T, U>
where
    T: Gcd<U> + DivAssign<<T as Gcd<U>>::Output>,
    T::Output: Clone,
    U: DivAssign<<T as Gcd<U>>::Output>,
{
    pub fn new(numerator: T, denominator: U) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    /// ```
    /// use embedded_algebra::{Fraction, Monomial, Polynomial};
    ///
    /// let poly = Polynomial::from("4a^2 + 2a");
    /// let mono = Monomial::from("2a");
    /// let mut frac = Fraction::new(poly, mono);
    /// frac.simplify();
    ///
    /// assert_eq!(frac, Fraction::from(Polynomial::from("2a + 1")));
    /// ```
    pub fn simplify(&mut self) {
        let gcd = self.numerator.gcd(&self.denominator);
        self.numerator /= gcd.clone();
        self.denominator /= gcd;
    }

    pub fn into_simplified(mut self) -> Self {
        self.simplify();
        self
    }
}

impl<T, U> From<T> for Fraction<T, U>
where
    T: Gcd<U> + DivAssign<<T as Gcd<U>>::Output>,
    T::Output: Clone,
    U: DivAssign<<T as Gcd<U>>::Output> + Default,
{
    fn from(numerator: T) -> Self {
        Self::new(numerator, U::default())
    }
}

impl<T, U> fmt::Display for Fraction<T, U>
where
    T: Display,
    U: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) / ({})", self.numerator, self.denominator)
    }
}
