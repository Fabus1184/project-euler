{-# LANGUAGE TypeApplications #-}

module P20 (factorialDigitSum) where

factorialDigitSum :: Integer
factorialDigitSum = sum . map (read . (: [])) . show @Integer . product $ [1 .. 100]
