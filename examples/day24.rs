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

#[derive(Debug)]
struct Signal {
    name: String,
    signal: Option<i8>,
}

#[derive(Debug)]
enum Gate {
    Xor(Signal, Signal, Signal),
    And(Signal, Signal, Signal),
    Or(Signal, Signal, Signal),
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

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let mut signals = HashMap::new();
    data.inputs.into_iter().for_each(|signal| {
        signals.insert(signal.name, signal.signal);
    });
    data.gates.iter().for_each(|gate| {
        let name = match gate {
            Gate::Xor(_, _, n) => n.name.clone(),
            Gate::Or(_, _, n) => n.name.clone(),
            Gate::And(_, _, n) => n.name.clone(),
        };
        signals.insert(name, None);
    });

    let gates = data.gates;

    while signals.values().any(Option::is_none) {
        for gate in gates.iter() {
            let (in1, in2) = match gate {
                Gate::Xor(a, b, _) => (&a.name, &b.name),
                Gate::Or(a, b, _) => (&a.name, &b.name),
                Gate::And(a, b, _) => (&a.name, &b.name),
            };
            match (signals.get(in1).unwrap(), signals.get(in2).unwrap()) {
                (Some(sig1), Some(sig2)) => match gate {
                    Gate::Xor(_, _, out) => {
                        *signals.get_mut(&out.name).unwrap() = Some(*sig1 ^ *sig2)
                    }
                    Gate::Or(_, _, out) => {
                        *signals.get_mut(&out.name).unwrap() = Some(*sig1 | *sig2)
                    }
                    Gate::And(_, _, out) => {
                        *signals.get_mut(&out.name).unwrap() = Some(*sig1 & *sig2)
                    }
                },
                _ => (),
            };
        }
    }

    dp!(signals);

    let outputs = signals
        .into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted_by_key(|(k, _)| Reverse(k.clone()))
        .collect_vec();

    dp!(outputs);

    let binary: String = outputs
        .into_iter()
        .map(|x| x.1.unwrap().to_string().chars().next().unwrap())
        .collect();

    dp!(binary);

    let answer = u64::from_str_radix(&binary, 2).unwrap();

    println!("{}", answer);

    Ok(())
}
