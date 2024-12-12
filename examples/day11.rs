use std::{
    collections::{HashMap, VecDeque},
    fs,
    path::Path,
};

use anyhow::Error;
use aoc2024::{dp, Args};
use clap::Parser;
use debug_print::debug_println;
use nom::{branch::*, character::complete::*, combinator::*, multi::*, sequence::*, *};

const TEST_INPUT: &str = "125 17";

#[derive(Debug)]
struct Data {
    data: Vec<u64>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_num = map_res(digit1, str::parse);
    let mut parse_problem = many1(terminated(parse_num, alt((eof, multispace1))));
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let data = Data { data: problems };
    Ok((i, data))
}

fn even_digits(n: u64) -> bool {
    n.to_string().len() % 2 == 0
}

fn split_even(n: u64) -> Vec<u64> {
    let s = n.to_string();
    let split = s.len() / 2;
    vec![
        s[0..split].parse().unwrap(),
        s[split..s.len()].parse().unwrap(),
    ]
}

fn run_rules(n: u64) -> Vec<u64> {
    if n == 0 {
        vec![1]
    } else if even_digits(n) {
        split_even(n)
    } else {
        vec![n * 2024]
    }
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

fn counts_for(n: u64, blinks: i32, memo: &mut HashMap<(u64, i32), u64>) -> u64 {
    if blinks == 0 {
        1
    } else {
        let key = (n, blinks);
        if let Some(c) = memo.get(&key) {
            *c
        } else {
            let mut c = 0;
            for subn in run_rules(n) {
                c += counts_for(subn, blinks - 1, memo);
            }
            memo.insert(key, c);
            c
        }
    }
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let mut memo = HashMap::new();

    let count = data
        .data
        .iter()
        .map(|n| counts_for(*n, 25, &mut memo))
        .sum::<u64>();

    println!("{count}");

    let count = data
        .data
        .into_iter()
        .map(|n| counts_for(n, 75, &mut memo))
        .sum::<u64>();

    println!("{count}");

    Ok(())
}
