main :: IO ()
main = print . sum . map (\x -> read [x]) . show $ 2^1000
