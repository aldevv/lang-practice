isEven = even

isOdd :: Int -> String
isOdd 0 = "2"

main :: IO ()
main = do
  print $ isOdd 3
