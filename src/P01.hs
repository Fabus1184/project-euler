module P01 (main) where

main :: Int
main =
    sum
        . filter (\x -> x `mod` 3 == 0 || x `mod` 5 == 0)
        $ [1 .. 999]