{-# LANGUAGE ExistentialQuantification #-}
{-# LANGUAGE InstanceSigs #-}
{-# LANGUAGE TemplateHaskell #-}

module Main (main) where

import Control.Monad (when)
import Data.Char (isNumber, toUpper)
import Data.Tuple.Extra (thd3)
import Language.Haskell.TH (Exp (ListE), appE, stringE, tupE, varE)
import Language.Haskell.TH.Syntax (showName)
import System.Environment (getArgs)
import System.TimeIt (timeIt)

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
import qualified P11
import qualified P12
import qualified P13
import qualified P14
import qualified P15
import qualified P16
import qualified P17
import qualified P18
import qualified P19
import qualified P20
import qualified P21
import qualified P22
import qualified P23
import qualified P24
import qualified P25
import qualified P26
import qualified P27
import qualified P28
import qualified P29
import qualified P30
import qualified P31
import qualified P32
import qualified P33
import qualified P34
import qualified P35
import qualified P36
import qualified P37
import qualified P39
import qualified P40
import qualified P41
import qualified P42
import qualified P47
import qualified P48
import qualified P50
import qualified P53
import qualified P54

data Showable = forall a. Show a => MkShowable a

instance Show Showable where
    show :: Showable -> String
    show (MkShowable a) = show a

pack :: Show a => a -> Showable
pack = MkShowable

problems :: [(String, Showable, Int)]
problems =
    $( ListE
        <$> mapM
            (\n -> tupE [stringE (showName n), appE (varE 'pack) (varE n), appE (varE 'parseProblemNumber) (stringE (showName n))])
            [ 'P01.multiplesOf3Or5
            , 'P02.evenFibonacciNumbers
            , 'P03.largestPrimeFactor
            , 'P04.largestPalindromeProduct
            , 'P05.smallestMultiple
            , 'P06.sumSquareDifference
            , 'P07.oneThousandAndFirstPrime
            , 'P08.largestProductInASeries
            , 'P09.specialPythagoreanTriplet
            , 'P10.summationOfPrimes
            , 'P11.largestProductInAGrid
            , 'P12.highlyDivisibleTriangularNumber
            , 'P13.largeSum
            , 'P14.longestCollatzSequence
            , 'P15.latticePaths
            , 'P16.powerDigitSum
            , 'P17.numberLetterCounts
            , 'P18.maximumPathSum1
            , 'P19.countingSundays
            , 'P20.factorialDigitSum
            , 'P21.amicableNumbers
            , 'P22.namesScores
            , 'P23.nonAbundantSums
            , 'P24.lexicographicPermutations
            , 'P25.oneThousandDigitFibonacciNumber
            , 'P26.reciprocalCycles
            , 'P27.quadraticPrimes
            , 'P28.numberSpiralDiagonals
            , 'P29.distinctPowers
            , 'P30.digitFifthPowers
            , 'P31.coinSums
            , 'P32.pandigitalProducts
            , 'P33.digitCancellingFractions
            , 'P34.digitFactorials
            , 'P35.circularPrimes
            , 'P36.doubleBasePalindromes
            , 'P37.truncatablePrimes
            , 'P39.integerRightTriangles
            , 'P40.champernownesConstant
            , 'P41.pandigitalPrime
            , 'P42.codedTriangleNumbers
            , 'P47.distinctPrimesFactors
            , 'P48.selfPowers
            , 'P50.consecutivePrimeSum
            , 'P53.combinatoricSelections
            , 'P54.pokerHands
            ]
     )
  where
    parseProblemNumber :: String -> Int
    parseProblemNumber ('P' : xs) = read $ takeWhile isNumber xs
    parseProblemNumber _ = error "Invalid property"

capitalize :: String -> String
capitalize [] = []
capitalize (x : xs) = toUpper x : xs

main :: IO ()
main = do
    as <- (read <$>) <$> getArgs :: IO [Int]
    when (any (`notElem` map thd3 problems) as) $ error "Invalid number"
    mapM_
        ( \(p, n, k) -> do
            putStrLn $ "Problem " ++ show k ++ ": " ++ (capitalize . tail . dropWhile isNumber . tail) p
            timeIt (print n)
        )
        . filter (\(_, _, n) -> null as || n `elem` as)
        $ problems
