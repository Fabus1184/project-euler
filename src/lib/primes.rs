pub trait Primes: Sized {
    fn is_prime(&self) -> bool;
    fn primes() -> impl Iterator<Item = Self>;
}

crate::impl_for_all!(
    [u8, u16, u32, u64, u128, usize],
    impl Primes {
        fn is_prime(&self) -> bool {
            if *self < 2 {
                return false;
            }
            let limit = (*self as f64).sqrt() as Self;
            (2..=limit).all(|x| self % x != 0)
        }

        fn primes() -> impl Iterator<Item = Self> {
            (2..).filter(|&x| x.is_prime())
        }
    }
);
