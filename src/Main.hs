module Main (main) where

import Control.Monad ((>=>))
import Data.List (uncons)
import Data.Maybe (fromMaybe)
import System.Environment (getArgs)
import Text.Read (readMaybe)

main :: IO ()
main = do
    n <-
        fromMaybe
            (error "failed to parse argument")
            . (uncons >=> readMaybe . fst)
            <$> getArgs ::
            IO Int
    print n