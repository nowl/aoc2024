use anyhow::Error;
use aoc2024::{dp, Args};
use bytes::complete::tag;
use character::complete::{alphanumeric1, digit1, multispace0, multispace1};
use clap::Parser;
use debug_print::debug_println;
use itertools::Itertools;
use multi::{many1, separated_list1};
use nom::*;
use sequence::{preceded, separated_pair, terminated, tuple};
use std::{cmp::Reverse, collections::HashMap, fs, path::Path};

const TEST_INPUT: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

#[derive(Debug, Clone)]
struct Signal {
    name: String,
    signal: Option<i8>,
}

#[derive(Debug, Clone)]
enum Gate {
    Xor(Signal, Signal, Signal),
    And(Signal, Signal, Signal),
    Or(Signal, Signal, Signal),
}

impl Gate {
    fn output_name(&self) -> &str {
        match self {
            Gate::Xor(_, _, out) => &out.name,
            Gate::And(_, _, out) => &out.name,
            Gate::Or(_, _, out) => &out.name,
        }
    }
}

#[derive(Debug)]
struct Data {
    inputs: Vec<Signal>,
    gates: Vec<Gate>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_input = separated_pair(alphanumeric1, tag(":"), preceded(multispace0, digit1));
    let parse_gate = separated_pair(
        separated_list1(multispace1, alphanumeric1),
        preceded(multispace0, tag("->")),
        preceded(multispace0, alphanumeric1),
    );
    let mut parse_problem = tuple((
        many1(terminated(parse_input, multispace0)),
        many1(terminated(parse_gate, multispace0)),
    ));
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let inputs = problems
        .0
        .into_iter()
        .map(|(a, b)| Signal {
            name: a.to_string(),
            signal: Some(b.parse().unwrap()),
        })
        .collect();

    let gates = problems
        .1
        .into_iter()
        .map(|(a, b)| {
            let signal_in1 = Signal {
                name: a[0].to_string(),
                signal: None,
            };
            let signal_in2 = Signal {
                name: a[2].to_string(),
                signal: None,
            };
            let signal_out = Signal {
                name: b.to_string(),
                signal: None,
            };
            match a[1] {
                "XOR" => Gate::Xor(signal_in1, signal_in2, signal_out),
                "OR" => Gate::Or(signal_in1, signal_in2, signal_out),
                "AND" => Gate::And(signal_in1, signal_in2, signal_out),
                _ => unreachable!(),
            }
        })
        .collect();

    let data = Data { inputs, gates };
    Ok((i, data))
}

fn read_data() -> Result<Data, Error> {
    let args = Args::parse();

    let contents = args.file.map_or(Ok(TEST_INPUT.to_string()), |input| {
        let file = Path::new(&input);
        fs::read_to_string(file)
    })?;

    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0.is_empty());
    Ok(data.1)
}

fn binary_values(typ: &str, signals: &HashMap<String, Option<i8>>) -> String {
    let outputs = signals
        .iter()
        .filter(|(k, _)| k.starts_with(typ))
        .sorted_by_key(|(k, _)| Reverse(k.to_string()))
        .collect_vec();

    outputs
        .into_iter()
        .map(|x| x.1.unwrap().to_string().chars().next().unwrap())
        .collect()
}

fn run_simulation(signals: &mut HashMap<String, Option<i8>>, gates: &Vec<Gate>) -> bool {
    loop {
        let mut changed = false;
        for gate in gates.iter() {
            let (in1, in2) = match gate {
                Gate::Xor(a, b, _) => (&a.name, &b.name),
                Gate::Or(a, b, _) => (&a.name, &b.name),
                Gate::And(a, b, _) => (&a.name, &b.name),
            };
            match (signals.get(in1).unwrap(), signals.get(in2).unwrap()) {
                (Some(sig1), Some(sig2)) => match gate {
                    Gate::Xor(_, _, out) => {
                        if signals.get(&out.name).unwrap().is_none() {
                            changed = true;
                        }
                        *signals.get_mut(&out.name).unwrap() = Some(*sig1 ^ *sig2);
                    }
                    Gate::Or(_, _, out) => {
                        if signals.get(&out.name).unwrap().is_none() {
                            changed = true;
                        }
                        *signals.get_mut(&out.name).unwrap() = Some(*sig1 | *sig2);
                    }
                    Gate::And(_, _, out) => {
                        if signals.get(&out.name).unwrap().is_none() {
                            changed = true;
                        }
                        *signals.get_mut(&out.name).unwrap() = Some(*sig1 & *sig2);
                    }
                },
                _ => (),
            };
        }

        if !changed {
            if signals.values().any(Option::is_none) {
                return false;
            } else {
                return true;
            }
        }
    }
}

fn swap_outputs(a: usize, b: usize, gates: &mut Vec<Gate>) -> (String, String) {
    let astr = match &gates[a] {
        Gate::Xor(_, _, out) => out.name.clone(),
        Gate::Or(_, _, out) => out.name.clone(),
        Gate::And(_, _, out) => out.name.clone(),
    };
    let bstr = match &gates[b] {
        Gate::Xor(_, _, out) => out.name.clone(),
        Gate::Or(_, _, out) => out.name.clone(),
        Gate::And(_, _, out) => out.name.clone(),
    };

    match gates.get_mut(a).unwrap() {
        Gate::Xor(_, _, sig) => sig.name = bstr.clone(),
        Gate::Or(_, _, sig) => sig.name = bstr.clone(),
        Gate::And(_, _, sig) => sig.name = bstr.clone(),
    }

    match gates.get_mut(b).unwrap() {
        Gate::Xor(_, _, sig) => sig.name = astr.clone(),
        Gate::Or(_, _, sig) => sig.name = astr.clone(),
        Gate::And(_, _, sig) => sig.name = astr.clone(),
    }

    (astr, bstr)
}

fn swap_outputs_by_name(a: &str, b: &str, gates: &mut Vec<Gate>) {
    let apos = gates
        .iter()
        .position(|v| {
            let out = match v {
                Gate::Xor(_, _, out) => &out.name,
                Gate::And(_, _, out) => &out.name,
                Gate::Or(_, _, out) => &out.name,
            };
            a == out
        })
        .unwrap();

    let bpos = gates
        .iter()
        .position(|v| {
            let out = match v {
                Gate::Xor(_, _, out) => &out.name,
                Gate::And(_, _, out) => &out.name,
                Gate::Or(_, _, out) => &out.name,
            };
            b == out
        })
        .unwrap();

    let (aname, bname) = swap_outputs(apos, bpos, gates);
    debug_assert_eq!(aname, a);
    debug_assert_eq!(bname, b);
}

fn count_bits(n: u64) -> i32 {
    let mut count = 0;
    for i in 0..64 {
        if (n >> i) & 0x1 == 1 {
            count += 1;
        }
    }
    count
}

fn swap_and_run(
    swaps: &Vec<(usize, usize)>,
    orig_signals: &HashMap<String, Option<i8>>,
    orig_gates: &Vec<Gate>,
    orig_answer: u64,
    bits_to_flip: u64,
    desired_bit_flips: i32,
) -> Option<(bool, bool)> {
    let mut signals = orig_signals.clone();
    let mut gates = orig_gates.clone();
    for &(a, b) in swaps {
        swap_outputs(a, b, &mut gates);
    }
    let valid_sim = run_simulation(&mut signals, &gates);
    if !valid_sim {
        return None;
    }
    //debug_println!("swapped {} (idx {}) and {} (idx {})", swap1, a, swap2, b);
    let binary = binary_values("z", &signals);
    let answer = u64::from_str_radix(&binary, 2).unwrap();
    debug_println!("actual z value:{:>50}, {:?}", binary, answer);
    debug_println!("bit diffs     :{:>50b}", orig_answer ^ answer);
    let undesired_bits = (!bits_to_flip) & (answer ^ orig_answer);
    let desired_bits = bits_to_flip & (answer ^ orig_answer);
    Some((
        undesired_bits == 0 && count_bits(desired_bits) >= desired_bit_flips,
        answer == orig_answer,
    ))
}

fn search_gates(output: &str, gates: &Vec<Gate>) -> Vec<String> {
    let mut inputs = vec![];
    for gate in gates.iter() {
        let (in1, in2, out) = match &gate {
            Gate::Xor(in1, in2, out) => (&in1.name, &in2.name, &out.name),
            Gate::And(in1, in2, out) => (&in1.name, &in2.name, &out.name),
            Gate::Or(in1, in2, out) => (&in1.name, &in2.name, &out.name),
        };

        if out == output {
            if !in1.starts_with("x") && !in1.starts_with("y") {
                inputs.push(in1.clone());
                inputs.extend(search_gates(in1, gates));
            }
            if !in2.starts_with("x") && !in2.starts_with("y") {
                inputs.push(in2.clone());
                inputs.extend(search_gates(in2, gates));
            }
        }
    }
    inputs
}

fn find_wrong_outputs(
    gates: &Vec<Gate>,
    signals: &HashMap<String, Option<i8>>,
) -> Option<Vec<String>> {
    let mut wrong_outputs = vec![];

    // identify wrong outputs
    for output_num in 0..50 {
        let mut signals = signals.clone();

        // set signal inputs
        signals.iter_mut().for_each(|(k, v)| {
            if k.starts_with("x") {
                *v = Some(0);
            }
            if k.starts_with("y") {
                *v = Some(0);
            }
            if k.starts_with(&format!("y{:2}", output_num)) {
                *v = Some(1);
            }
        });

        let x_binary = binary_values("x", &signals);
        let x_value = u64::from_str_radix(&x_binary, 2).unwrap();
        debug_println!("x value:       {:>50}, {:?}", x_binary, x_value);

        let y_binary = binary_values("y", &signals);
        let y_value = u64::from_str_radix(&y_binary, 2).unwrap();
        debug_println!("y value:       {:>50}, {:?}", y_binary, y_value);

        let z_value = x_value + y_value;
        debug_println!("target z value:{:>50b}, {:?}", z_value, z_value);

        let valid_sim = run_simulation(&mut signals, &gates);
        if !valid_sim {
            return None;
        }
        let binary = binary_values("z", &signals);
        let answer = u64::from_str_radix(&binary, 2).unwrap();
        debug_println!("actual z value:{:>50}, {:?}", binary, answer);

        let bits_to_flip = answer ^ z_value;
        debug_println!("bits to flip:  {:>50b}", bits_to_flip);

        if z_value != answer {
            wrong_outputs.push(format!("z{:2}", output_num));
        }
    }

    dp!(wrong_outputs);

    Some(wrong_outputs)
}

fn run_swap_sim(
    gates: &Vec<Gate>,
    signals: &HashMap<String, Option<i8>>,
) -> Option<(String, String)> {
    let wrong_outputs = find_wrong_outputs(gates, signals);
    if wrong_outputs.is_none() {
        return None;
    }

    let wrong_outputs = wrong_outputs.unwrap();

    let mut swap1 = None;
    for gate in gates.iter().filter(|&v| {
        let oname = v.output_name();
        !oname.starts_with("x") && !oname.starts_with("y")
    }) {
        let mut gates = gates.clone();
        swap_outputs_by_name(gate.output_name(), &wrong_outputs[0], &mut gates);
        let new_wrong_outputs = find_wrong_outputs(&gates, signals);
        if let Some(new_wrong_outputs) = new_wrong_outputs {
            if new_wrong_outputs.len() < wrong_outputs.len() {
                swap1 = Some((gate.output_name().to_string(), wrong_outputs[0].clone()));
                break;
            }
        }
    }

    swap1
}

fn orig_test(gates: &Vec<Gate>, orig_signals: &HashMap<String, Option<i8>>) {
    let x_binary = binary_values("x", &orig_signals);
    let x_value = u64::from_str_radix(&x_binary, 2).unwrap();
    println!("x value:       {:>50}, {:?}", x_binary, x_value);

    let y_binary = binary_values("y", &orig_signals);
    let y_value = u64::from_str_radix(&y_binary, 2).unwrap();
    println!("y value:       {:>50}, {:?}", y_binary, y_value);

    let z_value = x_value + y_value;
    println!("target z value:{:>50b}, {:?}", z_value, z_value);

    let mut signals = orig_signals.clone();
    run_simulation(&mut signals, &gates);

    let binary = binary_values("z", &signals);
    let answer = u64::from_str_radix(&binary, 2).unwrap();
    println!("actual z value:{:>50}, {:?}", binary, answer);

    let bits_to_flip = answer ^ z_value;
    println!("bits to flip:  {:>50b}", bits_to_flip);
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let orig_signals = {
        let mut signals = HashMap::new();
        data.inputs.into_iter().for_each(|signal| {
            signals.insert(signal.name, signal.signal);
        });
        data.gates.iter().for_each(|gate| {
            let name = gate.output_name().to_string();
            signals.insert(name, None);
        });
        signals
    };

    let mut all_swaps = vec![];

    // swap 1
    let swap = run_swap_sim(&data.gates, &orig_signals);
    debug_println!("{:?}", swap);

    // swap 2
    let mut gates = data.gates.clone();
    let Some((a, b)) = swap else {
        unreachable!();
    };
    all_swaps.push(a.clone());
    all_swaps.push(b.clone());
    swap_outputs_by_name(&a, &b, &mut gates);
    let swap = run_swap_sim(&gates, &orig_signals);

    debug_println!("{:?}", swap);

    // swap 3
    let Some((a, b)) = swap else {
        unreachable!();
    };
    all_swaps.push(a.clone());
    all_swaps.push(b.clone());
    swap_outputs_by_name(&a, &b, &mut gates);
    let swap = run_swap_sim(&gates, &orig_signals);

    debug_println!("{:?}", swap);

    // swap 4
    let Some((a, b)) = swap else {
        unreachable!();
    };
    all_swaps.push(a.clone());
    all_swaps.push(b.clone());
    swap_outputs_by_name(&a, &b, &mut gates);
    let swap = run_swap_sim(&gates, &orig_signals);

    debug_println!("{:?}", swap);

    let Some((a, b)) = swap else {
        unreachable!();
    };
    all_swaps.push(a.clone());
    all_swaps.push(b.clone());

    let answer = all_swaps.iter().sorted().join(",");
    println!("{}", answer);

    swap_outputs_by_name(&a, &b, &mut gates);
    orig_test(&gates, &orig_signals);

    Ok(())
}
