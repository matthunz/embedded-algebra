use std::{
    fmt::{self},
    iter::Product,
    ops::{DivAssign, Mul, MulAssign},
    str::{Chars, FromStr},
};

pub trait Monomials: AsRef<[Monomial]> + AsMut<[Monomial]> {}

impl<T> Monomials for T where T: AsRef<[Monomial]> + AsMut<[Monomial]> + ?Sized {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Monomial {
    pub coefficient: i64,
    pub exponents: [u16; 4],
}

impl Monomial {
    pub const fn new(coefficient: i64, exponents: [u16; 4]) -> Self {
        Self {
            coefficient,
            exponents,
        }
    }

    pub const fn coefficient(coefficient: i64) -> Self {
        Self::new(coefficient, [0; 4])
    }

    pub fn exponent(mut self, index: usize, degree: u16) -> Self {
        self.exponents[index] = degree;
        self
    }

    /// Compute the greatest common divisor for this and another monomial
    /// ```
    /// use embedded_algebra::Monomial;
    ///
    /// let a = Monomial::from("6ab");
    /// let b = Monomial::from("4a");
    ///
    /// assert_eq!(a.gcd(b), Monomial::from("2a"));
    ///
    pub fn gcd(mut self, rhs: Self) -> Self {
        #[inline]
        fn gcd(mut m: i64, mut n: i64) -> i64 {
            while m != 0 {
                let old_m = m;
                m = n % m;
                n = old_m;
            }
            n.abs()
        }

        self.coefficient = gcd(self.coefficient, rhs.coefficient);
        for (a, b) in self.exponents.iter_mut().zip(rhs.exponents.iter().copied()) {
            *a = b.min(*a)
        }
        self
    }
}

impl Default for Monomial {
    fn default() -> Self {
        Self::coefficient(1)
    }
}

impl From<i64> for Monomial {
    fn from(coefficient: i64) -> Self {
        Self::coefficient(coefficient)
    }
}

impl DivAssign for Monomial {
    fn div_assign(&mut self, rhs: Self) {
        self.coefficient /= rhs.coefficient;
        for (dest, src) in self.exponents.iter_mut().zip(rhs.exponents.iter()) {
            *dest -= src;
        }
    }
}

impl MulAssign for Monomial {
    fn mul_assign(&mut self, rhs: Self) {
        self.coefficient *= rhs.coefficient;
        for (dest, src) in self.exponents.iter_mut().zip(rhs.exponents.iter()) {
            *dest += src;
        }
    }
}

impl Mul for Monomial {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Product for Monomial {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Monomial::default(), |acc, monomial| acc * monomial)
    }
}

impl fmt::Display for Monomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.coefficient)?;

        for (pos, exp) in self.exponents.iter().copied().enumerate() {
            if exp != 0 {
                write!(f, "{}", char::from((97 + pos) as u8))?;

                if exp > 1 {
                    write!(f, "^{}", exp)?;
                }
            }
        }
        Ok(())
    }
}

impl From<&str> for Monomial {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum OverflowError {
    Coefficient,
    Exponent,
}

#[derive(Clone, Copy, Debug)]
pub enum ParseError {
    Empty,
    Overflow(OverflowError),
    Symbol,
}

impl FromStr for Monomial {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut pos = 0;
        let mut mono = Self::default();

        match chars.next() {
            Some('0'..='9') => {
                pos += 1;
                let (coeff, next) = parse_int(s, &mut chars, 0, &mut pos)
                    .ok_or(ParseError::Overflow(OverflowError::Coefficient))?;
                mono.coefficient = coeff;
                match next {
                    Some(c) => {
                        return parse_vars(s, &mut chars, &mut pos, c, &mut mono).map(|()| mono)
                    }
                    None => Ok(mono),
                }
            }
            Some(c) => parse_vars(s, &mut chars, &mut pos, c, &mut mono).map(|()| mono),
            None => Err(ParseError::Empty),
        }
    }
}

#[inline]
fn parse_vars(
    s: &str,
    chars: &mut Chars,
    pos: &mut usize,
    c: char,
    mono: &mut Monomial,
) -> Result<(), ParseError> {
    let mut c = c;
    loop {
        let idx = (c as u8 - 97) as usize;
        if idx > 4 {
            return Err(ParseError::Symbol);
        }
        match chars.next() {
            Some('^') => {
                *pos += 2;
                let (exp, next) = parse_int(s, chars, *pos, pos)
                    .ok_or(ParseError::Overflow(OverflowError::Exponent))?;
                mono.exponents[idx] = exp as _;
                if let Some(next) = next {
                    c = next;
                } else {
                    break;
                }
            }
            Some(c2) => {
                *pos += 1;
                mono.exponents[idx] = 1;
                c = c2;
            }
            None => {
                mono.exponents[idx] = 1;
                break;
            }
        }
    }
    Ok(())
}

#[inline]
fn parse_int(
    s: &str,
    chars: &mut Chars,
    start: usize,
    pos: &mut usize,
) -> Option<(i64, Option<char>)> {
    let next = loop {
        match chars.next() {
            Some('0'..='9') => {
                *pos += 1;
            }
            next => break next,
        }
    };
    let int_str = &s[start..*pos];
    int_str.parse().ok().map(|int| (int, next))
}
