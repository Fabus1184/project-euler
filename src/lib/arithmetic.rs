pub trait Arithmetic: Sized + Copy {
    fn gcd(self, other: Self) -> Self;
    fn lcm(self, other: Self) -> Self;
    fn prime_factors(self) -> Vec<Self>;
    fn divisors(self) -> Vec<Self>;
}

crate::impl_for_all!(
    [u8, u16, u32, u64, u128],
    impl Arithmetic {
        fn gcd(self, other: Self) -> Self {
            let mut a = self;
            let mut b = other;
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            a
        }

        fn lcm(self, other: Self) -> Self {
            self / self.gcd(other) * other
        }

        fn prime_factors(self) -> Vec<Self> {
            let mut n = self;
            let mut factors = Vec::new();
            let mut d = 2;
            while d * d <= n {
                while n % d == 0 {
                    factors.push(d);
                    n /= d;
                }
                d += 1;
            }
            if n > 1 {
                factors.push(n);
            }
            factors
        }

        fn divisors(self) -> Vec<Self> {
            let n = self;
            let mut divisors = Vec::new();
            let mut d = 1;
            while d * d <= n {
                if n % d == 0 {
                    divisors.push(d);
                    if d * d != n {
                        divisors.push(n / d);
                    }
                }
                d += 1;
            }
            divisors
        }
    }
);
