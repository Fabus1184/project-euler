toBinary :: Integer -> Integer
toBinary x = read . mconcat . map show . reverse $ toBinary' x
    where
        toBinary' 0 = []
        toBinary' y = let (a, b) = quotRem y 2 in [b] ++ toBinary' a
        
isPalindrome :: Integer -> Bool
isPalindrome x = show x == reverse (show x)

main :: IO ()
main = print . sum $ filter (\x -> isPalindrome x && isPalindrome (toBinary x)) [1 .. 999999]
