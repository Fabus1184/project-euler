module P07 (oneThousandAndFirstPrime) where

import MyLib (primes)

oneThousandAndFirstPrime :: Integer
oneThousandAndFirstPrime = primes !! 10000
