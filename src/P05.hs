module P05 (main) where

main :: Int
main = foldl lcm 1 [1 .. 20]