use std::collections::{HashMap, HashSet};
use std::{fs, path::Path};

use anyhow::Error;
use aoc2024::{dp, Args};
use clap::Parser;
use debug_print::debug_println;
use itertools::Itertools;
use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*, *,
};

const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

#[derive(Debug)]
struct Data {
    data: HashMap<(i32, i32), char>,
    width: usize,
    height: usize,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let mut parse_problem = terminated(
        many1(terminated(
            many1(alt((alphanumeric1, tag(".")))),
            multispace0,
        )),
        alt((eof, multispace1)),
    );
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let mut data = HashMap::new();
    let mut width = 0;
    let height = problems.len();
    for (row, v) in problems.iter().enumerate() {
        width = v.len();
        for (col, c) in v.iter().flat_map(|v| v.chars()).enumerate() {
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
    assert!(data.0 == "");
    Ok(data.1)
}

fn identify_antennas(map: &HashMap<(i32, i32), char>) -> HashMap<char, Vec<(i32, i32)>> {
    map.iter().fold(HashMap::new(), |mut acc, (k, v)| {
        if *v != '.' {
            acc.entry(*v).or_insert(Vec::new()).push(k.clone());
        }
        acc
    })
}

fn mirrors_of((r1, c1): &(i32, i32), (r2, c2): &(i32, i32)) -> ((i32, i32), (i32, i32)) {
    let dr = r2 - r1;
    let dc = c2 - c1;
    ((r2 + dr, c2 + dc), (r1 - dr, c1 - dc))
}

fn display_antinode_map(map: &HashSet<(i32, i32)>, width: usize, height: usize) {
    for row in 0..height {
        for col in 0..width {
            if map.contains(&(row as i32, col as i32)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let antennas = identify_antennas(&data.data);

    dp!(antennas);

    let mut antinode_map = HashSet::new();

    for (&_antenna, positions) in antennas.iter() {
        positions.iter().combinations(2).for_each(|pos| {
            let p1 = pos[0];
            let p2 = pos[1];
            let (anti1, anti2) = mirrors_of(p1, p2);
            if data.data.contains_key(&anti1) {
                antinode_map.insert(anti1);
            }
            if data.data.contains_key(&anti2) {
                antinode_map.insert(anti2);
            }
        });
    }

    let result = antinode_map.len();

    //display_antinode_map(&antinode_map, data.width, data.height);

    println!("{result}");

    Ok(())
}
