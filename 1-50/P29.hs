module P29 (distinctPowers) where

import Data.List (genericLength, nub)

distinctPowers :: Integer
distinctPowers = genericLength . nub $ [a ^ b | a <- [2 .. 100], b <- [2 .. 100]]
