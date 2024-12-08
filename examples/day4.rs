use std::collections::HashMap;
use std::{fs, path::Path};

use anyhow::Error;
use aoc2024::Args;
use clap::Parser;
use debug_print::debug_println;
use itertools::Itertools;
use nom::{character::complete::*, multi::*, sequence::*, *};

const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[derive(Debug)]
struct Data {
    data: HashMap<(usize, usize), char>,
    width: usize,
    height: usize,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let mut parse_problem = many1(terminated(alpha1, multispace0));
    let (i, problems) = parse_problem(i)?;

    debug_println!("{:?}", problems);

    let mut data = HashMap::new();
    let mut width = 0;
    let height = problems.len();
    for (row, v) in problems.iter().enumerate() {
        width = v.len();
        for (col, c) in v.chars().enumerate() {
            data.insert((row, col), c);
        }
    }

    let data = Data {
        data,
        width,
        height,
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
    assert!(data.0 == "");
    Ok(data.1)
}

fn check_position(r: i32, c: i32, data: &Data) -> usize {
    macro_rules! get {
        ($r:expr, $c:expr) => {
            if $r < 0 || $c < 0 {
                None
            } else {
                data.data.get(&($r as usize, $c as usize))
            }
        };
    }

    let mut count = 0;

    macro_rules! check {
        (($r1:expr, $c1:expr), ($r2:expr, $c2:expr), ($r3:expr, $c3:expr), ($r4:expr, $c4:expr)) => {
            if get![$r1, $c1] == Some(&'X')
                && get![$r2, $c2] == Some(&'M')
                && get![$r3, $c3] == Some(&'A')
                && get![$r4, $c4] == Some(&'S')
            {
                count += 1;
            }
        };
    }

    check!((r, c), (r, c + 1), (r, c + 2), (r, c + 3));
    check!((r, c), (r, c - 1), (r, c - 2), (r, c - 3));
    check!((r, c), (r + 1, c), (r + 2, c), (r + 3, c));
    check!((r, c), (r - 1, c), (r - 2, c), (r - 3, c));

    check!((r, c), (r + 1, c + 1), (r + 2, c + 2), (r + 3, c + 3));
    check!((r, c), (r - 1, c + 1), (r - 2, c + 2), (r - 3, c + 3));
    check!((r, c), (r + 1, c - 1), (r + 2, c - 2), (r + 3, c - 3));
    check!((r, c), (r - 1, c - 1), (r - 2, c - 2), (r - 3, c - 3));

    count
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    let mut count = 0;
    for row in 0..data.height {
        for col in 0..data.width {
            let r = check_position(row as i32, col as i32, &data);
            count += r;
        }
    }

    println!("{:?}", count);

    Ok(())
}
