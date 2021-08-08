use std::{
    fmt,
    ops::{Div, MulAssign},
};

mod frac;
pub use frac::Fraction;

mod term;
pub use term::Term;

#[derive(Debug)]
pub struct Polynomial<'t> {
    terms: &'t mut [Term],
}

impl<'t> Polynomial<'t> {
    pub fn new(terms: &'t mut [Term]) -> Self {
        Self { terms }
    }

    #[inline]
    pub fn gcd(&self) -> Term {
        let mut iter = self.terms.iter().copied();
        if let Some(init) = iter.next() {
            iter.fold(init, |acc, term| acc.gcd(term))
        } else {
            Term::default()
        }
    }

    pub fn nonzero(&self) -> impl Iterator<Item = Term> + '_ {
        self.terms
            .iter()
            .copied()
            .filter(|term| term.coefficient > 0)
    }

    pub fn combine(self) -> Combine<'t> {
        Combine { poly: self }
    }
}

impl<'n, 'd> Div<Polynomial<'d>> for Polynomial<'n> {
    type Output = Fraction<'n, 'd>;

    fn div(self, rhs: Polynomial<'d>) -> Self::Output {
        let frac = Fraction::new(self, rhs);
        let gcd = frac.numerator.gcd().gcd(frac.denominator.gcd());

        for term in frac
            .numerator
            .terms
            .iter_mut()
            .chain(frac.denominator.terms.iter_mut())
        {
            *term /= gcd;
        }
        frac
    }
}

impl MulAssign for Polynomial<'_> {
    fn mul_assign(&mut self, rhs: Self) {
        let product: Term = rhs.terms.iter().copied().product();
        for term in self.terms.iter_mut() {
            *term *= product;
        }
    }
}

impl fmt::Display for Polynomial<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for term in self.terms.iter() {
            write!(f, "{}", term)?;
        }
        Ok(())
    }
}

pub struct Combine<'t> {
    poly: Polynomial<'t>,
}

impl Iterator for Combine<'_> {
    type Item = Term;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = self
            .poly
            .terms
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
