use std::{
    iter::Product,
    ops::{Mul, MulAssign},
};

#[derive(Clone, Copy)]
pub struct Term {
    pub coefficient: i64,
    pub exponents: [u16; 4],
}

impl Default for Term {
    fn default() -> Self {
        Self {
            coefficient: 1,
            exponents: [0; 4],
        }
    }
}

impl MulAssign for Term {
    fn mul_assign(&mut self, rhs: Self) {
        self.coefficient *= rhs.coefficient;
        for (dest, src) in self.exponents.iter_mut().zip(rhs.exponents.iter()) {
            *dest += src;
        }
    }
}

impl Mul for Term {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Product for Term {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Term::default(), |acc, term| acc * term)
    }
}

pub struct Polynomial {
    terms: Vec<Term>,
}

impl MulAssign for Polynomial {
    fn mul_assign(&mut self, rhs: Self) {
        let product: Term = rhs.terms.iter().copied().product();
        for term in self.terms.iter_mut() {
            *term *= product;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
