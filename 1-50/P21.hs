module P21 (amicableNumbers) where

import MyLib (divisors)

amicables :: Integral a => [a]
amicables =
    filter
        ( \n ->
            let s = sum (divisors n)
                t = sum (divisors s)
             in t == n && n /= s
        )
        [1 .. 10000]

amicableNumbers :: Integer
amicableNumbers = sum amicables
