use std::collections::{HashMap, HashSet};

use day_24::{parse_input, Operation, INPUT};
use itertools::Itertools;

fn main() {
    println!("Part 2: {}", part_2(INPUT));
}

fn part_2(input: &str) -> String {
    let (_, connections) = parse_input(input);
    let mut connections = connections
        .into_iter()
        .map(|(c, input)| (c.to_string(), input))
        .collect::<Vec<_>>();

    let mut wire_func = WireFunc::new();

    for (c, (op, a, b)) in connections.iter() {
        if a.chars().nth(1).unwrap().is_ascii_digit() && a[1..] == b[1..] {
            let id = a[1..].parse().unwrap();
            match op {
                Operation::And => {
                    wire_func.add(c, Func::AandB(id));
                }
                Operation::Xor => {
                    wire_func.add(c, Func::AxorB(id));
                }
                Operation::Or => panic!(),
            }
        }
    }
    let mut swaps = Vec::new();
    let mut step = 1;
    loop {
        let mut data = get_next(&connections, &wire_func);
        if data.is_empty() {
            break;
        }
        assert_eq!(data.len(), 2);
        let (mut c2, (op2, a2, b2)) = data.pop().unwrap();
        let (mut c1, (op1, a1, b1)) = data.pop().unwrap();
        assert_eq!(op1, Operation::And);
        assert_eq!(op2, Operation::Xor);
        assert_eq!(a1, a2);
        assert_eq!(b1, b2);

        let expected_c = format!("z{step:02}");
        if c2 != expected_c {
            apply_swap(&mut connections, &mut wire_func, &c2, &expected_c);
            swaps.push(c2.clone());
            swaps.push(expected_c.clone());
            if c1 == expected_c {
                c1 = c2;
            }
            c2 = expected_c;
        }

        let expecteds: HashSet<Func> = HashSet::from([
            if step > 1 {
                Func::Carry(step - 1)
            } else {
                Func::AandB(0)
            },
            Func::AxorB(step),
        ]);
        let actuals = HashSet::from([wire_func.get(a1), wire_func.get(b1)]);
        if expecteds != actuals {
            let intersection: Vec<_> = expecteds.intersection(&actuals).collect();
            assert_eq!(intersection.len(), 1);
            for (act, exp) in actuals
                .difference(&expecteds)
                .zip(expecteds.difference(&actuals))
            {
                let real_act = wire_func.get_reverse(*act);
                let real_exp = wire_func.get_reverse(*exp);
                apply_swap(&mut connections, &mut wire_func, &real_act, &real_exp);
                swaps.push(real_act);
                swaps.push(real_exp);
            }
        }

        wire_func.add(&c1, Func::TempCarry(step));
        wire_func.add(&c2, Func::Sum(step));

        let mut data = get_next(&connections, &wire_func);
        assert_eq!(data.len(), 1);
        let (c, (op, a, b)) = data.pop().unwrap();
        assert_eq!(op, Operation::Or);

        let expecteds = HashSet::from([Func::AandB(step), Func::TempCarry(step)]);
        let actuals = HashSet::from([wire_func.get(a), wire_func.get(b)]);
        assert_eq!(expecteds, actuals);

        wire_func.add(&c, Func::Carry(step));

        step += 1;
    }
    assert_eq!(swaps.len(), 8);
    swaps.iter().sorted().join(",")
}

type Entry<'a> = (String, (Operation, &'a str, &'a str));

// See diagram.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Func {
    AandB(i32),
    TempCarry(i32),
    Carry(i32),
    AxorB(i32),
    Sum(i32),
}

struct WireFunc {
    wire_to_func: HashMap<String, Func>,
    func_to_wire: HashMap<Func, String>,
}

impl WireFunc {
    fn new() -> Self {
        Self {
            wire_to_func: HashMap::new(),
            func_to_wire: HashMap::new(),
        }
    }

    fn add(&mut self, wire: &str, func: Func) {
        self.wire_to_func.insert(wire.to_string(), func);
        self.func_to_wire.insert(func, wire.to_string());
    }

    fn swap(&mut self, actual: &str, expected: &str) {
        let old_act = self.wire_to_func.get(actual).cloned();
        let old_exp = self.wire_to_func.get(expected).cloned();
        if let Some(old_act) = old_act {
            self.add(expected, old_act);
        }
        if let Some(old_exp) = old_exp {
            self.add(actual, old_exp);
        }
    }

    fn contains(&self, wire: &str) -> bool {
        self.wire_to_func.contains_key(wire)
    }

    fn get(&self, wire: &str) -> Func {
        self.wire_to_func[wire]
    }

    fn get_reverse(&self, func: Func) -> String {
        self.func_to_wire[&func].clone()
    }
}

fn get_next<'a>(entries: &[Entry<'a>], wire_func: &WireFunc) -> Vec<Entry<'a>> {
    let mut active = Vec::new();
    for (c, (op, a, b)) in entries.iter() {
        if wire_func.contains(c) || !wire_func.contains(a) || !wire_func.contains(b) {
            continue;
        }
        if wire_func.get(a) > wire_func.get(b) {
            active.push((c.clone(), (*op, *b, *a)));
        } else {
            active.push((c.clone(), (*op, *a, *b)));
        }
    }
    active.sort_by_key(|(c, (op, a, b))| (*a, *b, *op, c.clone()));
    active
}

fn apply_swap(entries: &mut [Entry], wire_func: &mut WireFunc, actual: &str, expected: &str) {
    for (target, _) in entries.iter_mut() {
        if target == actual {
            *target = expected.to_string();
        } else if target == expected {
            *target = actual.to_string();
        }
    }
    wire_func.swap(actual, expected);
}
