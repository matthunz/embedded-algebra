use crate::{parse::Parser, Fraction, Gcd, Monomial};
use std::{
    fmt,
    iter::FromIterator,
    ops::{Div, DivAssign, MulAssign},
    str::FromStr,
};

pub trait Monomials: AsRef<[Monomial]> + AsMut<[Monomial]> {}

impl<T> Monomials for T where T: AsRef<[Monomial]> + AsMut<[Monomial]> + ?Sized {}

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

    /// Compute the greatest common divisor as a monomial
    /// ```
    /// use embedded_algebra::{Monomial, Polynomial};
    ///
    /// let poly = Polynomial::from("6a^2 + 4a");
    /// let gcf = poly.gcf();
    ///
    /// assert_eq!(gcf, Monomial::from("2a"));
    /// ```
    #[inline]
    pub fn gcf(&self) -> Monomial {
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

    /// Returns an iterator that outputs combined terms
    pub fn combine(&mut self) -> Combine<T> {
        Combine { poly: self }
    }

    /// Combine like terms into a new polynomial
    /// ```
    /// use embedded_algebra::Polynomial;
    ///
    /// let poly = Polynomial::from("2a + 2a^2 + a");
    /// let combined = poly.into_combined();
    ///
    /// assert_eq!(combined, Polynomial::from("3a + 2a^2"));
    /// ```
    pub fn into_combined(mut self) -> Polynomial {
        let monomials = self.combine().collect::<Vec<_>>().into();
        Polynomial::new(monomials)
    }
}

impl<T, U> Gcd<Polynomial<U>> for Polynomial<T>
where
    T: Monomials,
    U: Monomials,
{
    type Output = Monomial;

    fn gcd(&self, rhs: &Polynomial<U>) -> Self::Output {
        self.gcd(&rhs.gcf())
    }
}

impl<T> Gcd<Monomial> for Polynomial<T>
where
    T: Monomials,
{
    type Output = Monomial;

    fn gcd(&self, rhs: &Monomial) -> Self::Output {
        self.gcf().gcd(*rhs)
    }
}

impl<T> PartialEq for Polynomial<T>
where
    T: Monomials,
{
    fn eq(&self, other: &Self) -> bool {
        self.monomials() == other.monomials()
    }
}

impl<T> DivAssign<Monomial> for Polynomial<T>
where
    T: Monomials,
{
    fn div_assign(&mut self, rhs: Monomial) {
        for term in self.monomials_mut().iter_mut() {
            *term /= rhs;
        }
    }
}

impl<T, U> Div<Polynomial<U>> for Polynomial<T>
where
    T: Monomials,
    U: Monomials,
{
    type Output = Fraction<Self, Polynomial<U>>;

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

impl FromStr for Polynomial {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Parser::new(s).collect())
    }
}

impl From<&str> for Polynomial {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
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
