use anyhow::Error;
use aoc2024::{dp, Args};
use character::complete::{alpha1, multispace0};
use clap::Parser;
use debug_print::debug_println;
use itertools::Itertools;
use multi::many1;
use nom::*;
use sequence::terminated;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    path::Path,
};

const TEST_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

#[derive(Debug)]
struct Data {
    data: HashMap<(i32, i32), char>,
    width: usize,
    height: usize,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let mut parse_problem = many1(terminated(alpha1, multispace0));
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let mut data = HashMap::new();
    let mut width = 0;
    let height = problems.len();
    for (row, v) in problems.iter().enumerate() {
        width = v.len();
        for (col, c) in v.chars().enumerate() {
            data.insert((row as i32, col as i32), c);
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
    assert!(data.0.is_empty());
    Ok(data.1)
}

impl Data {
    fn find_adj(&self, (r, c): (i32, i32), filter: impl Fn(char) -> bool) -> Vec<(i32, i32)> {
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

    fn find_all_adj(&self, pos: (i32, i32), filter: impl Fn(char) -> bool) -> HashSet<(i32, i32)> {
        let mut queue = VecDeque::new();
        let mut result = HashSet::new();

        queue.push_front(pos);
        while let Some(v) = queue.pop_front() {
            for adj in self.find_adj(v, &filter) {
                if !result.contains(&adj) {
                    result.insert(adj);
                    queue.push_back(adj);
                }
            }
        }

        result
    }
}

fn identify_regions(data: &Data) -> HashMap<String, Vec<(i32, i32)>> {
    let mut h = HashMap::new();
    let mut work = (0..data.width)
        .flat_map(|col| {
            (0..data.height)
                .map(|row| (row as i32, col as i32))
                .collect_vec()
        })
        .collect_vec();

    let mut count = 0;
    while let Some(orig_pos) = work.pop() {
        count += 1;
        let val = *data.data.get(&orig_pos).unwrap();
        let name = {
            let mut s = String::from(val);
            s.push_str(&count.to_string());
            s
        };
        dp!(name);
        dp!(orig_pos);
        for p in data.find_all_adj(orig_pos, |c| c == val) {
            if let Some(idx) = work.iter().position(|x| *x == p) {
                dp!(p);
                work.swap_remove(idx);

                h.entry(name.clone()).or_insert_with(Vec::new).push(p);
            }
        }
        h.entry(name).or_insert_with(Vec::new).push(orig_pos);
    }
    h
}

fn calculate_cost(regions: &HashMap<String, Vec<(i32, i32)>>, data: &Data) -> u64 {
    let mut total_cost = 0;
    for (name, region) in regions.iter() {
        let mut perimeter = 0;
        for pos in region.iter() {
            macro_rules! on_edge {
                ($dr:expr, $dc:expr) => {{
                    let pos = (pos.0 + $dr, pos.1 + $dc);
                    !region.contains(&pos) || data.data.get(&pos).is_none()
                }};
            }
            if on_edge!(0, 1) {
                perimeter += 1;
            }
            if on_edge!(0, -1) {
                perimeter += 1;
            }
            if on_edge!(1, 0) {
                perimeter += 1;
            }
            if on_edge!(-1, 0) {
                perimeter += 1;
            }
        }
        let area = region.len();
        total_cost += perimeter as u64 * area as u64;
        dp!(name);
        dp!(area);
        dp!(perimeter);
    }

    total_cost
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let regions = identify_regions(&data);

    dp!(regions);

    let result = calculate_cost(&regions, &data);

    println!("{result}");

    Ok(())
}
