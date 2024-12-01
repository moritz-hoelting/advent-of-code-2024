import day_01
import gleeunit
import gleeunit/should

pub fn main() {
  gleeunit.main()
}

const sample = "3   4
4   3
2   5
1   3
3   9
3   3"

// gleeunit test functions end in `_test`
pub fn part_1_test() {
  day_01.part_1(
    sample,
  )
  |> should.equal(11)
}

pub fn part_2_test() {
  day_01.part_2(
    sample,
  )
  |> should.equal(31)
}