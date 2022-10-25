digits :: [Integer]
digits = map (read . (:[])) . mconcat $ map show [1 .. ]

main :: IO ()
main = print . product $ map ((digits !!) . pred . (10 ^)) [0 .. 6]
