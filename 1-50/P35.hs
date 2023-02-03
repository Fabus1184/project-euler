module P35 (circularPrimes) where

import MyLib (isPrime, primes)

rotate :: [a] -> [a]
rotate (x : xs) = xs ++ [x]

limit :: Integer
limit = 10 ^ 6

primes' :: Integer -> [Integer]
primes' x = takeWhile (< x) primes

digs :: Integral x => x -> [x]
digs 0 = []
digs x = digs (x `div` 10) ++ [x `mod` 10]

fromDigs :: Integral x => [x] -> x
fromDigs = foldl addDigit 0
 where
  addDigit num d = 10 * num + d

circulars :: [Integer]
circulars =
  filter
    ( \x ->
        let dx = digs x
            rs = map fromDigs $ take (length dx) $ iterate rotate dx
         in all isPrime rs
    )
    $ filter preselect (primes' limit)
 where
  preselect x = all (`notElem` digs x) [2, 4, 6, 8] || x < 10

circularPrimes :: Integer
circularPrimes = toInteger $ length circulars
