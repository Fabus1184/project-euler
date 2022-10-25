import Data.List (maximumBy)
import Data.Numbers.Primes (primes)
import Data.Ord (comparing)
import Debug.Trace ()

consecutive :: [Int] -> Int -> [[Int]]
consecutive xs 0 = [xs]
consecutive [] _ = []
consecutive xs n
    | length_greater xs n = take n xs : consecutive (tail xs) n
    | otherwise = []
  where
    length_greater xs n = length (take n xs) == n

limit :: Int
limit = 10 ^ 6

isPrime :: Int -> Bool
isPrime 2 = False
isPrime n = null $ filter (\x -> n `mod` x == 0) [2 .. n `div` 2]

main :: IO ()
main = do
    let conslimit = last $ takeWhile (\n -> sum (take n primes) < limit) [1 ..]
    print . head $
        concatMap
            ( \x ->
                filter isPrime
                    . takeWhile (< limit)
                    . map sum
                    $ consecutive primes x
            )
            [conslimit, conslimit - 1 .. 2]
