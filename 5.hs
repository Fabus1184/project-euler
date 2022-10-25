main :: IO ()
main = print . head $ filter (\x -> and [x `mod` y == 0 | y <- [2 .. 20]]) [1 .. ]
