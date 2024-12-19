use anyhow::Error;
use aoc2024::{dp, Args};
use bytes::complete::tag;
use character::complete::{alpha1, multispace0, multispace1};
use clap::Parser;
use debug_print::debug_println;
use multi::{many1, separated_list1};
use nom::*;
use sequence::{terminated, tuple};
use std::{fs, path::Path};

const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

#[derive(Debug)]
struct Data {
    stripes: Vec<String>,
    targets: Vec<String>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_stripes = separated_list1(terminated(tag(","), multispace0), alpha1);
    let parse_targets = many1(terminated(alpha1, multispace0));

    let mut parse_problem = tuple((terminated(parse_stripes, multispace1), parse_targets));
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let (stripes, targets) = problems;

    let data = Data {
        stripes: stripes.into_iter().map(|s| s.to_string()).collect(),
        targets: targets.into_iter().map(|s| s.to_string()).collect(),
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

fn is_possible(target: &str, pos: usize, stripes: &Vec<String>) -> bool {
    let sub = &target[pos..];

    for stripe in stripes.iter() {
        if sub == stripe {
            return true;
        } else if stripe.len() <= sub.len() && &sub[0..stripe.len()] == stripe {
            debug_println!("{} matches", stripe);
            if is_possible(target, pos + stripe.len(), stripes) {
                return true;
            }
        }
    }

    false
}

fn reduce_stripes(stripes: &Vec<String>) -> Vec<String> {
    let mut to_remove = vec![];
    for s in stripes.iter() {
        for s2 in stripes.iter() {
            if s != s2 && s2.starts_with(s) && stripes.contains(&s2[s.len()..].to_string()) {
                to_remove.push(s2);
            }
        }
    }

    stripes
        .iter()
        .filter(|s| !to_remove.contains(s))
        .cloned()
        .collect()
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let reduced = reduce_stripes(&data.stripes);

    debug_println!("{:?}", reduced);

    let mut count = 0;
    for target in data.targets.iter() {
        debug_println!("examining {}", target);
        let possible = is_possible(target, 0, &reduced);
        debug_println!("{}, possible: {}", target, possible);
        if possible {
            count += 1;
        }
        dp!(possible);
    }

    println!("{count}");

    Ok(())
}
