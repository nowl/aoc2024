use anyhow::Error;
use aoc2024::{dp, Args};
use character::complete::{multispace0, one_of};
use clap::Parser;
use debug_print::debug_println;
use itertools::Itertools;
use multi::{count, many1};
use nom::*;
use sequence::terminated;
use std::{fs, path::Path};

const TEST_INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

#[derive(Debug)]
enum Tumbler {
    Lock(Vec<i32>),
    Key(Vec<i32>),
}

#[derive(Debug)]
struct Data {
    data: Vec<Tumbler>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_line = terminated(count(one_of(".#"), 5), multispace0);
    let parse_lock = terminated(count(parse_line, 7), multispace0);
    let mut parse_problem = many1(parse_lock);
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let data = problems
        .into_iter()
        .map(|v| {
            // build tumbler
            if v[0][0] == '.' {
                // key
                let mut v2 = vec![];
                for col in 0..5 {
                    let mut count = 0;
                    for row in 0..7 {
                        match v[row][col] {
                            '#' => count += 1,
                            _ => (),
                        }
                    }
                    v2.push(count - 1);
                }
                Tumbler::Key(v2)
            } else if v[0][0] == '#' {
                //lock
                let mut v2 = vec![];
                for col in 0..5 {
                    let mut count = 0;
                    for row in 0..7 {
                        match v[row][col] {
                            '#' => count += 1,
                            _ => (),
                        }
                    }
                    v2.push(count - 1);
                }
                Tumbler::Lock(v2)
            } else {
                unreachable!()
            }
        })
        .collect();

    let data = Data { data };
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

fn key_lock_fit(key: &Tumbler, lock: &Tumbler) -> bool {
    let Tumbler::Key(key) = key else {
        unreachable!();
    };

    let Tumbler::Lock(lock) = lock else {
        unreachable!();
    };

    key.iter().zip_eq(lock.iter()).all(|(a, b)| a + b <= 5)
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let locks = data
        .data
        .iter()
        .filter(|&x| match x {
            Tumbler::Lock(_) => true,
            Tumbler::Key(_) => false,
        })
        .collect_vec();

    let keys = data
        .data
        .iter()
        .filter(|&x| match x {
            Tumbler::Lock(_) => false,
            Tumbler::Key(_) => true,
        })
        .collect_vec();

    let mut count = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if key_lock_fit(key, lock) {
                count += 1;
            }
        }
    }

    println!("{}", count);

    Ok(())
}
