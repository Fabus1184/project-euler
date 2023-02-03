module P41 (pandigitalPrime) where

import Data.List (inits, permutations, sort)
import MyLib (isPrime)

pandigits :: (Read a, Show a, Integral a) => [a]
pandigits = reverse . sort . map (read . mconcat . map show) . concatMap permutations . tail $ inits [1 :: Int .. 9]

pandigitalPrime :: Integer
pandigitalPrime = head . filter isPrime $ pandigits