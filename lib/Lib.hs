{-# LANGUAGE ExistentialQuantification #-}
{-# LANGUAGE FlexibleInstances #-}
{-# LANGUAGE InstanceSigs #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE TemplateHaskellQuotes #-}
{-# LANGUAGE TupleSections #-}
{-# LANGUAGE UndecidableInstances #-}

module Lib (
    embedHttp,
    Solution (..),
    IsSolution (..),
    mkSolutions,
) where

import Control.Lens ((^.))
import Control.Monad.Extra (mapMaybeM)
import Data.ByteString.Lazy.Char8 (unpack)
import Data.Functor ((<&>))
import Formatting (formatToString, int, lpadded, (%))
import Language.Haskell.TH (Exp, Q, listE, lookupValueName, runIO, varE)
import Network.Wreq (get, responseBody)

embedHttp :: String -> Q Exp
embedHttp url = do
    response <- unpack . (^. responseBody) <$> runIO (get url)
    [|response|]

class Solution a where
    toString :: a -> IO String

instance (Integral a) => Solution a where
    toString :: a -> IO String
    toString = pure . show . toInteger

instance Solution String where
    toString :: String -> IO String
    toString = pure

instance (Solution a) => Solution (IO a) where
    toString :: IO a -> IO String
    toString = (toString =<<)

data IsSolution = forall a. (Solution a) => IsSolution a

-- [(Int, IsSolution)]
mkSolutions :: Q Exp
mkSolutions = do
    names <-
        mapMaybeM
            ( (\(a, b) -> a <&> (>>= pure . (b,)))
                . ((lookupValueName . formatToString ("P" % lpadded 2 '0' int % ".main")) >>= (,))
            )
            [1 :: Int .. 999]
    listE $ map (\(a, b) -> let b' = varE b in [|(a, IsSolution $b')|]) names