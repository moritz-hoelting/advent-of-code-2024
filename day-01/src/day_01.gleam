import gleam/dict
import gleam/int
import gleam/io
import gleam/list
import gleam/option
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  let input =
    simplifile.read("input.txt")
    |> result.unwrap("")
  let part_1_result = part_1(input)
  io.debug(part_1_result)
  let part_2_result = part_2(input)
  io.debug(part_2_result)
}

fn parse_lists(input: String) -> List(List(Int)) {
  string.split(input, "\n")
    |> list.map(fn(e) {
      string.split(e, " ")
      |> list.filter(fn(e) { e != "" })
      |> list.map(fn(e) {
        string.trim(e)
        |> int.base_parse(10)
        |> result.unwrap(-1)
      })
    })
    |> list.transpose()
}

pub fn part_1(input: String) -> Int {
  let assert [a, b] =
    parse_lists(input)
    |> list.map(fn(e) { list.sort(e, by: int.compare) })

  list.zip(a, b)
  |> list.map(fn(p) { int.absolute_value(p.0 - p.1) })
  |> list.fold(0, fn(acc, cur) { acc + cur })
}


pub fn part_2(input: String) -> Int {
  let assert [a, b] = parse_lists(input)

  let counts = build_dict(dict.new(), b)

  list.fold(a, 0, fn(acc, cur) {
    let count = dict.get(counts, cur)
    case count {
      Error(_) -> acc
      Ok(c) -> acc + {cur * c}
    }
  })
}

fn build_dict(counts: dict.Dict(Int, Int), elems: List(Int)) -> dict.Dict(Int, Int) {
  case elems {
    [] -> counts
    [h, ..t] -> build_dict(
      dict.upsert(counts, h, fn(e) { option.unwrap(e, 0) + 1 }),
      t
    )
  }
}