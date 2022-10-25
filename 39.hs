import Data.List (groupBy, maximumBy, sortOn, nubBy, sort)
import Data.Ord (comparing)

triangles :: [(Integer, Integer, Integer)]
triangles = nubBy (\(a, b, c) (d, e, f) -> sort [a, b, c] == sort [d, e, f])
            [(a, b, c) | a <- [1 .. 500], b <- [1 .. 500 - a], let c = round . sqrt . fromIntegral $ a^2 + b^2, a^2 + b^2 == c^2]

main :: IO ()
main = print . fst . maximumBy (comparing snd) . map (\((x, _):xs) -> (x, length xs)) .
    groupBy (\a b -> fst a == fst b) . sortOn fst . filter ((<= 1000) . fst) . map (\(a, b, c) -> (a + b + c, (a, b, c))) $ triangles