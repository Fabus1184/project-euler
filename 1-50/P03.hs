module P03 (largestPrimeFactor) where

import MyLib (primeFactors)

largestPrimeFactor :: Integer
largestPrimeFactor = maximum $ primeFactors 600851475143
