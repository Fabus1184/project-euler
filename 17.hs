import Data.Char (isAlpha)

spell :: Integer -> String
spell 0 = "zero"
spell 1 = "one"
spell 2 = "two"
spell 3 = "three"
spell 4 = "four"
spell 5 = "five"
spell 6 = "six"
spell 7 = "seven"
spell 8 = "eight"
spell 9 = "nine"
spell 10 = "ten"
spell 11 = "eleven"
spell 12 = "twelve"
spell 13 = "thirteen"
spell 14 = "fourteen"
spell 15 = "fifteen"
spell 16 = "sixteen"
spell 17 = "seventeen"
spell 18 = "eighteen"
spell 19 = "nineteen"
spell 20 = "twenty"
spell 30 = "thirty"
spell 40 = "forty"
spell 50 = "fifty"
spell 60 = "sixty"
spell 70 = "seventy"
spell 80 = "eighty"
spell 90 = "ninety"
spell n
    | n < 100 = spell (n - n `mod` 10) ++ "-" ++ spell (n `mod` 10)
    | n >= 100 && n < 1000 = spell (n `div` 100) ++ " hundred" ++ if n `mod` 100 == 0 then "" else " and " ++ spell (n `mod` 100)
    | n >= 1000 = spell (n `div` 1000) ++ " thousand" ++ if n `mod` 1000 == 0 then "" else " " ++ spell (n `mod` 1000)

count :: String -> Integer
count = toInteger . length . filter isAlpha

main :: IO ()
main = print . count . mconcat . map spell $ [1 .. 1000]