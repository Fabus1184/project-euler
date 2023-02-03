-- Attention: This file is not part of the package, it is a standalone module for information purposes only.

module Missing (main) where

import Control.Monad (filterM)
import Data.Char (isNumber)
import Data.Functor ((<&>))
import Data.List (isSuffixOf, sort)
import System.Directory (doesDirectoryExist, listDirectory)

parseProblemNumber :: String -> Int
parseProblemNumber s = read $ takeWhile isNumber (tail s)

main :: IO ()
main = do
    dirs <- listDirectory "." >>= filterM doesDirectoryExist >>= mapM listDirectory <&> sort . map parseProblemNumber . filter (".hs" `isSuffixOf`) . concat
    putStrLn $ (tail . init) (show . concatMap (\(a, b) -> [succ a .. pred b]) $ zip dirs (tail dirs)) ++ "," ++ (show $ succ (last dirs)) ++ "..."