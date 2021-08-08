use std::ops::{Div, MulAssign};

mod frac;
pub use frac::Fraction;

mod term;
pub use term::Term;

#[derive(Debug)]
pub struct Polynomial {
    terms: Vec<Term>,
}

impl Polynomial {
    pub fn new(terms: Vec<Term>) -> Self {
       Self { terms} 
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
}

impl Div for Polynomial {
    type Output = Fraction;

    fn div(self, rhs: Self) -> Self::Output {
        let mut frac = Fraction::new(self, rhs);
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

impl MulAssign for Polynomial {
    fn mul_assign(&mut self, rhs: Self) {
        let product: Term = rhs.terms.iter().copied().product();
        for term in self.terms.iter_mut() {
            *term *= product;
        }
    }
}
