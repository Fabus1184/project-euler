module P27 (quadraticPrimes) where

import Data.List (maximumBy)
import Data.Numbers.Primes (isPrime, primes)
import Data.Ord (comparing)

formula :: Num a => a -> a -> a -> a
formula a b n = n ^ 2 + a * n + b

quadraticPrimes :: Integer
quadraticPrimes = uncurry (*) . fst . maximumBy (comparing snd) . map (\x -> (x, f x)) $ [(a, b) | a <- [-1000 .. 1000], b <- [-1000 .. 1000]]
 where
  f (a, b) = length . takeWhile isPrime $ map (formula a b) [0 ..]
  primes' x = takeWhile (<= x) primes
