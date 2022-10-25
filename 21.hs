import Data.List (groupBy)

amicables :: [Integer]
amicables = filter (\n -> let s = sum (divisors n)
                              t = sum (divisors s)
                          in t == n && n /= s ) $ [1 .. 10000]
    where
        divisors n = filter ((== 0) . (n `mod`)) [1 .. n `div` 2]

main :: IO ()
main = print $ sum amicables
