main :: IO ()
main = print $ sum $ filter (\x -> 0 `elem` (map (x `mod`) [3, 5])) [1 .. 999]
