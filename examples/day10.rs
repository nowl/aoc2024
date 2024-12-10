use std::collections::{HashMap, HashSet, VecDeque};
use std::{fs, path::Path};

use anyhow::Error;
use aoc2024::{dp, Args};
use clap::Parser;
use debug_print::debug_println;
use itertools::Itertools;
use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*, *,
};

const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

#[derive(Debug)]
struct Data {
    data: HashMap<(i32, i32), u32>,
    width: usize,
    height: usize,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_single_digit = map_res(anychar, |c| c.to_digit(10).ok_or("bad parse"));
    let mut parse_problem = many1(terminated(
        many1(parse_single_digit),
        alt((eof, multispace1)),
    ));
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let mut data = HashMap::new();
    let mut width = 0;
    let height = problems.len();
    for (row, v) in problems.iter().enumerate() {
        width = v.len();
        for (col, c) in v.iter().enumerate() {
            data.insert((row as i32, col as i32), *c);
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

impl Data {
    fn adjacent_to(&self, (r, c): (i32, i32), filter: impl Fn(u32) -> bool) -> Vec<(i32, i32)> {
        let mut adjs = Vec::new();

        macro_rules! check_and_add {
            ($dr:expr, $dc:expr) => {
                let pos = (r + $dr, c + $dc);
                if self.data.get(&pos).filter(|&v| filter(*v)).is_some() {
                    adjs.push(pos)
                }
            };
        }

        check_and_add!(-1, 0);
        check_and_add!(1, 0);
        check_and_add!(0, -1);
        check_and_add!(0, 1);
        adjs
    }

    fn find_trailheads(&self) -> Vec<&(i32, i32)> {
        self.data
            .iter()
            .filter_map(|(k, v)| if *v == 0 { Some(k) } else { None })
            .collect()
    }
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let mut count = 0;
    let trailheads = data.find_trailheads();
    for trailhead in trailheads {
        dp!(trailhead);

        let mut endings = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((trailhead.clone(), 1));

        while let Some((pos, next)) = queue.pop_front() {
            for adj in data.adjacent_to(pos, |v| v == next) {
                if next == 9 {
                    endings.insert(adj);
                } else {
                    queue.push_back((adj, next + 1));
                }
            }
        }

        dp!(endings);

        count += endings.len();
    }

    println!("{count}");

    Ok(())
}
