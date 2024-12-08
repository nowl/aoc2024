use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::{fs, path::Path};

use anyhow::Error;
use aoc2024::{dp, Args};
use character::streaming::multispace0;
use clap::Parser;
use debug_print::debug_println;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*, *,
};

const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

#[derive(Debug)]
struct Data {
    orderings: Vec<(i32, i32)>,
    updates: Vec<Vec<i32>>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_line = separated_pair(
        map_res(digit1, str::parse),
        tag("|"),
        map_res(digit1, str::parse),
    );
    let mut parse_problem = many1(terminated(parse_line, newline));
    let (i, orderings) = parse_problem(i)?;

    debug_println!("{:?}", orderings);

    let (i, _) = multispace0(i)?;

    let parse_line = terminated(
        separated_list1(tag(","), map_res(digit1, str::parse)),
        alt((multispace1, eof)),
    );
    let mut parse_problem = many1(parse_line);
    let (i, updates) = parse_problem(i)?;

    debug_println!("{:?}", updates);

    let data = Data { orderings, updates };
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

fn build_sort_fun(order: HashMap<i32, HashSet<i32>>) -> impl Fn(&i32, &i32) -> Ordering {
    move |a, b| {
        if a == b {
            return Ordering::Equal;
        }

        if let Some(v) = order.get(a) {
            if v.contains(b) {
                return Ordering::Less;
            }
        }

        Ordering::Greater
    }
}

fn main() -> Result<(), Error> {
    let data = read_data()?;
    dp!(data);

    let order_table = {
        let mut m = HashMap::new();
        for (k, v) in data.orderings.iter() {
            m.entry(*k).or_insert_with(HashSet::new).insert(*v);
        }
        m
    };

    let sort_func = build_sort_fun(order_table);

    let mut result = 0;
    for update in data.updates.iter() {
        let update_sorted = {
            let mut sorted = update.clone();
            sorted.sort_by(&sort_func);
            sorted
        };
        if &update_sorted == update {
            if update.len() % 2 == 1 {
                result += update[update.len() / 2];
            } else {
                result += (update[update.len() / 2] + update[update.len() / 2 - 1]) / 2;
            }
        }
    }

    println!("{result}");

    Ok(())
}
