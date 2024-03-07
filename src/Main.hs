{-# LANGUAGE BangPatterns #-}
{-# LANGUAGE LambdaCase #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE TemplateHaskell #-}

module Main (main) where

import Data.Maybe (fromMaybe)
import Formatting (fixed, formatToString, int, string, (%))
import GHC.Clock (getMonotonicTime)
import System.Environment (getArgs)
import Text.Read (readMaybe)

import Lib (IsSolution (IsSolution), Solution (..), mkSolutions)

import qualified P01
import qualified P02
import qualified P03
import qualified P04
import qualified P05
import qualified P06
import qualified P07
import qualified P08
import qualified P09
import qualified P10

solutions :: [(Int, IsSolution)]
solutions = $(mkSolutions)

main :: IO ()
main = do
    getArgs >>= \case
        ["all"] -> mapM_ (f . fst) solutions
        [x] -> f $ fromMaybe (error "failed to parse argument") (readMaybe x)
        _ -> putStrLn "Usage: euler <problem number>"
  where
    f n = case lookup n solutions of
        Just (IsSolution solution) -> do
            time <- getMonotonicTime
            !s <- toString solution
            time' <- getMonotonicTime
            putStrLn $ formatToString ("Problem " % int % ": " % string % " (" % fixed 2 % "s)") n s (time' - time)
        Nothing -> putStrLn $ "No such solution: " <> show n