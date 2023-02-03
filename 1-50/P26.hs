module P26 (reciprocalCycles) where

import Data.Foldable (maximumBy)
import Data.Ord (comparing)
import Data.Set (empty, insert, member)

longestCycle :: Integer -> Integer -> Integer
longestCycle a b = longestCycle' a b empty
 where
  longestCycle' n d seen
    | n == 0 = 0
    | n == (r * 10) = 0
    | r `member` seen = 0
    | otherwise = succ $ longestCycle' (r * 10) d (insert r seen)
   where
    (_, r) = n `quotRem` d

reciprocalCycles :: Integer
reciprocalCycles = maximumBy (comparing (longestCycle 1)) [1 .. 999]
