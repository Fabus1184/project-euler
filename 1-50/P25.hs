module P25 (oneThousandDigitFibonacciNumber) where

import Data.List (findIndex)
import Data.Maybe (fromJust)
import MyLib (fibs)

oneThousandDigitFibonacciNumber :: Integer
oneThousandDigitFibonacciNumber = toInteger . fromJust . findIndex ((>= 1000) . length . show) $ fibs
