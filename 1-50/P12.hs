module P12 (highlyDivisibleTriangularNumber) where

import Data.List (group)
import MyLib (primeFactors)

triangleNumbers :: [Integer]
triangleNumbers = scanl1 (+) [1 ..]

nDivisors :: Integer -> Int
nDivisors = product . map (succ . length) . group . primeFactors

highlyDivisibleTriangularNumber :: Integer
highlyDivisibleTriangularNumber = head . filter ((> 500) . nDivisors) $ triangleNumbers