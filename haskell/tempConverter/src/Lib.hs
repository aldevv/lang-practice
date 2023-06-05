module Lib
  ( someFunc,
  )
where

import Text.Read (readMaybe)

someFunc :: IO ()
someFunc = do
  putStrLn "enter the temperature in celcius"
  f <- getLine
  let parsedC = stringToFloat f
  let temperature = toFaranheit parsedC
  print temperature

toFaranheit :: Float -> Float
toFaranheit celcius = (celcius * (9 / 5)) + 32

toCelcius :: Float -> Float
toCelcius fahrenheit = (fahrenheit - 32) * (5 / 9)

stringToFloat :: String -> Float
stringToFloat myString =
  case readMaybe myString of
    Just x -> x
    Nothing -> error "invalid float entered"
