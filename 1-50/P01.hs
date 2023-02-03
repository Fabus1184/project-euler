module P01 (multiplesOf3Or5) where

multiplesOf3Or5 :: Integer
multiplesOf3Or5 = sum . concat $ [[0, 3 .. 999], [0, 5 .. 999], [0, -15 .. -999]]