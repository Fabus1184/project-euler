import Data.Numbers.Primes (primes, primeFactors)
import Data.List (group, sort) 

main :: IO ()
main = print . head . fst . head . filter ((== 1) . maximum . map length . group . sort . concat . snd) . filter (all ((== 4) . length) . snd) . map (\xs -> (xs, map (map (\x -> head x ^ length x) . group . primeFactors) xs)) . map (take 4 . iterate succ) $ [1 ..]
