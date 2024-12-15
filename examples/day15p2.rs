use anyhow::Error;
use aoc2024::{dp, read_line, Args};
use character::complete::{multispace0, one_of};
use clap::Parser;
use debug_print::debug_println;
use multi::many1;
use nom::*;
use sequence::{terminated, tuple};
use std::{
    collections::HashMap,
    fmt::{Display, Write},
    fs,
    path::Path,
};

const TEST_INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Floor,
    Wall,
    BoxL,
    BoxR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Debug)]
struct Data {
    data: HashMap<(i32, i32), Tile>,
    width: usize,
    height: usize,
    robot: (i32, i32),
    directions: Vec<Direction>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_map = many1(terminated(many1(one_of(".#O@")), multispace0));
    let parse_directions = many1(terminated(many1(one_of("<>v^")), multispace0));
    let mut parse_problem = tuple((
        terminated(parse_map, multispace0),
        terminated(parse_directions, multispace0),
    ));
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let mut robot = (0, 0);
    let mut width = 0;
    let height = problems.0.len();
    let mut data = HashMap::new();
    problems.0.into_iter().enumerate().for_each(|(r, row)| {
        width = row.len() * 2;
        row.into_iter().enumerate().for_each(|(c, ch)| {
            use Tile::*;
            for n in 0..2 {
                let tile = match ch {
                    '#' => Wall,
                    'O' => match n {
                        0 => BoxL,
                        1 => BoxR,
                        _ => unreachable!(),
                    },
                    '.' => Floor,
                    '@' => {
                        robot = (r as i32, c as i32 * 2);
                        Floor
                    }
                    _ => unreachable!(),
                };
                data.insert((r as i32, c as i32 * 2 + n), tile);
            }
        })
    });

    let directions = problems
        .1
        .into_iter()
        .flat_map(|row| {
            row.into_iter().map(|c| match c {
                'v' => Direction::S,
                '^' => Direction::N,
                '<' => Direction::W,
                '>' => Direction::E,
                _ => unreachable!(),
            })
        })
        .collect();

    let data = Data {
        data,
        width,
        height,
        robot,
        directions,
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

fn delta_spot((r, c): (i32, i32), direction: Direction) -> (i32, i32) {
    use Direction::*;
    match direction {
        N => (r - 1, c),
        S => (r + 1, c),
        E => (r, c + 1),
        W => (r, c - 1),
    }
}

fn push_ew(
    map: &mut HashMap<(i32, i32), Tile>,
    spot: (i32, i32),
    direction: Direction,
    to_push: Option<Tile>,
    perform_push: bool,
) -> bool {
    use Tile::*;
    let opposite_dir = match direction {
        Direction::E => Direction::W,
        Direction::W => Direction::E,
        _ => unreachable!(),
    };
    let old_spot = delta_spot(spot, opposite_dir);
    let new_spot = delta_spot(spot, direction);
    let new_spot2 = delta_spot(new_spot, direction);
    let Some(new_tile) = map.get(&(new_spot)) else {
        unreachable!();
    };

    match new_tile {
        Floor => {
            if to_push.is_some() && perform_push {
                match direction {
                    Direction::W => {
                        *map.get_mut(&new_spot).unwrap() = BoxL;
                        *map.get_mut(&spot).unwrap() = BoxR;
                        *map.get_mut(&old_spot).unwrap() = Floor;
                    }
                    Direction::E => {
                        *map.get_mut(&new_spot).unwrap() = BoxR;
                        *map.get_mut(&spot).unwrap() = BoxL;
                        *map.get_mut(&old_spot).unwrap() = Floor;
                    }
                    _ => unreachable!(),
                }
            }

            true
        }
        Wall => false,
        BoxR => {
            // recursively try to push box
            debug_assert!(direction == Direction::W);
            if push_ew(map, new_spot2, direction, Some(BoxL), perform_push) {
                if to_push.is_some() && perform_push {
                    *map.get_mut(&new_spot).unwrap() = BoxL;
                    *map.get_mut(&spot).unwrap() = BoxR;
                    *map.get_mut(&old_spot).unwrap() = Floor;
                }
                true
            } else {
                false
            }
        }
        BoxL => {
            // recursively try to push box
            debug_assert!(direction == Direction::E);
            if push_ew(map, new_spot2, direction, Some(BoxR), perform_push) {
                if to_push.is_some() && perform_push {
                    *map.get_mut(&new_spot).unwrap() = BoxR;
                    *map.get_mut(&spot).unwrap() = BoxL;
                    *map.get_mut(&old_spot).unwrap() = Floor;
                }
                true
            } else {
                false
            }
        }
        _ => unreachable!(),
    }
}

fn push_ns(
    map: &mut HashMap<(i32, i32), Tile>,
    spot: (i32, i32),
    direction: Direction,
    to_push: Option<Tile>,
    perform_push: bool,
) -> bool {
    use Tile::*;

    let new_spot = delta_spot(spot, direction);
    let Some(new_tile) = map.get(&(new_spot)) else {
        unreachable!();
    };

    match new_tile {
        Floor => {
            if to_push.is_some() && perform_push {
                *map.get_mut(&new_spot).unwrap() = to_push.unwrap();
                *map.get_mut(&spot).unwrap() = Floor;
            }

            true
        }
        Wall => false,
        BoxL => {
            // recursively try to push box
            let new_spot2 = delta_spot(new_spot, Direction::E);
            if push_ns(map, new_spot, direction, None, perform_push)
                && push_ns(map, new_spot2, direction, None, perform_push)
            {
                push_ns(map, new_spot, direction, Some(BoxL), perform_push);
                push_ns(map, new_spot2, direction, Some(BoxR), perform_push);
                if to_push.is_some() && perform_push {
                    *map.get_mut(&new_spot).unwrap() = BoxL;
                    *map.get_mut(&new_spot2).unwrap() = BoxR;
                    *map.get_mut(&spot).unwrap() = Floor;
                    *map.get_mut(&delta_spot(spot, Direction::E)).unwrap() = Floor;
                }
                true
            } else {
                false
            }
        }
        BoxR => {
            // recursively try to push box
            let new_spot2 = delta_spot(new_spot, Direction::W);
            if push_ns(map, new_spot, direction, None, perform_push)
                && push_ns(map, new_spot2, direction, None, perform_push)
            {
                push_ns(map, new_spot, direction, Some(BoxR), perform_push);
                push_ns(map, new_spot2, direction, Some(BoxL), perform_push);
                if to_push.is_some() && perform_push {
                    *map.get_mut(&new_spot).unwrap() = BoxR;
                    *map.get_mut(&new_spot2).unwrap() = BoxL;
                    *map.get_mut(&spot).unwrap() = Floor;
                    *map.get_mut(&delta_spot(spot, Direction::W)).unwrap() = Floor;
                }
                true
            } else {
                false
            }
        }
        _ => unreachable!(),
    }
}

fn push_dir(
    map: &mut HashMap<(i32, i32), Tile>,
    spot: (i32, i32),
    direction: Direction,
    to_push: Option<Tile>,
    perform_push: bool,
) -> bool {
    match direction {
        Direction::N | Direction::S => push_ns(map, spot, direction, to_push, perform_push),
        Direction::E | Direction::W => push_ew(map, spot, direction, to_push, perform_push),
    }
}

fn step(map: &mut HashMap<(i32, i32), Tile>, robot: &mut (i32, i32), direction: Direction) {
    let new_spot = delta_spot(*robot, direction);

    if push_dir(map, *robot, direction, None, false) {
        push_dir(map, *robot, direction, None, true);
        #[cfg(debug_assertions)]
        {
            let Some(tile) = map.get(&(new_spot)) else {
                unreachable!();
            };
            dp!(new_spot);
            dp!(tile);
            debug_assert!(*tile == Tile::Floor);
        }
        *robot = new_spot;
        dp!(robot);
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.height {
            for c in 0..self.width {
                use Tile::*;
                let ch = if self.robot == (r as i32, c as i32) {
                    '@'
                } else {
                    match self.data.get(&(r as i32, c as i32)) {
                        Some(t) => match t {
                            Floor => '.',
                            Wall => '#',
                            BoxL => '[',
                            BoxR => ']',
                        },
                        None => unreachable!(),
                    }
                };
                f.write_char(ch)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    debug_println!("{data}");

    let mut map = data.data.clone();
    let mut robot_position = data.robot;
    let mut count = 0;
    for dir in data.directions {
        count += 1;
        dp!(count);
        dp!(dir);
        step(&mut map, &mut robot_position, dir);
        dp!(robot_position);
        #[cfg(debug_assertions)]
        {
            let disp_data = Data {
                data: map.clone(),
                width: data.width,
                height: data.height,
                robot: robot_position,
                directions: vec![],
            };
            debug_println!("{disp_data}");

            read_line()?;
            //sleep(Duration::from_millis(250));
        }
    }

    // calc score

    let score = map
        .into_iter()
        .filter(|(_k, v)| *v == Tile::BoxL)
        .map(|(k, _v)| k)
        .fold(0, |acc, (r, c)| acc + 100 * r + c);

    println!("{score}");

    Ok(())
}
