digitPower :: Integer -> Integer
digitPower = sum . map (^ 5) . map (read . (:[])) . show

main :: IO ()
main = print . sum $ filter (\x -> x == digitPower x) [2 .. 200000]
