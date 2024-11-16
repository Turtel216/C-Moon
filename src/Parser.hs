-- Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
-- Use of this source code is governed by a MIT
-- license that can be found in the LICENSE file.

-- | Module: Parser
-- This module defines the parsing functionality for a C compiler. It includes:
--   1. A `Parser` type for handling input strings.
--   2. A `CMoonValue` data type to represent parsed constructs.
module Parser where

import Control.Applicative

-- | The `CMoonValue` type represents different syntactic constructs
-- in a C-like language. It serves as the intermediate representation
-- for parsing results.
data CMoonValue
  = Program
  | FunctionDefinition
  | Function String
  | Statement
  | Expression
  | Constant Int
  deriving (Show, Eq)

-- Pretty Printer for CMoonValues
prettyPrint :: CMoonValue -> String
prettyPrint (Constant n) = show n
prettyPrint (Function name) = "Function: " ++ name
prettyPrint _ = "Other construct"

newtype Parser a = Parser
  { runParser :: String -> Maybe (String, a)
  }

instance Functor Parser where
  fmap f (Parser p) = Parser $ \input -> do
    (input', x) <- p input
    Just (input', f x)

instance Applicative Parser where
  pure x = Parser $ \input -> Just (input, x)
  (Parser p1) <*> (Parser p2) =
    Parser $ \input -> do
      (input', f) <- p1 input
      (input'', a) <- p2 input'
      Just (input'', f a)

instance Alternative Parser where
  empty = Parser $ const Nothing
  (Parser p1) <|> (Parser p2) =
    Parser $ \input -> p1 input <|> p2 input

-- Might be redundant
instance Monad Parser where
  return = pure
  (Parser p1) >>= f = Parser $ \input -> do
    (input', x) <- p1 input
    runParser (f x) input'

-- Monad for maybe something like:
-- parseExample = do
--  a <- someParser
--  b <- anotherParser
--  return (a, b)

someFunc :: IO ()
someFunc = putStrLn "someFunc"
