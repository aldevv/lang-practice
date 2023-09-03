module Lib (
  someFunc,
) where

someFunc :: IO ()
someFunc = do
  print "Enter age"
  ageS <- getLine
  print "Enter resting heart rate"
  restHRS <- getLine
  let age = read ageS :: Int
  let restHR = read restHRS :: Int
  print "Intensity  |   Rate"
  calcHRRange age restHR 55 95 5

targetHeartRate :: Int -> Int -> Double -> Double
targetHeartRate age rhr intensity = (fromIntegral ((220 - age) - rhr) * intensity) + fromIntegral rhr

calcHRRange :: Int -> Int -> Int -> Int -> Int -> IO ()
calcHRRange age rhr _min _max rate = do
  let res = round $ targetHeartRate age rhr (fromIntegral _min / 100)
  print (show _min ++ "%  -  " ++ show res ++ " bpm")
  if _min /= _max
    then calcHRRange age rhr (_min + rate) _max rate
    else return ()
