module P34 (digitFactorials) where

import Data.List.Extra (nubOrd)

factorial :: (Eq a, Num a, Enum a) => a -> a
factorial 0 = 1
factorial 1 = 1
factorial n = product [1 .. n]

allCombinations :: [[Int]]
allCombinations = filter ((/= 0) . head) $ allCombinations' limit
  where
    limit :: Int
    limit = head $ dropWhile (\n -> n * factorial 9 >= 10 ^ n) [1 ..]

    allCombinations' :: Int -> [[Int]]
    allCombinations' 2 = [[x] | x <- [0 .. 9]] ++ [[x, y] | x <- [0 .. 9], y <- [0 .. 9]]
    allCombinations' n =
        let as = allCombinations' (n - 1)
         in as ++ [x : xs | x <- [0 .. 9], xs <- as]

fromDigits :: [Int] -> Int
fromDigits = foldl (\acc x -> acc * 10 + x) 0

digitFactorials :: Int
digitFactorials =
    sum
        . nubOrd
        . map fst
        . filter (\(s, fs) -> sum (map factorial fs) == s)
        . map (fromDigits >>= (,))
        $ allCombinations