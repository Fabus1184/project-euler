module P09 (main) where

pythagoreanTriplets :: [[Int]]
pythagoreanTriplets =
    [ [a, b, c]
    | m <- [1 ..]
    , n <- [1 .. m - 1]
    , let a = m * m - n * n
    , let b = 2 * m * n
    , let c = m * m + n * n
    ]

main :: Int
main = product . head $ filter ((== 1000) . sum) pythagoreanTriplets