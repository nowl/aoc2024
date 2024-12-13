use anyhow::Error;
use aoc2024::{dp, Args};
use bytes::complete::tag;
use character::complete::{digit1, multispace0};
use clap::Parser;
use combinator::map_res;
use debug_print::debug_println;
use multi::many1;
use nom::*;
use num::Integer;
use sequence::{preceded, separated_pair, terminated, tuple};
use std::{fs, path::Path};

const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

#[derive(Debug)]
struct Game {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize_loc: (i64, i64),
}

#[derive(Debug)]
struct Data {
    data: Vec<Game>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_val = |ptag| {
        preceded(
            multispace0,
            preceded(tag(ptag), map_res(digit1, str::parse)),
        )
    };
    let parse_xy = || separated_pair(parse_val("X+"), tag(","), parse_val("Y+"));
    let parse_xyeq = separated_pair(parse_val("X="), tag(","), parse_val("Y="));

    let parse_game = tuple((
        terminated(preceded(tag("Button A:"), parse_xy()), multispace0),
        terminated(preceded(tag("Button B:"), parse_xy()), multispace0),
        terminated(preceded(tag("Prize:"), parse_xyeq), multispace0),
    ));
    let mut parse_problem = many1(terminated(parse_game, multispace0));
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let data = problems
        .into_iter()
        .map(|p| Game {
            button_a: p.0,
            button_b: p.1,
            prize_loc: (p.2 .0 + 10000000000000, p.2 .1 + 10000000000000),
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

fn solve_game(game: &Game) -> Option<((i64, i64), i64)> {
    let (ax, ay) = game.button_a;
    let (bx, by) = game.button_b;
    let (px, py) = game.prize_loc;
    let bnum = ax * py - ay * px;
    let bden = ax * by - ay * bx;
    let b_and_rem = bnum.div_rem(&bden);

    if b_and_rem.1 != 0 {
        // no solution
        None
    } else {
        let b = b_and_rem.0;
        let anum = px - b * bx;
        let a_and_rem = anum.div_rem(&ax);
        if a_and_rem.1 != 0 {
            // again no solution
            None
        } else {
            let a = a_and_rem.0;
            let cost = a * 3 + b;
            Some(((a, b), cost))
        }
    }
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let mut tokens = 0;
    for game in data.data {
        dp!(game);
        let solution = solve_game(&game);
        dp!(solution);

        if let Some((_, cost)) = solution {
            tokens += cost;
        }
    }

    println!("{tokens}");

    Ok(())
}
