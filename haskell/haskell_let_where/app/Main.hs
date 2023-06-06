module Main (main) where

import Lib

main :: IO ()
main = do
  someFunc
  print eqLet
  print eqWhere
  print $ sumValues [1, 2, 3, 4, 5]
  print $ unwrapString $ Just "my wrapped string"
  print $ isEven 10
  print $ isOdd 10
  print $ isOddIf 10
