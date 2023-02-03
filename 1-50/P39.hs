module P39 (integerRightTriangles) where

import Data.List (groupBy, maximumBy, nubBy, sort, sortOn)
import Data.Ord (comparing)

triangles :: Integral a => [(a, a, a)]
triangles =
    nubBy
        (\(a, b, c) (d, e, f) -> sort [a, b, c] == sort [d, e, f])
        [(a, b, c) | a <- [1 .. 500], b <- [1 .. 500 - a], let c = round . sqrt . fromIntegral $ a ^ 2 + b ^ 2, a ^ 2 + b ^ 2 == c ^ 2]

integerRightTriangles :: Integer
integerRightTriangles =
    fst
        . maximumBy (comparing snd)
        . map (\((x, _) : xs) -> (x, length xs))
        . groupBy (\a b -> fst a == fst b)
        . sortOn fst
        . filter ((<= 1000) . fst)
        . map (\(a, b, c) -> (a + b + c, (a, b, c)))
        $ triangles