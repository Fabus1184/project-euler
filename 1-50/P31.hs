module P31 (coinSums) where

import Numeric.AD (maclaurin)

coins :: Num a => [a]
coins = [1, 2, 5, 10, 20, 50, 100, 200]

f :: Fractional a => a -> a
f x = recip $ product (map (\c -> 1 - (x ^^ c)) coins)

coinSums :: Integer
coinSums = round (maclaurin f 1 !! 200 :: Rational)
