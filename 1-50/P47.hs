module P47 (distinctPrimesFactors) where

import Data.List (group, sort)
import MyLib (primeFactors)

distinctPrimesFactors :: Integer
distinctPrimesFactors =
    head
        . fst
        . head
        . filter ((== 1) . maximum . map length . group . sort . concat . snd)
        . filter (all ((== 4) . length) . snd)
        . map ((\xs -> (xs, map (map (\x -> head x ^ length x) . group . primeFactors) xs)) . take 4 . iterate succ)
        $ [1 ..]
