isPalindrome :: Int -> Bool
isPalindrome n = show n == reverse (show n)

main :: IO ()
main = print . maximum $ [x * y | x <- [100 .. 999], y <- [100 .. 999], isPalindrome (x * y)]
