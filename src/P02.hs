{-# LANGUAGE NumericUnderscores #-}

module P02 (main) where

main :: Int
main =
    sum
        . filter even
        . takeWhile (<= 4_000_000)
        $ fibs
  where
    fibs = 1 : 2 : zipWith (+) fibs (tail fibs)