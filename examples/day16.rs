use anyhow::Error;
use aoc2024::{
    dijkstra::{DijkstraConfig, DijkstraInput, DijkstraMap},
    dp, Args,
};
use character::complete::{multispace0, one_of};
use clap::Parser;
use debug_print::debug_println;
use multi::many1;
use nom::*;
use sequence::terminated;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

const TEST_INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Facing {
    N,
    S,
    E,
    W,
}

#[derive(Debug)]
struct Data {
    map: HashMap<(i32, i32), Tile>,
    width: usize,
    height: usize,
    start: (i32, i32),
    end: (i32, i32),
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    use Tile::*;

    let parse_val = one_of(".#SE");
    let mut parse_problem = terminated(
        many1(terminated(many1(parse_val), multispace0)),
        multispace0,
    );
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let mut start = (0, 0);
    let mut end = start.clone();
    let mut map = HashMap::new();

    let height = problems.len();
    let mut width = 0;
    for (ridx, r) in problems.into_iter().enumerate() {
        width = r.len();
        for (cidx, c) in r.into_iter().enumerate() {
            let pos = (ridx as i32, cidx as i32);
            let tile = match c {
                '.' => Empty,
                '#' => Wall,
                'S' => {
                    start = pos.clone();
                    Empty
                }
                'E' => {
                    end = pos.clone();
                    Empty
                }
                _ => unreachable!(),
            };
            map.insert(pos, tile);
        }
    }

    let data = Data {
        map,
        width,
        height,
        start,
        end,
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

impl DijkstraInput for Data {
    type Cost = i32;

    type Index = ((i32, i32), Facing);

    fn get_adjacent(&self, x: &Self::Index) -> Vec<(Self::Cost, Self::Index)> {
        use Facing::*;
        let &(pos, facing) = x;

        let mut v = vec![];

        macro_rules! forward_test {
            ($dr:expr, $dc:expr) => {{
                let pos = (pos.0 + $dr, pos.1 + $dc);
                if self.map.get(&pos).is_some_and(|t| *t == Tile::Empty) {
                    v.push((1, (pos, facing)));
                }
            }};
        }

        match facing {
            N => {
                v.push((1000, (pos, E)));
                v.push((1000, (pos, W)));
                forward_test!(-1, 0)
            }
            S => {
                v.push((1000, (pos, E)));
                v.push((1000, (pos, W)));
                forward_test!(1, 0)
            }
            E => {
                v.push((1000, (pos, N)));
                v.push((1000, (pos, S)));
                forward_test!(0, 1)
            }
            W => {
                v.push((1000, (pos, N)));
                v.push((1000, (pos, S)));
                forward_test!(0, -1)
            }
        }

        v
    }
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let mut dmap = DijkstraMap::new(&data, DijkstraConfig { print_1000: true });

    let start = (0, (data.start, Facing::E));
    let costs = dmap.run(start);

    let cost = costs
        .iter()
        .filter(|(k, _)| k.0 == data.end)
        .map(|e| e.1)
        .min_by_key(|e| e.0)
        .unwrap();

    //let path = DijkstraMap::<Data, (_, _)>::extract_path(&start.1, &cost.1, &costs);
    //println!("{:?}", path);

    println!("{:?}", cost.0);

    // all best path Dijkstra

    let mut dmap = DijkstraMap::new(&data, DijkstraConfig { print_1000: true });

    let start = (0, (data.start, Facing::E));
    let costs = dmap.run(start);

    let paths = DijkstraMap::<Data, Vec<_>>::extract_all_paths(&start.1, &cost.1, &costs);

    let mut set = HashSet::new();

    for path in paths {
        for (pos, _) in path {
            set.insert(pos);
        }
    }

    println!("{}", set.len() + 1);

    Ok(())
}
