import Data.List (maximumBy)
import Data.Ord (comparing)

collatz :: Integer -> [Integer]
collatz 1 = [1]
collatz n
    | n `mod` 2 == 0 = n : collatz (n `div` 2)
    | otherwise = n : collatz (succ $ 3 * n)

main :: IO ()
main = print . maximumBy (comparing (length . collatz )) $ [1 .. 10^6]
