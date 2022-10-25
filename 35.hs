import qualified Data.Numbers.Primes as P

rotate :: String -> String
rotate (x:xs) = xs ++ [x]

limit :: Integer
limit = 10^6

primes :: [Integer]
primes = takeWhile (< limit) P.primes

circulars :: [Integer]
circulars =
    filter (\x -> all (`elem` primes) $ rs x) $ 
        filter preselect primes
    where
        preselect x = let sx = show x in length sx == 1 || all (`notElem` sx) "2468"
        rs x = map read . take (length $ show x) $ iterate rotate (show x)

main :: IO ()
main = print $ length circulars
