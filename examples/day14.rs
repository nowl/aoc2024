use anyhow::Error;
use aoc2024::{dp, Args};
use bytes::complete::tag;
use character::complete::{char, digit1, multispace0, multispace1};
use clap::Parser;
use combinator::{map_res, opt};
use debug_print::debug_println;
use itertools::Itertools;
use multi::many1;
use nom::*;
use num::Integer;
use sequence::{preceded, separated_pair, terminated, tuple};
use std::{fs, path::Path};

const TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

#[derive(Debug)]
struct Data {
    data: Vec<Robot>,
    width: usize,
    height: usize,
}

fn parse_data(i: &str, width: usize, height: usize) -> IResult<&str, Data> {
    let parse_val = || {
        map_res(
            tuple((opt(char('-')), digit1)),
            |(sign, num): (Option<char>, &str)| {
                let n: i32 = num.parse()?;
                if sign.is_some() {
                    Ok::<i32, Error>(-n)
                } else {
                    Ok(n)
                }
            },
        )
    };

    let parse_xy = || separated_pair(parse_val(), tag(","), parse_val());
    let parse_p = preceded(tag("p="), parse_xy());
    let parse_v = preceded(tag("v="), parse_xy());
    let parse_robot = tuple((parse_p, preceded(multispace1, parse_v)));
    let mut parse_problem = many1(terminated(parse_robot, multispace0));
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let data = problems
        .into_iter()
        .map(|(pos, vel)| Robot { pos, vel })
        .collect();

    let data = Data {
        data,
        width,
        height,
    };
    Ok((i, data))
}

fn read_data() -> Result<Data, Error> {
    let args = Args::parse();

    let mut width = 11;
    let mut height = 7;

    let contents = args.file.map_or(Ok(TEST_INPUT.to_string()), |input| {
        width = 101;
        height = 103;
        let file = Path::new(&input);
        fs::read_to_string(file)
    })?;

    let data = parse_data(&contents, width, height);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0.is_empty());
    Ok(data.1)
}

impl Robot {
    fn step(&mut self, width: usize, height: usize) {
        self.pos.0 = (self.pos.0 + self.vel.0).mod_floor(&(width as i32));
        self.pos.1 = (self.pos.1 + self.vel.1).mod_floor(&(height as i32));
    }

    fn quadrant(&self, width: usize, height: usize) -> (Option<usize>, Option<usize>) {
        let x = if (self.pos.0 as usize) < width / 2 {
            Some(0)
        } else if (self.pos.0 as usize) > width / 2 {
            Some(1)
        } else {
            None
        };

        let y = if (self.pos.1 as usize) < height / 2 {
            Some(0)
        } else if (self.pos.1 as usize) > height / 2 {
            Some(1)
        } else {
            None
        };

        (x, y)
    }
}

fn calc_score(robots: &Vec<Robot>, width: usize, height: usize) -> u64 {
    let quads = robots
        .iter()
        .map(|r| r.quadrant(width, height))
        .filter(|(x, y)| x.is_some() && y.is_some())
        .counts();

    dp!(quads);

    quads.into_values().map(|x| x as u64).product()
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let mut robots = data.data.clone();
    for r in robots.iter_mut() {
        dp!(r);
        for _step in 0..100 {
            r.step(data.width, data.height);
        }
        dp!(r);
    }

    dp!(robots);

    let score = calc_score(&robots, data.width, data.height);

    println!("{score}");

    Ok(())
}
