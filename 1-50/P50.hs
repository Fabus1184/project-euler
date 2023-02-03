module P50 (consecutivePrimeSum) where

import Debug.Trace ()
import MyLib (isPrime, primes)

consecutive :: [a] -> Int -> [[a]]
consecutive xs 0 = [xs]
consecutive [] _ = []
consecutive xs n
    | length_greater xs n = take n xs : consecutive (tail xs) n
    | otherwise = []
  where
    length_greater xs n = length (take n xs) == n

limit :: Integral a => a
limit = 10 ^ 6

consecutivePrimeSum :: Integer
consecutivePrimeSum =
    let conslimit = last $ takeWhile (\n -> sum (take n primes) < limit) [1 ..]
     in head $
            concatMap
                ( filter isPrime
                    . takeWhile (< limit)
                    . map sum
                    . consecutive primes
                )
                [conslimit, conslimit - 1 .. 2]
