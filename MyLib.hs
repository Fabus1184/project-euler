module MyLib where

import Data.List (subsequences)
import Data.List.Extra (nub)
import qualified Data.Numbers.Primes as P (isPrime, primeFactors, primes)

primes :: Integral a => [a]
primes = P.primes

primeFactors :: Integral a => a -> [a]
primeFactors = P.primeFactors

isPrime :: Integral a => a -> Bool
isPrime = P.isPrime

fibs :: Integral a => [a]
fibs = 0 : 1 : zipWith (+) fibs (tail fibs)

isPalindrome :: Eq a => [a] -> Bool
isPalindrome [] = True
isPalindrome [_] = True
isPalindrome (x : xs) = x == last xs && isPalindrome (init xs)

factorial :: Integral a => a -> a
factorial n = product [2 .. n]

divisors :: Integral a => a -> [a]
divisors = nub . map product . subsequences . primeFactors

nChooseK :: Integral a => a -> a -> a
nChooseK n k = product [2 .. n] `div` (product [1 .. k] * product [1 .. n - k])
