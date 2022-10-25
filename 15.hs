factorial :: Integer -> Integer
factorial n = product [2 .. n]

main :: IO ()
main = print . (\n -> factorial (2 * n) `div` (factorial n) ^ 2) $ 20
