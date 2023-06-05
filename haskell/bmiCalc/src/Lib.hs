module Lib
  ( someFunc,
  )
where

import Text.Read (readMaybe)

someFunc :: IO ()
someFunc =
  putStrLn "Enter weight: "
    >> getLine
    >>= \weight ->
      putStrLn "Enter height: "
        >> getLine
        >>= \height ->
          let fweight = stringToFloat weight
              fheight = stringToFloat height
              bmi = calcBMI fweight fheight
           in print bmi

calcBMI :: Float -> Float -> Float
calcBMI weight height = (weight / height / height) * 10000

stringToFloat :: String -> Float
stringToFloat s = case readMaybe s of
  Just x -> x
  Nothing -> error "can't convert value to float"
