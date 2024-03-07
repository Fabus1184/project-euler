module P06 (main) where

-- https://oeis.org/A052149
f :: Int -> Int
f x = x * (x - 1) * (x + 1) * (3 * x + 2) `div` 12

main :: Int
main = f 100
