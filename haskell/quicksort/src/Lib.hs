module Lib (
  someFunc,
) where

someFunc :: IO ()
someFunc = print $ q [9, 3, 5, 1, 8]

q :: [Int] -> [Int]
q [] = []
q (x : xs) = q left ++ [x] ++ q right
 where
  left = [l | l <- xs, l <= x]
  right = [r | r <- xs, r > x]
