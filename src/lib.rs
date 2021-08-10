mod frac;
pub use frac::Fraction;

pub mod parse;

mod mono;
pub use mono::Monomial;

pub mod poly;
pub use poly::{Monomials, Polynomial};

pub trait Gcd<Rhs = Self> {
    type Output;

    fn gcd(&self, rhs: &Rhs) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let p = Monomial::from("123a^123bc^123");
        println!("{}", p);
    }
}
