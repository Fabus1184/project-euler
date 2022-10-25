main :: IO ()
main = print . sum . map (\x -> read [x]) . show $ product [2 .. 100]
