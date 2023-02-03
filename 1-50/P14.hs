{-# LANGUAGE TupleSections #-}

module P14 (longestCollatzSequence) where

import Control.Monad.Trans.State (State, evalState, get, modify)
import Data.List (maximumBy)
import Data.Map (Map, empty, insert, lookup)
import Data.Ord (comparing)
import Prelude hiding (lookup)

collatzLength :: Integral a => a -> State (Map a a) a
collatzLength 1 = pure 0
collatzLength n = do
    cache <- get
    let calculate = succ <$> if even n then collatzLength (n `div` 2) else collatzLength (succ $ 3 * n)
    if n < 35 * 10 ^ 4
        then case lookup n cache of
            Just l -> pure l
            Nothing -> do
                l <- calculate
                modify (insert n l)
                pure l
        else calculate

longestCollatzSequence :: Integer
longestCollatzSequence = fst . flip evalState empty $ do
    cll <- mapM (\x -> (x,) <$> collatzLength x) [1 .. 10 ^ 6]
    pure $ maximumBy (comparing snd) cll
