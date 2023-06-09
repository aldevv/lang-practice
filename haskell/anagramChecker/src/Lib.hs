module Lib
  ( someFunc,
  )
where

someFunc :: IO ()
someFunc = print $ anagramCheck "ana"

myReverse :: String -> String
myReverse "" = ""
myReverse (x : xs) = myReverse xs ++ [x]

anagramCheck :: String -> Bool
anagramCheck "" = True
anagramCheck s = myReverse s == s
