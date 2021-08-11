use super::{Monomials, Polynomial};
use crate::Monomial;

pub struct Combine<T> {
    poly: Polynomial<T>,
}

impl<T> From<Polynomial<T>> for Combine<T> {
    fn from(poly: Polynomial<T>) -> Self {
        Self { poly }
    }
}

impl<T> Iterator for Combine<T>
where
    T: Monomials,
{
    type Item = Monomial;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = self
            .poly
            .monomials_mut()
            .iter_mut()
            .filter(|monomial| monomial.coefficient > 0);

        if let Some(next) = iter.next() {
            let mut acc = *next;
            next.coefficient = 0;

            for monomial in iter {
                if monomial.exponents == acc.exponents {
                    acc.coefficient += monomial.coefficient;
                    monomial.coefficient = 0;
                }
            }

            Some(acc)
        } else {
            None
        }
    }
}
