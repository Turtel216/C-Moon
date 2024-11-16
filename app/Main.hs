module Main where

import qualified Parser (someFunc)

main :: IO ()
main = do
  putStrLn "Hello, Haskell!"
  Parser.someFunc
