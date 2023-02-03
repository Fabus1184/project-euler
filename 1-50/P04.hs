module P04 (largestPalindromeProduct) where

import MyLib (isPalindrome)

largestPalindromeProduct :: Integer
largestPalindromeProduct = maximum $ [x * y | x <- [100 .. 999], y <- [100 .. 999], isPalindrome (show $ x * y)]
