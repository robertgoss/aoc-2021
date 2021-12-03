module Sonar(numberIncreasing, numberIncreasingWindow) where 

import Data.List(tails)

numberIncreasing :: [Int] -> Int
numberIncreasing (x:(y:rest)) 
   | x < y = 1 + numberIncreasing (y:rest)
   | otherwise = numberIncreasing (y:rest)
numberIncreasing _ = 0

numberIncreasingWindow :: [Int] -> Int
numberIncreasingWindow = numberIncreasing . windows
  where windows = map (sum . take 3) . tails