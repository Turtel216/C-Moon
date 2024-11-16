-- Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
-- Use of this source code is governed by a MIT
-- license that can be found in the LICENSE file.

module Main where

import qualified Parser (test)

main :: IO ()
main = do
  Parser.test
