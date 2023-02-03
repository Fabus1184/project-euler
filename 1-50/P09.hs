{-# LANGUAGE TypeApplications #-}

module P09 (specialPythagoreanTriplet) where

specialPythagoreanTriplet :: Integer
specialPythagoreanTriplet = head [a * b * c | b <- [1 ..], a <- [1 .. b], let c = round @Double . sqrt . fromIntegral $ a ^ 2 + b ^ 2, a ^ 2 + b ^ 2 == c ^ 2, a + b + c == 1000]
