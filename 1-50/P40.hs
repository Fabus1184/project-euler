module P40 (champernownesConstant) where

digits :: (Read a, Show a, Integral a) => [a]
digits = map (read . (: [])) . mconcat $ map show [1 :: Int ..]

champernownesConstant :: Integer
champernownesConstant = product $ map ((digits !!) . pred . (10 ^)) [0 :: Int .. 6]
