main :: IO ()
main = do
    let s1 = sum $ map (^ 2) [1 .. 100]
    let s2 = (^ 2) $ sum [1 .. 100]
    print $ s2 - s1
