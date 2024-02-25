{-# LANGUAGE TemplateHaskell #-}

module Main (main) where

import Control.Exception (assert)
import Lib (embedHttp)

main :: IO ()
main = do
    testEmbedHttp

embed :: String
embed = $(embedHttp "https://pastebin.com/raw/Jpmuyz0t")

testEmbedHttp :: IO ()
testEmbedHttp =
    assert (embed == "test") $
        putStrLn "Test passed!"
