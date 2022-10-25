series :: [Integer]
series = map (\n -> n ^ n) [1 ..]

main :: IO ()
main = putStrLn . reverse . take 10 . reverse . show . sum $ take 1000 series
