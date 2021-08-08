pub mod poly;
pub use poly::Polynomial;

#[cfg(test)]
mod tests {
    use super::poly::*;

    #[test]
    fn it_works() {
        let a = Polynomial::builder()
            .monomial(Monomial::from(4).exponent(0, 2))
            .monomial(Monomial::from(2).exponent(1, 1))
            .monomial(Monomial::from(2).exponent(0, 2))
            .build();

        println!("{}", &a);
        println!("{}", a.into_combined());
    }
}
