use anyhow::Error;
use aoc2024::{
    dijkstra::{DijkstraConfig, DijkstraInput, DijkstraMap},
    dp, Args,
};
use character::complete::{alphanumeric1, multispace0};
use clap::Parser;
use debug_print::debug_println;
use multi::many1;
use nom::*;
use sequence::terminated;
use std::{fs, path::Path};

const TEST_INPUT: &str = "029A
980A
179A
456A
379A";

#[derive(Debug)]
struct Data {
    codes: Vec<String>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let mut parse_problem = many1(terminated(alphanumeric1, multispace0));
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let data = Data {
        codes: problems.into_iter().map(str::to_string).collect(),
    };
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NumericKeypadState {
    NumS0,
    NumS1,
    NumS2,
    NumS3,
    NumS4,
    NumS5,
    NumS6,
    NumS7,
    NumS8,
    NumS9,
    NumSA,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum RobotKeypadState {
    RobotSUp,
    RobotSDown,
    RobotSLeft,
    RobotSRight,
    RobotSA,
}

struct Machine {}

use NumericKeypadState::*;
use RobotKeypadState::*;

impl DijkstraInput for Machine {
    type Cost = i32;

    type Index = (RobotKeypadState, RobotKeypadState, NumericKeypadState);

    fn get_adjacent(&self, &(r1, r2, r3): &Self::Index) -> Vec<(Self::Cost, Self::Index)> {
        let mut v = vec![];

        if let Some((m, _)) = apply_move_robot_robot(RobotSUp, r1) {
            v.push((1, (m, r2, r3)));
        }
        if let Some((m, _)) = apply_move_robot_robot(RobotSDown, r1) {
            v.push((1, (m, r2, r3)));
        }
        if let Some((m, _)) = apply_move_robot_robot(RobotSLeft, r1) {
            v.push((1, (m, r2, r3)));
        }
        if let Some((m, _)) = apply_move_robot_robot(RobotSRight, r1) {
            v.push((1, (m, r2, r3)));
        }
        if let Some((_, true)) = apply_move_robot_robot(RobotSA, r1) {
            if let Some((m, val)) = apply_move_robot_robot(r1, r2) {
                if val {
                    if let Some((m, val)) = apply_move_robot_keypad(r2, r3) {
                        if val {
                            // pass
                        } else {
                            v.push((1, (r1, r2, m)));
                        }
                    }
                } else {
                    v.push((1, (r1, m, r3)));
                }
            }
        }

        v
    }
}

fn apply_move_robot_robot(
    m: RobotKeypadState,
    s: RobotKeypadState,
) -> Option<(RobotKeypadState, bool)> {
    macro_rules! ret {
        ($ret:expr) => {
            Some(($ret, false))
        };
    }

    match m {
        RobotSUp => match s {
            RobotSDown => ret!(RobotSUp),
            RobotSRight => ret!(RobotSA),
            _ => None,
        },
        RobotSDown => match s {
            RobotSUp => ret!(RobotSDown),
            RobotSA => ret!(RobotSRight),
            _ => None,
        },
        RobotSLeft => match s {
            RobotSA => ret!(RobotSUp),
            RobotSRight => ret!(RobotSDown),
            RobotSDown => ret!(RobotSLeft),
            _ => None,
        },
        RobotSRight => match s {
            RobotSDown => ret!(RobotSRight),
            RobotSUp => ret!(RobotSA),
            RobotSLeft => ret!(RobotSDown),
            _ => None,
        },
        RobotSA => Some((s, true)),
    }
}

fn apply_move_robot_keypad(
    m: RobotKeypadState,
    s: NumericKeypadState,
) -> Option<(NumericKeypadState, bool)> {
    macro_rules! ret {
        ($ret:expr) => {
            Some(($ret, false))
        };
    }

    match m {
        RobotSUp => match s {
            NumS0 => ret!(NumS2),
            NumSA => ret!(NumS3),
            NumS1 => ret!(NumS4),
            NumS2 => ret!(NumS5),
            NumS3 => ret!(NumS6),
            NumS4 => ret!(NumS7),
            NumS5 => ret!(NumS8),
            NumS6 => ret!(NumS9),
            _ => None,
        },
        RobotSDown => match s {
            NumS7 => ret!(NumS4),
            NumS8 => ret!(NumS5),
            NumS9 => ret!(NumS6),
            NumS4 => ret!(NumS1),
            NumS5 => ret!(NumS2),
            NumS6 => ret!(NumS3),
            NumS2 => ret!(NumS0),
            NumS3 => ret!(NumSA),
            _ => None,
        },
        RobotSLeft => match s {
            NumS8 => ret!(NumS7),
            NumS5 => ret!(NumS4),
            NumS2 => ret!(NumS1),
            NumS9 => ret!(NumS8),
            NumS6 => ret!(NumS5),
            NumS3 => ret!(NumS2),
            NumSA => ret!(NumS0),
            _ => None,
        },
        RobotSRight => match s {
            NumS7 => ret!(NumS8),
            NumS4 => ret!(NumS5),
            NumS1 => ret!(NumS2),
            NumS8 => ret!(NumS9),
            NumS5 => ret!(NumS6),
            NumS2 => ret!(NumS3),
            NumS0 => ret!(NumSA),
            _ => None,
        },
        RobotSA => Some((s, true)),
    }
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let mut prev = None;

    let mut total_count = 0;
    for combo in data.codes {
        let factor = combo[0..3].parse::<i32>().unwrap();
        let mut count = 0;
        for num in combo.chars() {
            let this_num = match num {
                '0' => NumS0,
                '1' => NumS1,
                '2' => NumS2,
                '3' => NumS3,
                '4' => NumS4,
                '5' => NumS5,
                '6' => NumS6,
                '7' => NumS7,
                '8' => NumS8,
                '9' => NumS9,
                'A' => NumSA,
                _ => unreachable!(),
            };

            let start = if let Some(val) = prev { val } else { NumSA };

            prev = Some(this_num);

            let initial_state = (RobotSA, RobotSA, start);

            let machine = Machine {};
            let mut dmap =
                DijkstraMap::<_, (RobotKeypadState, RobotKeypadState, NumericKeypadState)>::new(
                    &machine,
                    DijkstraConfig { print_1000: false },
                );
            let costs = dmap.run((0, initial_state));
            let cost = costs[&(RobotSA, RobotSA, this_num)];
            count += cost.0 + 1;
            debug_println!("{:?}", count);
        }
        debug_println!("{count}");
        let value = factor * count;
        debug_println!("{value}");
        total_count += value;
    }

    println!("{total_count}");

    Ok(())
}
