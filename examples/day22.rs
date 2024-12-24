use anyhow::Error;
use aoc2024::{dp, Args};
use character::complete::{digit1, multispace0};
use clap::Parser;
use combinator::map_res;
use debug_print::debug_println;
use multi::many1;
use nom::*;
use sequence::terminated;
use std::{fs, path::Path};

const TEST_INPUT: &str = "1
10
100
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

    let score = data
        .data
        .into_iter()
        .map(|mut secret| {
            for _n in 0..2000 {
                secret = iterate(secret);
            }
            dp!(secret);
            secret as u64
        })
        .sum::<u64>();

    println!("{score}");

    Ok(())
}
