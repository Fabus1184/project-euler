{-# LANGUAGE NumericUnderscores #-}

module P10 (main) where

import Data.Numbers.Primes (primes)

main :: Int
main = sum $ takeWhile (< 2_000_000) primes
