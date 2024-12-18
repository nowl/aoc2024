use anyhow::Error;
use aoc2024::{
    dijkstra::{DijkstraConfig, DijkstraInput, DijkstraMap},
    dp, Args,
};
use bytes::complete::tag;
use character::complete::{digit1, multispace0};
use clap::Parser;
use combinator::map_res;
use debug_print::debug_println;
use multi::many1;
use nom::*;
use sequence::{separated_pair, terminated};
use std::{collections::HashMap, fs, path::Path};

const TEST_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

#[derive(Debug)]
struct Data {
    data: Vec<(i32, i32)>,
    width: usize,
    height: usize,
}

fn parse_data(i: &str, width: usize, height: usize) -> IResult<&str, Data> {
    let parse_val = || map_res(digit1, str::parse);

    let parse_xy = separated_pair(parse_val(), tag(","), parse_val());
    let mut parse_problem = many1(terminated(parse_xy, multispace0));
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let data = Data {
        data: problems,
        width,
        height,
    };
    Ok((i, data))
}

fn read_data() -> Result<Data, Error> {
    let args = Args::parse();

    let mut width = 7;
    let mut height = 7;

    let contents = args.file.map_or(Ok(TEST_INPUT.to_string()), |input| {
        width = 71;
        height = 71;
        let file = Path::new(&input);
        fs::read_to_string(file)
    })?;

    let data = parse_data(&contents, width, height);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0.is_empty());
    Ok(data.1)
}

#[derive(Debug)]
struct Map {
    data: HashMap<(i32, i32), bool>,
}

impl DijkstraInput for Map {
    type Cost = i32;

    type Index = (i32, i32);

    fn get_adjacent(&self, (c, r): &Self::Index) -> Vec<(Self::Cost, Self::Index)> {
        let mut v = vec![];

        for (dc, dr) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let pos = (c + dc, r + dr);
            if let Some(true) = self.data.get(&pos) {
                v.push((1, pos));
            }
        }

        v
    }
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let mut map = HashMap::new();

    for r in 0..data.height {
        for c in 0..data.width {
            map.insert((c as i32, r as i32), true);
        }
    }

    // changes for example
    let num_bytes_to_apply = if data.width == 7 {
        12
    } else if data.width == 71 {
        1024
    } else {
        unreachable!()
    };

    data.data
        .iter()
        .take(num_bytes_to_apply)
        .for_each(|&(c, r)| {
            map.insert((c, r), false);
        });

    dp!(map);

    let map = Map { data: map };

    let start = (0, 0);
    let end = (data.width as i32 - 1, data.height as i32 - 1);

    let mut dmap = DijkstraMap::new(&map, DijkstraConfig::default());

    let costs = dmap.run((0, start.clone()));

    let path = DijkstraMap::<Map, (i32, i32)>::extract_path(&start, &end, &costs);

    dp!(path);

    println!("{}", path.len());

    Ok(())
}
