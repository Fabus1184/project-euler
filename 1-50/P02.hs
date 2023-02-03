module P02 (evenFibonacciNumbers) where

import MyLib (fibs)

evenFibonacciNumbers :: Integer
evenFibonacciNumbers = sum . filter ((== 0) . (`mod` 2)) . takeWhile (<= 4 * 10 ^ 6) $ fibs
