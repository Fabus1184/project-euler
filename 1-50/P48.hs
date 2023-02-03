{-# LANGUAGE TypeApplications #-}

module P48 (selfPowers) where

series :: Integral a => [a]
series = map (\n -> n ^ n) [1 ..]

selfPowers :: String
selfPowers = reverse . take 10 . reverse . show @Integer . sum $ take 1000 series
