import Data.Numbers.Primes (primes)

prime_factors :: Integer -> [Integer]
prime_factors 1 = [1]
prime_factors 2 = [2]
prime_factors n = f : prime_factors (n `div` f)
    where
        f = head $ filter ((== 0) . (n `mod`)) primes

main :: IO ()
main = print $ maximum $ prime_factors 600851475143
