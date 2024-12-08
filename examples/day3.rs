use std::{fs, path::Path};

use anyhow::Error;
use aoc2024::Args;
use clap::Parser;
use debug_print::debug_println;
use nom::{bytes::complete::*, character::complete::*, combinator::*, sequence::*, *};

const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

#[derive(Debug)]
struct Data {
    data: Vec<(i32, i32)>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_mul = || {
        terminated(
            preceded(
                tag("mul("),
                separated_pair(
                    map_res(digit1::<&str, ()>, str::parse),
                    tag(","),
                    map_res(digit1, str::parse),
                ),
            ),
            tag(")"),
        )
    };

    let mut data = vec![];
    let mut rest = i;
    while rest.len() > 0 {
        if let Ok((remaining, problem)) = parse_mul()(rest) {
            debug_println!("{:?}", problem);
            data.push(problem);
            rest = remaining;
        } else {
            rest = &rest[1..rest.len()];
        }
    }

    let data = Data { data };
    Ok(("", data))
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
    let data = read_data()?;

    let result = data.data.iter().fold(0, |acc, (a, b)| acc + a * b);

    println!("{:?}", result);

    Ok(())
}
