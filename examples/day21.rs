use anyhow::Error;
use aoc2024::{dp, Args};
use character::complete::{alphanumeric1, multispace0};
use clap::Parser;
use debug_print::debug_println;
use itertools::Itertools;
use multi::many1;
use nom::*;
use sequence::terminated;
use std::{collections::HashMap, fs, path::Path};

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
    SU,
    SD,
    SL,
    SR,
    SA,
}

// prefer L, D, U, R
fn moves_keypad(s: NumericKeypadState, d: NumericKeypadState) -> Vec<RobotKeypadState> {
    match s {
        NumS0 => match d {
            NumS0 => vec![],
            NumS1 => vec![SL, SU],
            NumS2 => vec![SU],
            NumS3 => vec![SR, SU],
            NumS4 => vec![SL, SU, SU],
            NumS5 => vec![SU, SU],
            NumS6 => vec![SU, SU, SR],
            NumS7 => vec![SL, SU, SU, SU],
            NumS8 => vec![SU, SU, SU],
            NumS9 => vec![SU, SU, SU, SR],
            NumSA => vec![SR],
        },
        NumS1 => match d {
            NumS0 => vec![SD, SR],
            NumS1 => vec![],
            NumS2 => vec![SR],
            NumS3 => vec![SR, SR],
            NumS4 => vec![SU],
            NumS5 => vec![SU, SR],
            NumS6 => vec![SU, SR, SR],
            NumS7 => vec![SU, SU],
            NumS8 => vec![SU, SU, SR],
            NumS9 => vec![SU, SU, SR, SR],
            NumSA => vec![SD, SR, SR],
        },
        NumS2 => match d {
            NumS0 => vec![SD],
            NumS1 => vec![SL],
            NumS2 => vec![],
            NumS3 => vec![SR],
            NumS4 => vec![SL, SU],
            NumS5 => vec![SU],
            NumS6 => vec![SR, SU],
            NumS7 => vec![SL, SU, SU],
            NumS8 => vec![SU, SU],
            NumS9 => vec![SU, SU, SR],
            NumSA => vec![SD, SR],
        },
        NumS3 => match d {
            NumS0 => vec![SL, SD],
            NumS1 => vec![SL, SL],
            NumS2 => vec![SL],
            NumS3 => vec![],
            NumS4 => vec![SL, SL, SU],
            NumS5 => vec![SL, SU],
            NumS6 => vec![SU],
            NumS7 => vec![SL, SL, SU, SU],
            NumS8 => vec![SU, SU, SL],
            NumS9 => vec![SU, SU],
            NumSA => vec![SD],
        },
        NumS4 => match d {
            NumS0 => vec![SD, SD, SR],
            NumS1 => vec![SD],
            NumS2 => vec![SL, SD],
            NumS3 => vec![SD, SR, SR],
            NumS4 => vec![],
            NumS5 => vec![SR],
            NumS6 => vec![SR, SR],
            NumS7 => vec![SU],
            NumS8 => vec![SU, SR],
            NumS9 => vec![SU, SR, SR],
            NumSA => vec![SD, SD, SR, SR],
        },
        NumS5 => match d {
            NumS0 => vec![SD, SD],
            NumS1 => vec![SD, SL],
            NumS2 => vec![SD],
            NumS3 => vec![SD, SR],
            NumS4 => vec![SL],
            NumS5 => vec![],
            NumS6 => vec![SR],
            NumS7 => vec![SU, SL],
            NumS8 => vec![SU],
            NumS9 => vec![SU, SR],
            NumSA => vec![SD, SD, SR],
        },
        NumS6 => match d {
            NumS0 => vec![SD, SD, SL],
            NumS1 => vec![SD, SL, SL],
            NumS2 => vec![SD, SL],
            NumS3 => vec![SD],
            NumS4 => vec![SL, SL],
            NumS5 => vec![SL],
            NumS6 => vec![],
            NumS7 => vec![SU, SL, SL],
            NumS8 => vec![SU, SL],
            NumS9 => vec![SU],
            NumSA => vec![SD, SD],
        },
        NumS7 => match d {
            NumS0 => vec![SD, SD, SD, SR],
            NumS1 => vec![SD, SD],
            NumS2 => vec![SD, SD, SR],
            NumS3 => vec![SD, SD, SR, SR],
            NumS4 => vec![SD],
            NumS5 => vec![SD, SR],
            NumS6 => vec![SD, SR, SR],
            NumS7 => vec![],
            NumS8 => vec![SR],
            NumS9 => vec![SR, SR],
            NumSA => vec![SD, SD, SD, SR, SR],
        },
        NumS8 => match d {
            NumS0 => vec![SD, SD, SD],
            NumS1 => vec![SL, SD, SD],
            NumS2 => vec![SD, SD],
            NumS3 => vec![SD, SD, SR],
            NumS4 => vec![SL, SD],
            NumS5 => vec![SD],
            NumS6 => vec![SD, SR],
            NumS7 => vec![SL],
            NumS8 => vec![],
            NumS9 => vec![SR],
            NumSA => vec![SD, SD, SD, SR],
        },
        NumS9 => match d {
            NumS0 => vec![SD, SD, SD, SL],
            NumS1 => vec![SD, SD, SL, SL],
            NumS2 => vec![SD, SD, SL],
            NumS3 => vec![SD, SD],
            NumS4 => vec![SD, SL, SL],
            NumS5 => vec![SD, SL],
            NumS6 => vec![SD],
            NumS7 => vec![SL, SL],
            NumS8 => vec![SL],
            NumS9 => vec![],
            NumSA => vec![SD, SD, SD],
        },
        NumSA => match d {
            NumS0 => vec![SL],
            NumS1 => vec![SU, SL, SL],
            NumS2 => vec![SU, SL],
            NumS3 => vec![SU],
            NumS4 => vec![SU, SU, SL, SL],
            NumS5 => vec![SU, SU, SL],
            NumS6 => vec![SU, SU],
            NumS7 => vec![SU, SU, SU, SL, SL],
            NumS8 => vec![SU, SU, SU, SL],
            NumS9 => vec![SU, SU, SU],
            NumSA => vec![],
        },
    }
}

// prefer U,D,L,R
fn moves_robot(s: RobotKeypadState, d: RobotKeypadState) -> Vec<Vec<RobotKeypadState>> {
    match s {
        SU => match d {
            SU => vec![vec![]],
            SD => vec![vec![SD]],
            SL => vec![vec![SD, SL]],
            SR => vec![vec![SD, SR], vec![SR, SD]],
            SA => vec![vec![SR]],
        },
        SD => match d {
            SU => vec![vec![SU]],
            SD => vec![vec![]],
            SL => vec![vec![SL]],
            SR => vec![vec![SR]],
            SA => vec![vec![SR, SU], vec![SU, SR]],
        },
        SL => match d {
            SU => vec![vec![SR, SU]],
            SD => vec![vec![SR]],
            SL => vec![vec![]],
            SR => vec![vec![SR, SR]],
            SA => vec![vec![SR, SR, SU], vec![SR, SU, SR]],
        },
        SR => match d {
            SU => vec![vec![SL, SU], vec![SU, SL]],
            SD => vec![vec![SL]],
            SL => vec![vec![SL, SL]],
            SR => vec![vec![]],
            SA => vec![vec![SU]],
        },
        SA => match d {
            SU => vec![vec![SL]],
            SD => vec![vec![SL, SD], vec![SD, SL]],
            SL => vec![vec![SL, SD, SL], vec![SD, SL, SL]],
            SR => vec![vec![SD]],
            SA => vec![vec![]],
        },
    }
}

fn count_moves(
    cur_level: i32,
    max_level: i32,
    path: Vec<RobotKeypadState>,
    cache: &mut HashMap<(i32, Vec<RobotKeypadState>), u64>,
) -> u64 {
    if let Some(count) = cache.get(&(cur_level, path.clone())) {
        return *count;
    }

    let all_moves = moves_robot(path[0].clone(), path[path.len() - 1].clone());

    let count = if cur_level == max_level {
        all_moves[0].len() as u64 + 1
    } else {
        let mut best_count = None;
        for mut moves in all_moves {
            moves.push(SA);
            moves.insert(0, SA);
            let current_count = moves.windows(2).into_iter().fold(0, |acc, vals| {
                let s = vals[0];
                let d = vals[1];
                acc + count_moves(cur_level + 1, max_level, vec![s, d], cache)
            });
            if best_count.is_none() || best_count.unwrap() > current_count {
                best_count = Some(current_count);
            }
        }
        best_count.unwrap()
    };

    cache.insert((cur_level, path), count);
    count
}

fn is_illegal_keypad_move(start: NumericKeypadState, moves: &Vec<RobotKeypadState>) -> bool {
    match start {
        NumS0 => moves.starts_with(&[SL]),
        NumS1 => moves.starts_with(&[SD]),
        NumS4 => moves.starts_with(&[SD, SD]),
        NumS7 => moves.starts_with(&[SD, SD, SD]),
        NumSA => moves.starts_with(&[SL, SL]),
        _ => false,
    }
}

use NumericKeypadState::*;
use RobotKeypadState::*;

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let robot_count = 25;

    let mut cache = HashMap::new();
    let mut total_count = 0;
    for combo in data.codes {
        let mut prev = NumSA;
        let factor = combo[0..3].parse::<u64>().unwrap();
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

            debug_println!("moving from {:?} to {:?}", prev, this_num);
            let initial_robot_moves_orig = moves_keypad(prev, this_num);
            let irmo_len = initial_robot_moves_orig.len();
            let mut best_move_count = None;
            for mut initial_robot_moves in
                initial_robot_moves_orig.into_iter().permutations(irmo_len)
            {
                if is_illegal_keypad_move(prev, &initial_robot_moves) {
                    continue;
                }
                initial_robot_moves.insert(0, SA);
                initial_robot_moves.push(SA);

                dp!(initial_robot_moves);

                let move_count = initial_robot_moves
                    .windows(2)
                    .into_iter()
                    .fold(0, |acc, vals| {
                        let s = vals[0];
                        let d = vals[1];
                        dp!(s);
                        dp!(d);

                        let move_count = count_moves(1, robot_count, vec![s, d], &mut cache);
                        dp!(move_count);
                        acc + move_count
                    });

                dp!(move_count);

                if best_move_count.is_none_or(|x| x > move_count) {
                    best_move_count = Some(move_count);
                }
            }

            prev = this_num;

            count += best_move_count.unwrap();
        }
        debug_println!("{count}");
        let value = factor * count;
        debug_println!("{value}");
        total_count += value;
    }

    println!("{total_count}");

    Ok(())
}
