use anyhow::Error;
use aoc2024::{
    dijkstra::{DijkstraConfig, DijkstraInput, DijkstraMap},
    dp, Args,
};
use bytes::complete::tag;
use character::complete::{alpha1, multispace0, multispace1};
use clap::Parser;
use debug_print::debug_println;
use multi::{many1, separated_list1};
use nom::*;
use sequence::{terminated, tuple};
use std::{collections::HashSet, fs, path::Path};

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

#[derive(Debug)]
struct TestTarget<'a> {
    stripe_set: &'a HashSet<&'a str>,
    target: &'a str,
    max_len: usize,
}

impl<'a> DijkstraInput for TestTarget<'a> {
    type Cost = i32;

    type Index = usize;

    fn get_adjacent(&self, x: &Self::Index) -> Vec<(Self::Cost, Self::Index)> {
        let mut v = vec![];

        let pos = *x;

        for n in 1..=self.max_len {
            let end = pos + n;
            if end <= self.target.len() {
                let substr = &self.target[pos..end];
                if self.stripe_set.contains(substr) {
                    v.push((0, end));
                }
            }
        }

        v
    }
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let max_stripe_len = data.stripes.iter().map(|s| s.len()).max().unwrap();
    let stripe_set = data.stripes.iter().map(|s| s.as_str()).collect();

    dp!(stripe_set);

    let mut count = 0;
    for target in data.targets.iter() {
        let test = TestTarget {
            stripe_set: &stripe_set,
            target,
            max_len: max_stripe_len,
        };

        dp!(test);

        let mut dmap = DijkstraMap::new(&test, DijkstraConfig { print_1000: false });
        let costs = dmap.run((0, 0));
        if costs.contains_key(&target.len()) {
            let path_count =
                DijkstraMap::<TestTarget, usize>::count_all_paths(&0, &target.len(), &costs);

            count += path_count;
        }
    }

    println!("{count}");

    Ok(())
}
