{-# LANGUAGE TemplateHaskellQuotes #-}

module Lib (
    embedHttp,
) where

import Control.Lens ((^.))
import Data.ByteString.Lazy.Char8 (unpack)
import Language.Haskell.TH (Exp, Q, runIO)
import Network.Wreq (get, responseBody)

embedHttp :: String -> Q Exp
embedHttp url = do
    response <- unpack . (^. responseBody) <$> runIO (get url)
    [|response|]
