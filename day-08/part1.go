package main

import (
	"strings"
)

func part1(input string) int {
	parts := strings.Split(input, "\n")
	m := getAntennas(parts)
	lx := len(strings.Trim(parts[0], " \r"))
	ly := len(parts)
	antinodes := make(map[coords]bool)
	for _, crds := range m {
		for i, a := range crds {
			for j, b := range crds {
				if i == j {
					continue
				}
				dx := a.x - b.x
				dy := a.y - b.y
				crd := coords{a.x + dx, a.y + dy}
				if crd.x >= 0 && crd.y >= 0 && crd.x < lx && crd.y < ly {
					antinodes[crd] = true
				}
			}
		}
	}
	return len(antinodes)
}
