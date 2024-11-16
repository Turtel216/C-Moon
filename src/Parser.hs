-- Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
-- Use of this source code is governed by a MIT
-- license that can be found in the LICENSE file.

-- | Module: Parser
-- This module defines the parsing functionality for a C compiler. It includes:
--   1. A `Parser` type for handling input strings.
module Parser (test) where

import Control.Applicative hiding (many, some)
import Data.Char (isAlpha, isAlphaNum, isSpace)
import Data.List (isPrefixOf)

-- AST
data Expr
  = Num Int -- A number
  | Add Expr Expr -- Addition
  | Mul Expr Expr -- Multiplication
  | Var String -- A variable
  deriving (Show)

data Stmt
  = Return Expr -- A return statement
  | Block [Stmt] -- A block of statements
  deriving (Show)

data Function
  = Function String [String] Stmt -- Function name, arguments, and body
  deriving (Show)

newtype Parser a = Parser
  { runParser :: String -> [(a, String)]
  }

instance Functor Parser where
  fmap f (Parser p) = Parser $ \input -> [(f result, rest) | (result, rest) <- p input]

instance Applicative Parser where
  pure x = Parser $ \input -> [(x, input)]
  (Parser pf) <*> (Parser px) = Parser $ \input ->
    [(f x, rest2) | (f, rest1) <- pf input, (x, rest2) <- px rest1]

instance Alternative Parser where
  -- Represents a parser that always fails
  empty = Parser $ \_ -> []

  -- Represents a choice between two parsers
  (Parser p1) <|> (Parser p2) = Parser $ \input ->
    case p1 input of
      [] -> p2 input -- If the first parser fails, try the second
      res -> res -- Otherwise, use the result from the first parser

instance Monad Parser where
  (Parser p) >>= f = Parser $ \input ->
    concat [runParser (f result) rest | (result, rest) <- p input]

satisfy :: (Char -> Bool) -> Parser Char
satisfy p = Parser $ \input ->
  case input of
    (x : xs) | p x -> [(x, xs)]
    _ -> []

many :: Parser a -> Parser [a]
many p = some p <|> pure []

some :: Parser a -> Parser [a]
some p = (:) <$> p <*> many p

-- Parse a single character
char :: Char -> Parser Char
char c = Parser $ \input ->
  case input of
    (x : xs) | x == c -> [(c, xs)]
    _ -> []

-- Parse a digit
digit :: Parser Char
digit = Parser $ \input ->
  case input of
    (x : xs) | x >= '0' && x <= '9' -> [(x, xs)]
    _ -> []

-- Parse an integer
integer :: Parser Int
integer = fmap (read :: String -> Int) (some digit)

-- Parse an addition
add :: Parser Expr
add = do
  _ <- char '('
  expr1 <- expr
  _ <- char '+'
  expr2 <- expr
  _ <- char ')'
  return (Add expr1 expr2)

-- Parse a multiplication
mul :: Parser Expr
mul = do
  _ <- char '('
  expr1 <- expr
  _ <- char '*'
  expr2 <- expr
  _ <- char ')'
  return (Mul expr1 expr2)

-- Parse an expression
expr :: Parser Expr
expr = add <|> mul <|> num

-- Parse a number
num :: Parser Expr
num = fmap Num integer

-- Parse a specific string (keyword or symbol)
string :: String -> Parser String
string s = Parser $ \input ->
  if s `isPrefixOf` input
    then [(s, drop (length s) input)]
    else []

-- Parse whitespace
ws :: Parser ()
ws = Parser $ \input -> [((), dropWhile isSpace input)]

-- Parse an identifier (e.g., "main")
identifier :: Parser String
identifier = do
  first <- satisfy isAlpha
  rest <- many (satisfy isAlphaNum)
  ws
  return (first : rest)

-- Parse a specific symbol
symbol :: Char -> Parser Char
symbol c = char c <* ws

-- Parse a return statement
returnStmt :: Parser Stmt
returnStmt = do
  _ <- string "return"
  ws
  expr <- expr
  _ <- symbol ';'
  return (Return expr)

-- Parse a block of statements
block :: Parser Stmt
block = do
  _ <- symbol '{'
  stmts <- many stmt
  _ <- symbol '}'
  return (Block stmts)

-- Parse a function
function :: Parser Function
function = do
  _ <- string "int"
  ws
  name <- identifier
  _ <- symbol '('
  _ <- symbol ')' -- No parameters for now
  ws
  body <- block
  return (Function name [] body)

-- Top-level parser
stmt :: Parser Stmt
stmt = returnStmt <|> block

topLevel :: Parser Function
topLevel = function

test :: IO ()
test = do
  let input = "int main() { return 0; }"
  case runParser topLevel input of
    [(func, "")] -> print func
    _ -> putStrLn "Parsing failed."
