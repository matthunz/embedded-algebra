use std::{
    fmt,
    ops::{Div, MulAssign},
};

mod frac;
pub use frac::Fraction;

mod term;
pub use term::{Term, Terms};

#[derive(Debug)]
pub struct Polynomial<T> {
    terms: T,
}

impl<T> Polynomial<T>
where
    T: Terms,
{
    pub fn new(terms: T) -> Self {
        Self { terms }
    }

    pub fn terms(&self) -> &[Term] {
        self.terms.as_ref()
    }

    pub fn terms_mut(&mut self) -> &mut [Term] {
        self.terms.as_mut()
    }

    #[inline]
    pub fn gcd(&self) -> Term {
        let mut iter = self.terms().iter().copied();
        if let Some(init) = iter.next() {
            iter.fold(init, |acc, term| acc.gcd(term))
        } else {
            Term::default()
        }
    }

    pub fn nonzero(&self) -> impl Iterator<Item = Term> + '_ {
        self.terms()
            .iter()
            .copied()
            .filter(|term| term.coefficient > 0)
    }

    pub fn combine(&mut self) -> Combine<T> {
        Combine { poly: self }
    }
}

impl<T, U> Div<Polynomial<U>> for Polynomial<T>
where
    T: Terms,
    U: Terms,
{
    type Output = Fraction<T, U>;

    fn div(self, rhs: Polynomial<U>) -> Self::Output {
        let mut frac = Fraction::new(self, rhs);
        let gcd = frac.numerator.gcd().gcd(frac.denominator.gcd());

        for term in frac
            .numerator
            .terms_mut()
            .iter_mut()
            .chain(frac.denominator.terms_mut().iter_mut())
        {
            *term /= gcd;
        }
        frac
    }
}

impl<T> MulAssign for Polynomial<T>
where
    T: Terms,
{
    fn mul_assign(&mut self, rhs: Self) {
        let product: Term = rhs.terms().iter().copied().product();
        for term in self.terms_mut().iter_mut() {
            *term *= product;
        }
    }
}

impl<T> fmt::Display for Polynomial<T>
where
    T: Terms,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for term in self.terms().iter() {
            write!(f, "{}", term)?;
        }
        Ok(())
    }
}

pub struct Combine<'p, T> {
    poly: &'p mut Polynomial<T>,
}

impl<T> Iterator for Combine<'_, T>
where
    T: Terms,
{
    type Item = Term;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = self
            .poly
            .terms_mut()
            .iter_mut()
            .filter(|term| term.coefficient > 0);

        if let Some(next) = iter.next() {
            let mut acc = *next;
            next.coefficient = 0;

            for term in iter {
                acc.coefficient += term.coefficient;
                term.coefficient = 0;
            }

            Some(acc)
        } else {
            None
        }
    }
}
