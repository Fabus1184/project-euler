pub trait NumberThings {
    fn gcd(self, other: Self) -> Self;
    fn lcm(self, other: Self) -> Self;
}

impl<T: num_traits::PrimInt> NumberThings for T {
    fn gcd(self, other: Self) -> Self {
        let mut a = self;
        let mut b = other;
        while b != T::zero() {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    fn lcm(self, other: Self) -> Self {
        self / self.gcd(other) * other
    }
}

pub trait Primes: Sized {
    fn is_prime(&self) -> bool;
    fn primes() -> impl Iterator<Item = Self>;
}

macro_rules! impl_for_all {
    ([$($t:ty),*], impl $trait:ident $impl:tt) => {
        $(
            impl $trait for $t $impl
        )*
    };
}

impl_for_all!(
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
