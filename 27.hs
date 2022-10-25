import Data.Numbers.Primes (primes)
import Data.List (maximumBy)
import Data.Ord (comparing)

formula :: Integer -> Integer -> (Integer -> Integer)
formula a b = \n -> n ^ 2 + (a * n) + b

main :: IO ()
main = print . (\(a, b) -> a * b) . fst . maximumBy (comparing (\(_, x) -> x)) . map (\x -> (x, (f x))) $
    [(a, b) | a <- [-1000 .. 1000], b <- [-1000 .. 1000]]
    where
        f (a, b) = length . takeWhile (\x -> x `elem` (primes' x)) $ zipWith ($) (repeat $ formula a b) [0 .. ]
        primes' x = takeWhile (<= x) primes
