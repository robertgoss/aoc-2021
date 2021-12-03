module Main where

import Sonar

import qualified Data.Text as T
import qualified Data.Text.IO as TI
import qualified Data.Text.Read as TR
import Data.Either.Combinators (swapEither)

readInput :: Int -> IO T.Text
readInput day = TI.readFile path
  where path = "../data/day-" ++ show day ++ ".txt"

type Err = Either String

parseLines :: T.Text -> Err [Int]
parseLines = mapM parseInt . T.lines
  where parseInt :: T.Text -> Err Int
        parseInt = fmap fst . TR.decimal


challenge :: Int -> T.Text -> Err Int
challenge 1 = fmap numberIncreasing . parseLines
challenge _ = const (Left "Unknown challenge")


main :: IO ()
main = do 
    input <- readInput ((num+1) `div` 2)
    let res = challenge num input
    case challenge num input of 
        Right res -> print res
        Left err -> print err
  where num = 1
