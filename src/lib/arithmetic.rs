pub trait Arithmetic: Sized + Copy {
    fn gcd(self, other: Self) -> Self;
    fn lcm(self, other: Self) -> Self;
    fn prime_factors(self) -> Vec<Self>;
    fn proper_divisors(self) -> impl Iterator<Item = Self>;
    fn digits(self) -> Vec<u8>;
}

crate::impl_for_all!(
    [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize],
    impl Arithmetic {

        #[allow(clippy::unnecessary_cast)]
        fn digits(self) -> Vec<u8> {
            let mut n = self;
            let mut digits = Vec::new();
            while n > 0 {
                digits.push((n % 10) as u8);
                n /= 10;
            }
            digits
        }

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

        fn proper_divisors(self) -> impl Iterator<Item = Self> {
            (2..(self as f64).sqrt() as Self + 1).filter(move |&x| self % x == 0).flat_map(move |x| {
                if x * x == self {
                    vec![x].into_iter()
                } else {
                    vec![x, self / x].into_iter()
                }
            }).chain(std::iter::once(1))
        }

    }
);

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::Arithmetic;

    #[test]
    fn test_digits() -> anyhow::Result<()> {
        anyhow::ensure!(12345.digits() == vec![5, 4, 3, 2, 1]);
        anyhow::ensure!(54321.digits() == vec![1, 2, 3, 4, 5]);
        Ok(())
    }

    #[test]
    fn test_gcd() -> anyhow::Result<()> {
        anyhow::ensure!(12.gcd(15) == 3);
        anyhow::ensure!(15.gcd(12) == 3);
        Ok(())
    }

    #[test]
    fn test_lcm() -> anyhow::Result<()> {
        anyhow::ensure!(12.lcm(15) == 60);
        anyhow::ensure!(15.lcm(12) == 60);
        Ok(())
    }

    #[test]
    fn test_prime_factors() -> anyhow::Result<()> {
        anyhow::ensure!(12.prime_factors() == vec![2, 2, 3]);
        anyhow::ensure!(15.prime_factors() == vec![3, 5]);
        Ok(())
    }

    #[test]
    fn test_proper_divisors() -> anyhow::Result<()> {
        anyhow::ensure!(
            28_i32.proper_divisors().sorted().collect::<Vec<_>>() == vec![1, 2, 4, 7, 14]
        );
        anyhow::ensure!(
            220_i32.proper_divisors().sorted().collect::<Vec<_>>()
                == vec![1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110]
        );
        Ok(())
    }
}
