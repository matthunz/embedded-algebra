pub mod parse;

pub mod poly;
pub use poly::{Fraction, Monomial, Monomials, Polynomial};

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn it_works() {
        let p = Monomial::from("123a^123bc^123");
        println!("{}", p);
    }
}
