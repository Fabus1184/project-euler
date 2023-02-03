module P15 (latticePaths) where

import MyLib (factorial)

latticePaths :: Integer
latticePaths = let n = 20 in factorial (2 * n) `div` factorial n ^ 2
