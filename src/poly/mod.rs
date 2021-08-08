use std::{
    fmt,
    iter::FromIterator,
    ops::{Div, MulAssign},
};

mod frac;
pub use frac::Fraction;

mod mono;
pub use mono::{Monomial, Monomials};

#[derive(Debug)]
pub struct Polynomial<T = Box<[Monomial]>> {
    monomials: T,
}

impl Polynomial {
    #[inline]
    pub fn builder() -> Builder {
        Builder::default()
    }
}

impl<T> Polynomial<T>
where
    T: Monomials,
{
    pub fn new(monomials: T) -> Self {
        Self { monomials }
    }

    pub fn monomials(&self) -> &[Monomial] {
        self.monomials.as_ref()
    }

    pub fn monomials_mut(&mut self) -> &mut [Monomial] {
        self.monomials.as_mut()
    }

    #[inline]
    pub fn gcd(&self) -> Monomial {
        let mut iter = self.monomials().iter().copied();
        if let Some(init) = iter.next() {
            iter.fold(init, |acc, monomial| acc.gcd(monomial))
        } else {
            Monomial::default()
        }
    }

    pub fn nonzero(&self) -> impl Iterator<Item = Monomial> + '_ {
        self.monomials()
            .iter()
            .copied()
            .filter(|monomial| monomial.coefficient > 0)
    }

    pub fn combine(&mut self) -> Combine<T> {
        Combine { poly: self }
    }

    pub fn into_combined(mut self) -> Polynomial {
        let monomials = self.combine().collect::<Vec<_>>().into();
        Polynomial::new(monomials)
    }
}

impl<T, U> Div<Polynomial<U>> for Polynomial<T>
where
    T: Monomials,
    U: Monomials,
{
    type Output = Fraction<T, U>;

    fn div(self, rhs: Polynomial<U>) -> Self::Output {
        Fraction::new(self, rhs).into_simplified()
    }
}

impl<T> MulAssign for Polynomial<T>
where
    T: Monomials,
{
    fn mul_assign(&mut self, rhs: Self) {
        let product: Monomial = rhs.monomials().iter().copied().product();
        for monomial in self.monomials_mut().iter_mut() {
            *monomial *= product;
        }
    }
}

impl<T> fmt::Display for Polynomial<T>
where
    T: Monomials,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.monomials().iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;

            for monomial in iter {
                write!(f, " + {}", monomial)?;
            }
        }

        Ok(())
    }
}

impl FromIterator<Monomial> for Polynomial {
    fn from_iter<T: IntoIterator<Item = Monomial>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect::<Vec<_>>().into())
    }
}

pub struct Combine<'p, T> {
    poly: &'p mut Polynomial<T>,
}

impl<T> Iterator for Combine<'_, T>
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

#[derive(Default)]
pub struct Builder {
    monomials: Vec<Monomial>,
}

impl Builder {
    #[inline]
    pub fn monomial(mut self, monomial: Monomial) -> Self {
        self.monomials.push(monomial);
        self
    }

    #[inline]
    pub fn build(self) -> Polynomial {
        Polynomial::new(self.monomials.into())
    }
}
