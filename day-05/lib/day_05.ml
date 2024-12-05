let list_to_tuple l =
  match l with
  | [a; b] -> (a, b)
  | _ -> failwith "Invalid list"

let split_input input =
  let (specs, rest) = List.partition (fun x -> String.contains x '|') input in
  let updates = List.filter (fun x -> (String.trim x) <> "") rest in
  (specs, updates)

let parse_input input =
  let (a, b) = split_input input in
  let a = List.map (fun x -> String.split_on_char '|' x |> List.map (String.trim) |> list_to_tuple) a in
  let b = List.rev_map (fun x -> String.split_on_char ',' x |> List.map (String.trim)) b in
  (a, b)

let rec invalid rules before remaining =
  match remaining with
  | [] -> false
  | h :: t -> List.exists (fun bef -> List.exists (fun (a, b) -> (h = a) && (bef = b)) rules) before || (invalid rules (h :: before) t)

let get_middle_int line = 
  let length = List.length line in
  let middle = (length - 1) / 2 in
  int_of_string (List.nth line middle)

let part1 input = 
  let (specification, updates) = parse_input input in
  let filtered = List.filter (fun line -> not (invalid specification [] line)) updates in
  List.fold_left (fun acc line -> acc + (get_middle_int line)) 0 filtered

let rec build_dependencies deps rules =
  match rules with
  | [] -> deps
  | (a, b) :: t -> 
    let deps = 
      match List.assoc_opt b deps with
      | None -> (b, [a]) :: deps
      | Some l -> (b, a :: l) :: (List.remove_assoc b deps)
    in
    build_dependencies deps t

let filter_dependencies deps elems =
  List.filter_map (fun (depends_on, dependants) ->
    if List.mem depends_on elems then
      Some (depends_on, List.filter (fun x -> List.mem x elems) dependants)
    else
      None
  ) deps

exception CycleDetected

let topological_sort elements dependencies =
  let dependency_map = List.fold_left (fun acc (elem, deps) ->
    List.assoc_opt elem acc
    |> (function
        | None -> (elem, deps) :: acc
        | Some _ -> acc)
  ) [] dependencies in

  let get_dependencies elem =
    match List.assoc_opt elem dependency_map with
    | Some deps -> deps
    | None -> []
  in

  let visited = ref [] in
  let processing = ref [] in

  let rec dfs result elem =
    if List.mem elem !processing then
      raise CycleDetected
    else if List.mem elem !visited then
      result
    else begin
      processing := elem :: !processing;
      let deps = get_dependencies elem in
      let result = List.fold_left dfs result deps in
      processing := List.filter ((<>) elem) !processing;
      visited := elem :: !visited;
      elem :: result
    end
  in

  List.fold_left (fun result elem ->
    try
      dfs result elem
    with CycleDetected -> failwith "Cycle detected in dependencies"
  ) [] elements

let part2 input =
  let (specification, updates) = parse_input input in
  let dependencies = build_dependencies [] specification in
  let corrected = List.filter_map (fun line -> 
    if invalid specification [] line then
      let filtered_deps = filter_dependencies dependencies line in
      let sorted = topological_sort line filtered_deps in
      Some sorted
    else
      None
    ) updates in
  List.fold_left (fun acc line -> acc + (get_middle_int line)) 0 corrected

let sample = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"

let%test _ = part1 (String.split_on_char '\n' sample) = 143
let%test _ = part2 (String.split_on_char '\n' sample) = 123
