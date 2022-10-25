fibs :: [Integer]
fibs = 1 : 1 : zipWith (+) fibs (tail fibs)

main :: IO ()
main = print . snd . head . filter (\(x, _) -> (>= 1000) . length $ show x) $ zip fibs [1 ..]
