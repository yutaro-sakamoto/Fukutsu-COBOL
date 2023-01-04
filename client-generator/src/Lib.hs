{-# LANGUAGE OverloadedStrings #-}

module Lib
  ( someFunc,
    someFunc2,
  )
where

import Data.Text (Text)
import Types (CobText)

someFunc :: IO ()
someFunc = putStrLn "someFunc"

someFunc2 :: IO CobText
someFunc2 = return "hello"
