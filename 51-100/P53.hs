module P53 (combinatoricSelections) where

import Data.List (genericLength)
import MyLib (nChooseK)

combinatoricSelections :: Integer
combinatoricSelections = genericLength . filter (> 10 ^ 6) $ [nChooseK n k | n <- [1 .. 100], k <- [1 .. n]]