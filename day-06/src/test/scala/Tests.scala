def sampleInput = """....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."""


// For more information on writing tests, see
// https://scalameta.org/munit/docs/getting-started.html
class Tests extends munit.FunSuite {
  test("Part 1") {
    assertEquals(part1(sampleInput), 41)
  }

  test("Part 2") {
    assertEquals(part2(sampleInput), 6)
  }
}
