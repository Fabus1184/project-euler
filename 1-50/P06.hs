module P06 (sumSquareDifference) where

sumSquareDifference :: Integer
sumSquareDifference =
    let s1 = sum $ map (^ (2 :: Int)) [1 .. 100]
        s2 = (^ (2 :: Int)) $ sum [1 .. 100]
     in s2 - s1
