module P30 (digitFifthPowers) where

digitPower :: (Show a, Read a, Integral a) => a -> a
digitPower = sum . map ((^ (5 :: Int)) . read . (: [])) . show

digitFifthPowers :: Integer
digitFifthPowers = sum $ filter (\x -> x == digitPower x) [2 .. 200000]
