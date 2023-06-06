module Lib
  ( someFunc,
    eqLet,
    eqWhere,
    sumValues,
    unwrapString,
    isOdd,
    isOddIf,
    isEven,
  )
where

someFunc :: IO ()
someFunc = putStrLn "someFunc"

eqLet :: Bool
eqLet =
  let one = True
      two = True
   in one && two

eqWhere :: Bool
eqWhere = one && two
  where
    one = True
    two = False

sumValues :: [Int] -> Int
sumValues [] = 0
sumValues (x : xs) = x + sumValues xs

unwrapString :: Maybe a -> a
unwrapString s = case s of
  Just x -> x
  Nothing -> error "no string"

-- unwrapString :: Maybe a -> a
-- unwrapString (Just x) = x
-- unwrapString Nothing = error "no string"

isOdd :: Int -> Bool
isOdd n
  | n `mod` 2 == 0 = False
  | otherwise = True

isEven :: Int -> Bool
isEven n = n `mod` 2 == 0

isOddIf :: Int -> Bool
isOddIf n =
  if n `mod` 2 == 0
    then False
    else True
