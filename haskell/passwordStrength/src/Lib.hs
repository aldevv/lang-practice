module Lib
  ( someFunc,
  )
where

someFunc :: IO ()
someFunc = getLine >>= \line -> print $ passwordStrength line

data StrengthLevel = VeryWeak | Weak | Medium | Strong | VeryStrong | Unknown deriving (Show, Eq)

passwordStrength :: String -> StrengthLevel
passwordStrength s
  | onlyDigits s = VeryWeak
  | onlyLetters s && myLength s < 8 = Weak
  | alphaCond && not (containsSpecial s) = Strong
  | alphaCond && containsSpecial s = VeryStrong
  | myLength s >= 8 = Medium
  | otherwise = Weak
  where
    alphaCond = myLength s >= 8 && containsLetter s && containsNumber s

myLength :: String -> Int
myLength "" = 0
myLength (_ : xs) = 1 + myLength xs

onlyDigits :: String -> Bool
onlyDigits "" = True
onlyDigits (x : xs)
  | x `elem` ['0' .. '9'] = onlyDigits xs
  | otherwise = False

onlyLetters :: String -> Bool
onlyLetters "" = True
onlyLetters (x : xs)
  | x `elem` ['a' .. 'z'] || x `elem` ['A' .. 'Z'] = onlyLetters xs
  | otherwise = False

containsSpecial :: String -> Bool
containsSpecial "" = False
containsSpecial (x : xs)
  | x `elem` ['!', '"', '#', '$', '%', '&', '/', '(', ')', '`', '='] = True
  | otherwise = containsSpecial xs

containsNumber :: String -> Bool
containsNumber "" = False
containsNumber (x : xs)
  | x `elem` ['0' .. '9'] = True
  | otherwise = containsNumber xs

containsLetter :: String -> Bool
containsLetter "" = False
containsLetter (x : xs)
  | x `elem` ['a' .. 'z'] || x `elem` ['A' .. 'Z'] = True
  | otherwise = containsNumber xs
