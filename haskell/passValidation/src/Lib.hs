module Lib
  ( passValidate,
  )
where

import GHC.IO.FD (openFile)
import GHC.IO.Handle (hClose, hGetContents)
import GHC.IO.IOMode (IOMode (ReadMode))

passValidate :: IO ()
passValidate = do
  file <- loadFile "data/data.txt"
  let passwords = init $ split file
  print "enter your password please: "
  userPass <- getLine
  -- print $ isValidPassword userPass "hola2"
  print $ checkEachPassword userPass passwords

loadFile :: String -> IO String
loadFile s = do
  contents <- readFile s
  return contents

split :: String -> [String]
split [] = [""]
split (c : cs)
  | c == '\n' = "" : rest -- this returns an STRING
  | otherwise = (c : head rest) : tail rest -- this returns an ARRAY
  where
    rest = split cs

checkEachPassword :: String -> [String] -> Bool
checkEachPassword _ [] = False
checkEachPassword userPass (x : xs)
  | userPass == x = True
  | otherwise = checkEachPassword userPass xs
