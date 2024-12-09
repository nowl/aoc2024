use std::collections::{HashMap, HashSet};
use std::{fs, path::Path};

use anyhow::Error;
use aoc2024::{dp, Args};
use clap::Parser;
use debug_print::debug_println;
use nom::{branch::*, character::complete::*, combinator::*, multi::*, sequence::*, *};

const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[derive(Debug, Clone)]
enum Tile {
    Open(bool),
    Blocked,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Data {
    data: HashMap<(usize, usize), Tile>,
    width: usize,
    height: usize,
    guard: ((usize, usize), Facing),
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let mut parse_problem = many1(terminated(many1(one_of(".^#")), alt((eof, multispace1))));
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let mut data = HashMap::new();
    let mut width = 0;
    let mut guard = ((0, 0), Facing::Up);
    let height = problems.len();
    for (row, v) in problems.iter().enumerate() {
        width = v.len();
        for (col, c) in v.iter().enumerate() {
            let t = match *c {
                '.' => Tile::Open(false),
                '^' => {
                    guard = ((row, col), Facing::Up);
                    Tile::Open(true)
                }
                '#' => Tile::Blocked,
                _ => unreachable!(),
            };
            data.insert((row, col), t);
        }
    }

    let data = Data {
        data,
        width,
        height,
        guard,
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

#[derive(PartialEq, Eq)]
enum StepResult {
    InBounds,
    OutOfBounds,
    InLoop,
}

fn step(data: &mut Data, prev_positions: &mut HashSet<((usize, usize), Facing)>) -> StepResult {
    use StepResult::*;

    let ((grow, gcol), ref facing) = data.guard;

    macro_rules! get {
        ($dr:expr, $dc:expr) => {{
            let r = grow as i32 + $dr;
            let c = gcol as i32 + $dc;
            if r < 0 || c < 0 {
                None
            } else {
                data.data.get(&((r as usize, c as usize)))
            }
        }};
    }

    macro_rules! move_or_turn {
        ($dr:expr, $dc:expr, $fcur:expr, $fnew:expr) => {
            match get!($dr, $dc) {
                None => OutOfBounds,
                Some(Tile::Open(_)) => {
                    let new_pos = ((grow as i32 + $dr) as usize, (gcol as i32 + $dc) as usize);
                    data.guard = (new_pos, $fcur);
                    data.data
                        .entry(new_pos)
                        .and_modify(|t| *t = Tile::Open(true));
                    if prev_positions.contains(&data.guard) {
                        InLoop
                    } else {
                        prev_positions.insert(data.guard.clone());
                        InBounds
                    }
                }
                Some(Tile::Blocked) => {
                    data.guard = ((grow, gcol), $fnew);
                    InBounds
                }
            }
        };
    }

    use Facing::*;

    match facing {
        Facing::Up => move_or_turn!(-1, 0, Up, Right),
        Facing::Down => move_or_turn!(1, 0, Down, Left),
        Facing::Left => move_or_turn!(0, -1, Left, Up),
        Facing::Right => move_or_turn!(0, 1, Right, Down),
    }
}

fn main() -> Result<(), Error> {
    let data = read_data()?;
    dp!(data);

    let mut count = 0;
    for col in 0..data.height {
        for row in 0..data.width {
            if let Some(Tile::Blocked) = data.data.get(&(row, col)) {
                continue;
            }

            let mut prev_positions = HashSet::new();
            let mut data = data.clone();
            // add obstruction
            data.data
                .entry((row, col))
                .and_modify(|v| *v = Tile::Blocked);

            loop {
                let result = step(&mut data, &mut prev_positions);
                match result {
                    StepResult::InBounds => (),
                    StepResult::OutOfBounds => {
                        break;
                    }
                    StepResult::InLoop => {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("{count}");

    Ok(())
}
