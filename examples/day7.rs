use std::collections::VecDeque;
use std::{fs, path::Path};

use anyhow::Error;
use aoc2024::{dp, Args};
use clap::Parser;
use debug_print::debug_println;
use itertools::Itertools;
use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*, *,
};

const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

#[derive(Debug)]
struct Data {
    data: Vec<(i64, Vec<i64>)>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_right = separated_list1(space1, map_res(digit1, str::parse));
    let parse_line = separated_pair(map_res(digit1, str::parse), tag(": "), parse_right);
    let mut parse_problem = many1(terminated(parse_line, alt((eof, multispace1))));
    let (i, data) = parse_problem(i)?;

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
    assert!(data.0 == "");
    Ok(data.1)
}

fn test_line((target, nums): &(i64, Vec<i64>)) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back((nums[0], nums.iter().skip(1).cloned().collect_vec()));

    while let Some((this_target, others)) = queue.pop_front() {
        if others.is_empty() {
            if this_target == *target {
                return true;
            } else {
                continue;
            }
        } else {
            let next_num = others[0];
            let others = others.iter().skip(1).cloned().collect_vec();
            queue.push_front((this_target * next_num, others.clone()));
            queue.push_front((this_target + next_num, others));
        }
    }
    false
}

fn main() -> Result<(), Error> {
    let data = read_data()?;
    dp!(data);

    let mut count = 0;
    for line in data.data {
        let result = test_line(&line);
        if result {
            count += line.0;
        }
    }

    println!("{count}");

    Ok(())
}
