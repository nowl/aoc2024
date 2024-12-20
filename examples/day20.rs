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
use std::{collections::HashMap, fs, path::Path};

const TEST_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    Wall,
    Empty,
}

#[derive(Debug, Clone)]
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

    type Index = (i32, i32);

    fn get_adjacent(&self, pos: &Self::Index) -> Vec<(Self::Cost, Self::Index)> {
        let mut v = vec![];

        macro_rules! adj_test {
            ($dr:expr, $dc:expr) => {{
                let pos = (pos.0 + $dr, pos.1 + $dc);
                if self.map.get(&pos).is_some_and(|t| *t == Tile::Empty) {
                    v.push((1, pos));
                }
            }};
        }

        adj_test!(-1, 0);
        adj_test!(1, 0);
        adj_test!(0, -1);
        adj_test!(0, 1);

        v
    }
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let mut dmap =
        DijkstraMap::<Data, (i32, i32)>::new(&data, DijkstraConfig { print_1000: false });
    let costs = dmap.run((0, data.start));

    let initial_end_cost = costs.get(&data.end).unwrap().0;

    dp!(initial_end_cost);

    // run cheats
    let mut count = 0;
    for cheat_loc in data.map.iter().filter_map(|(k, v)| match v {
        Tile::Empty => None,
        Tile::Wall => Some(k.clone()),
    }) {
        let mut map = data.clone();
        *map.map.get_mut(&cheat_loc).unwrap() = Tile::Empty;
        let mut dmap =
            DijkstraMap::<Data, (i32, i32)>::new(&map, DijkstraConfig { print_1000: false });
        let costs = dmap.run((0, data.start));

        let end_cost = costs.get(&data.end).unwrap().0;

        let cost_diff = initial_end_cost - end_cost;
        if cost_diff >= 100 {
            count += 1;
        }
    }
    println!("{}", count);

    Ok(())
}
