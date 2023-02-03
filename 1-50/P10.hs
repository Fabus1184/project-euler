module P10 (summationOfPrimes) where

import MyLib (primes)

summationOfPrimes :: Integer
summationOfPrimes = sum . takeWhile (<= 2 * 10 ^ 6) $ primes
