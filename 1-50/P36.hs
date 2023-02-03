module P36 (doubleBasePalindromes) where

import MyLib (isPalindrome)

toBinary :: (Show a, Read a, Integral a) => a -> a
toBinary x = read . mconcat . map show . reverse $ toBinary' x
 where
  toBinary' 0 = []
  toBinary' y = let (a, b) = quotRem y 2 in b : toBinary' a

doubleBasePalindromes :: Integer
doubleBasePalindromes = sum . filter (\x -> isPalindrome (show x) && isPalindrome (show $ toBinary x)) $ [1 .. 999999]
