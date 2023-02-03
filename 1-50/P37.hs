module P37 (truncatablePrimes) where

import MyLib (isPrime, primes)

isLeftTruncatable :: Integer -> Bool
isLeftTruncatable n =
    all (isPrime . read)
        . take (length $ show n)
        . iterate tail
        . show
        $ n

isRightTruncatable :: Integer -> Bool
isRightTruncatable n =
    all (isPrime . read)
        . take (length $ show n)
        . iterate init
        . show
        $ n

truncatablePrimes :: Integer
truncatablePrimes =
    sum
        . take 11
        . filter (\x -> isLeftTruncatable x && isRightTruncatable x)
        . drop 4
        $ primes