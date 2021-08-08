pub mod poly;
pub use poly::Polynomial;

#[cfg(test)]
mod tests {
    use super::poly::*;

    #[test]
    fn it_works() {
        let a = Polynomial::new(vec![Term::new(2, [4, 4, 3, 3])]);
        let b = Polynomial::new(vec![Term::new(4, [2, 8, 2, 5])]);

        dbg!(a / b);
    }
}
