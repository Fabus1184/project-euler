fibs :: [Integer]
fibs = 0 : 1 : zipWith (+) fibs (tail fibs)

main :: IO ()
main = print $ sum $ filter ((== 0) . (`mod` 2)) $ takeWhile (<= 4 * 10^6) fibs
