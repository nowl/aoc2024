use std::{fs, path::Path};

use anyhow::Error;
use aoc2024::Args;
use clap::Parser;
use itertools::Itertools;
use nom::{character::complete::*, combinator::*, multi::*, sequence::*, *};

const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

#[derive(Debug)]
struct Data {
    col1: Vec<i32>,
    col2: Vec<i32>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_line = tuple((
        map_res(digit1, str::parse),
        multispace1,
        map_res(digit1, str::parse),
    ));
    let mut parse_problem = many1(terminated(parse_line, multispace0));
    let (i, problems) = parse_problem(i)?;

    let col1 = problems.iter().map(|(x, _, _)| x).cloned().collect_vec();
    let col2 = problems.iter().map(|(_, _, x)| x).cloned().collect_vec();

    let data = Data { col1, col2 };
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
    assert!(data.0 == "");
    Ok(data.1)
}

fn main() -> Result<(), Error> {
    let mut data = read_data()?;

    data.col1.sort_by(|a, b| a.cmp(b));
    data.col2.sort_by(|a, b| a.cmp(b));

    let result = data
        .col1
        .iter()
        .zip_eq(data.col2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    println!("{}", result);

    // part 2

    let counts = data.col2.iter().counts();

    let result = data
        .col1
        .iter()
        .map(|&x| x as usize * counts.get(&x).map_or(0, |x| *x))
        .sum::<usize>();

    println!("{}", result);
    Ok(())
}
