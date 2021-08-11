use crate::{Monomial, Polynomial};

#[derive(Default)]
pub struct Builder {
    monomials: Vec<Monomial>,
}

impl Builder {
    #[inline]
    pub fn monomial<T>(mut self, monomial: T) -> Self
    where
        T: Into<Monomial>,
    {
        self.push(monomial);
        self
    }

    #[inline]
    pub fn push<T>(&mut self, monomial: T)
    where
        T: Into<Monomial>,
    {
        self.monomials.push(monomial.into());
    }

    #[inline]
    pub fn build(self) -> Polynomial {
        Polynomial::new(self.monomials.into())
    }
}
