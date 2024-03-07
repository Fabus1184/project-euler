module P03 (main) where

factorize :: Int -> [Int]
factorize n = factorize' n 2
  where
    factorize' 1 _ = []
    factorize' n' f
        | n' `mod` f == 0 = f : factorize' (n' `div` f) f
        | otherwise = factorize' n' (f + 1)

main :: Int
main = maximum $ factorize 600851475143