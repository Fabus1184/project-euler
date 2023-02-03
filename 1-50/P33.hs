module P33 (digitCancellingFractions) where

import Control.Monad (guard)
import Data.List (delete, intersect)
import Data.Ratio (denominator, (%))

fractions :: (Enum a, Num a) => [(a, a)]
fractions = [(a, b) | b <- [10 .. 99], a <- [10 .. b]]

digits :: Integral a => a -> [a]
digits 0 = []
digits n = digits (n `div` 10) ++ [n `mod` 10]

fromDigits :: Num a => [a] -> a
fromDigits = foldl (\a b -> a * 10 + b) 0

wrongCancel :: (Integer, Integer) -> [Rational]
wrongCancel (n, d) = do
    d' <- filter (/= 0) $ digits n `intersect` digits d
    let nm = fromDigits $ delete d' $ digits n
    let dn = fromDigits $ delete d' $ digits d
    guard $ dn /= 0
    [nm % dn]

digitCancellingFractions :: Integer
digitCancellingFractions =
    denominator
        . product
        . filter (< 1)
        . map (uncurry (%))
        . filter (\r@(a, b) -> elem (a % b) $ wrongCancel r)
        $ fractions
