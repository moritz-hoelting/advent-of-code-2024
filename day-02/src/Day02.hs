module Day02 (part1, part2) where

part1 :: String -> Int
part1 input = do
    let l = map (\line -> map read (words line)) (lines input)
    countValid $ l

countValid :: [[Int]] -> Int
countValid [] = 0
countValid (x:xs) = if validIncreasing x || validDecreasing x then 1 + countValid xs else countValid xs

validIncreasing :: [Int] -> Bool
validIncreasing [] = False
validIncreasing [_] = True
validIncreasing [a,b] = a < b && b - a <= 3
validIncreasing (a:b:t) = a < b && b - a <= 3 && validIncreasing (b:t)

validDecreasing :: [Int] -> Bool
validDecreasing [] = False
validDecreasing [_] = True
validDecreasing [a,b] = a > b && a - b <= 3
validDecreasing (a:b:t) = a > b && a - b <= 3 && validDecreasing (b:t)


part2 :: String -> Int
part2 input = do
    let l = map (\line -> map read (words line)) (lines input)
    countValidOrAlmostValid $ l

countValidOrAlmostValid :: [[Int]] -> Int
countValidOrAlmostValid [] = 0
countValidOrAlmostValid (x:xs) =
    if isValid x || isAlmostValid x
        then 1 + countValidOrAlmostValid xs
        else countValidOrAlmostValid xs

isValid :: [Int] -> Bool
isValid levels = validIncreasing levels || validDecreasing levels

isAlmostValid :: [Int] -> Bool
isAlmostValid levels = any isValid (removeOne levels)

removeOne :: [a] -> [[a]]
removeOne [] = []
removeOne (x:xs) = xs : map (x :) (removeOne xs)
