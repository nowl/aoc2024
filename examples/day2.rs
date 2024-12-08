use std::{fs, path::Path};

use anyhow::Error;
use aoc2024::Args;
use clap::Parser;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use nom::{character::complete::*, combinator::*, multi::*, sequence::*, *};

const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

#[derive(Debug)]
struct Data {
    data: Vec<Vec<i32>>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_line = separated_list1(space1, map_res(digit1, str::parse));
    let mut parse_problem = many1(terminated(parse_line, multispace0));
    let (i, problems) = parse_problem(i)?;

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
    assert!(data.0 == "");
    Ok(data.1)
}

fn check_line(line: &Vec<i32>) -> bool {
    let result = line.iter().tuple_windows().fold_while(None, |acc, (a, b)| {
        let diff = (b - a).abs();
        let dir = (b - a).signum();
        if diff > 3 || diff < 1 {
            Done(Some((false, 0)))
        } else {
            match acc {
                None => Continue(Some((true, dir))),
                Some((_, edir)) if edir == dir => Continue(Some((true, dir))),
                _ => Done(Some((false, 0))),
            }
        }
    });
    !result.is_done()
}

fn check_line2(line: &Vec<i32>) -> bool {
    if check_line(line) {
        return true;
    }

    // remove values one at a time to test
    for n in 0..line.len() {
        let line2 = {
            let mut r = line.clone();
            r.remove(n);
            r
        };

        if check_line(&line2) {
            return true;
        }
    }

    false
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    let result = data
        .data
        .iter()
        .map(|line| check_line(line))
        .filter(|line| *line)
        .count();
    println!("{:?}", result);

    // part 2

    let result = data
        .data
        .iter()
        .map(|line| check_line2(line))
        .filter(|line| *line)
        .count();
    println!("{:?}", result);

    Ok(())
}
