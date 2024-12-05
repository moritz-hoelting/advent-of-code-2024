let read_file filename = 
    let lines = ref [] in
    let chan = open_in filename in
    try
        while true; do
            lines := input_line chan :: !lines
        done; !lines
    with End_of_file ->
        close_in chan;
        List.rev !lines ;;

let () = 
    let input = read_file "input.txt" in
    let res1 = Day_05.part1 input in
    Printf.printf "Part 1: %d\n" res1;
    let res2 = Day_05.part2 input in
    Printf.printf "Part 2: %d\n" res2
