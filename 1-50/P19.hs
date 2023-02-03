module P19 (countingSundays) where

import Data.Time (Day, DayOfWeek (Sunday), addGregorianMonthsClip, dayOfWeek, fromGregorian, toGregorian)

firstsOfMonths :: [Day]
firstsOfMonths = takeWhile ((<= 2000) . year) $ iterate (addGregorianMonthsClip 1) (fromGregorian 1901 1 1)
 where
  year = fromIntegral . (\(y, _, _) -> y) . toGregorian

countingSundays :: Integer
countingSundays = toInteger $ length $ filter ((== Sunday) . dayOfWeek) firstsOfMonths