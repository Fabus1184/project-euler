{-# LANGUAGE TypeApplications #-}

module P24 (lexicographicPermutations) where

import Data.List (permutations, sort)

lexicographicPermutations :: String
lexicographicPermutations = concatMap (show @Integer) $ (sort . permutations $ [0 .. 9]) !! pred (10 ^ (6 :: Int))