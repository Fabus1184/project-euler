module P28 (numberSpiralDiagonals) where

numberSpiralDiagonals :: Integer
numberSpiralDiagonals = succ $ sum [4 * (n * n) - (6 * n) + 6 | n <- [3, 5 .. 1001]]