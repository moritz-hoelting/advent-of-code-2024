package main

import (
	"strings"
)

func part2(input string) int {
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
				i := 0
				for a.x+dx*i >= 0 && a.y+dy*i >= 0 && a.x+dx*i < lx && a.y+dy*i < ly {
					crd := coords{a.x + dx*i, a.y + dy*i}
					antinodes[crd] = true
					i++
				}
			}
		}
	}
	return len(antinodes)
}
