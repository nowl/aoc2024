use anyhow::Error;
use aoc2024::{dp, Args};
use character::complete::{digit1, multispace0};
use clap::Parser;
use combinator::map_res;
use debug_print::debug_println;
use itertools::Itertools;
use multi::many1;
use nom::*;
use sequence::terminated;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

const TEST_INPUT1: &str = "1
10
100
2024";

const TEST_INPUT: &str = "1
2
3
2024";

#[derive(Debug)]
struct Data {
    data: Vec<i32>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let mut parse_problem = many1(terminated(map_res(digit1, str::parse), multispace0));
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let data = Data { data: problems };
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

fn iterate(n: i32) -> i32 {
    let n2 = (n ^ (n * 64)) % 16777216;
    let n3 = ((n2 ^ (n2 / 32)) % 16777216) as i64;
    let n4 = (n3 ^ (n3 * 2048)) % 16777216;
    n4 as i32
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    // part 1

    let score = data
        .data
        .iter()
        .cloned()
        .map(|mut secret| {
            for _n in 0..2000 {
                secret = iterate(secret);
            }
            dp!(secret);
            secret as u64
        })
        .sum::<u64>();

    println!("{score}");

    // part 2

    // build diff map
    let diffs = data
        .data
        .into_iter()
        .map(|mut secret| {
            let single_digits = (0..2000)
                .into_iter()
                .fold(vec![secret], |mut acc, _n| {
                    secret = iterate(secret);
                    acc.push(secret);
                    acc
                })
                .into_iter()
                .map(|v| v % 10)
                .collect_vec();

            let mut diffs = HashMap::new();

            single_digits
                .windows(5)
                .map(|v| ((v[1] - v[0], v[2] - v[1], v[3] - v[2], v[4] - v[3]), v[4]))
                .for_each(|(key, v)| {
                    diffs.entry(key).or_insert(v);
                });

            diffs
        })
        .collect_vec();

    // collect all sequences
    let sequences: HashSet<_> = diffs.iter().flat_map(|x| x.keys()).collect();

    // find best sequence
    let mut best_sequence: Option<(_, i32)> = None;
    for sequence in sequences {
        let winnings = diffs
            .iter()
            .fold(0, |acc, v| acc + v.get(sequence).map(|x| *x).unwrap_or(0));
        if best_sequence.is_none_or(|x| x.1 < winnings) {
            best_sequence = Some((sequence, winnings));
            dp!(best_sequence);
        }
    }

    println!("{}", best_sequence.unwrap().1);

    Ok(())
}
