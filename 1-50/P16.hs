{-# LANGUAGE TypeApplications #-}

module P16 (powerDigitSum) where

powerDigitSum :: Integer
powerDigitSum = sum . map (read . (: [])) . show @Integer $ 2 ^ (1000 :: Integer)
