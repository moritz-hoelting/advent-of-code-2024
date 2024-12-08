package main

import "testing"

var input string = `............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............`

func TestPart1(t *testing.T) {
	expected := 14
	actual := part1(input)

	if actual != expected {
		t.Errorf("part1 = %d, expected %d", actual, expected)
	}
}

func TestPart2(t *testing.T) {
	expected := 34
	actual := part2(input)

	if actual != expected {
		t.Errorf("part2 = %d, expected %d", actual, expected)
	}
}
