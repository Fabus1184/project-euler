module P04 (main) where

import Control.Arrow ((&&&))

isPalindrome :: Int -> Bool
isPalindrome = uncurry (==) . (show &&& reverse . show)

main :: Int
main = last $ filter isPalindrome [x * y | x <- [100 .. 999], y <- [100 .. 999]]
