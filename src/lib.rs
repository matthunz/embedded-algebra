pub mod parse;

pub mod poly;
pub use poly::{Fraction, Monomial, Monomials, Polynomial};

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn it_works() {
        let p = Polynomial::from("2a^2 + ab + 2ab");
        println!("{}", p.into_combined());
    }
}
