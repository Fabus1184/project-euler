import Data.List (permutations, sort)

main :: IO ()
main = putStrLn . concatMap show $ (sort . permutations $ [0 .. 9]) !! pred (10^6)