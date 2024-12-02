module Main (main) where

import System.Exit (exitFailure)
import Day02 (part1, part2)

sample :: String
sample = "7 6 4 2 1\n\
\1 2 7 8 9\n\
\9 7 6 2 1\n\
\1 3 2 4 5\n\
\8 6 4 4 1\n\
\1 3 6 7 9"

main :: IO ()
main = do
    test1
    test2

test1 :: IO ()
test1 = do
    let res = part1 sample
    if res == 2
        then putStrLn "Part1 passed"
        else do
            putStrLn $ "Part1 failed with " ++ show res
            exitFailure

test2 :: IO ()
test2 = do
    let res = part2 sample
    if res == 4
        then putStrLn "Part2 passed"
        else do
            putStrLn $ "Part2 failed with " ++ show res
            exitFailure
