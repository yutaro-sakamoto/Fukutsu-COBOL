module Main (main) where

import Lib

main :: IO ()
main = someFunc2 >>= print
