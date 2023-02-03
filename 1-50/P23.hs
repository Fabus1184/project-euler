module P23 (nonAbundantSums) where

import Data.Set (Set, fromList, notMember)
import MyLib (divisors)

abundantNumbers :: [Integer]
abundantNumbers = filter (\x -> (> x) . sum . tail . init $ divisors x) [2 ..]

abundantSums :: Set Integer
abundantSums = fromList $ [x + y | x <- as, y <- takeWhile (<= x) as] <> [20162 .. 28123]
 where
  as = takeWhile (<= 20161) abundantNumbers

nonAbundantSums :: Integer
nonAbundantSums = sum $ filter (`notMember` abundantSums) [1 .. 28123]