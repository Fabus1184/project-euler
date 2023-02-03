module P05 (smallestMultiple) where

smallestMultiple :: Integer
smallestMultiple = head . filter (\x -> all ((== 0) . (x `mod`)) [20, 19 .. 2]) $ [20, 40 ..]
