module P32 (pandigitalProducts) where

import Data.List (nub, permutations)
import Data.Tuple.Extra (thd3)

pandigitalProducts :: Integer
pandigitalProducts =
    let f = read . concatMap show
     in sum
            . nub
            $ ( \p ->
                    map thd3
                        . filter (\(a, b, c) -> a * b == c)
                        $ map
                            (\(a, b, c) -> (f a, f b, f c))
                            [ (take 2 p, take 3 (drop 2 p), take 4 (drop 5 p))
                            , (take 1 p, take 4 (drop 1 p), take 4 (drop 5 p))
                            ]
              )
                =<< permutations [1 :: Int .. 9]