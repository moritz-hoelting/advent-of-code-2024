package main

import (
	"strings"
	"unicode"
)

type coords struct {
	x int
	y int
}

func getAntennas(parts []string) map[rune][]coords {
	m := make(map[rune][]coords)
	for y, p := range parts {
		for x, c := range strings.Trim(p, " \r") {
			if unicode.IsDigit(c) || unicode.IsLetter(c) {
				m[c] = append(m[c], coords{x, y})
			}
		}
	}
	return m
}
