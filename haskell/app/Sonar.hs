module Sonar(numberIncreasing) where 

numberIncreasing :: [Int] -> Int
numberIncreasing (x:(y:rest)) 
   | x < y = 1 + numberIncreasing (y:rest)
   | otherwise = numberIncreasing (y:rest)
numberIncreasing _ = 0