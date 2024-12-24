use anyhow::Error;
use aoc2024::{dp, Args};
use bytes::complete::tag;
use character::complete::{alpha1, multispace0};
use clap::Parser;
use combinator::map_res;
use debug_print::debug_println;
use itertools::Itertools;
use multi::many1;
use nom::*;
use sequence::{separated_pair, terminated};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

const TEST_INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

#[derive(Debug)]
struct Data {
    data: Vec<(String, String)>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_line = separated_pair(
        map_res(alpha1, |v: &str| {
            Result::<String, nom::Err<()>>::Ok(v.to_string())
        }),
        tag("-"),
        map_res(alpha1, |v: &str| {
            Result::<String, nom::Err<()>>::Ok(v.to_string())
        }),
    );
    let mut parse_problem = many1(terminated(parse_line, multispace0));
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let data = Data { data: problems };
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

fn build_connections(data: &Vec<(String, String)>) -> HashMap<String, HashSet<String>> {
    let mut hm = HashMap::new();
    data.into_iter().for_each(|(c1, c2)| {
        hm.entry(c1.clone())
            .or_insert(HashSet::new())
            .insert(c2.clone());
        hm.entry(c2.clone())
            .or_insert(HashSet::new())
            .insert(c1.clone());
    });
    hm
}

fn path_exists_aux(
    prev: Vec<&str>,
    s: &str,
    e: &str,
    data: &HashMap<String, HashSet<String>>,
) -> bool {
    if prev.contains(&s) {
        return false;
    }
    let set = data.get(s).unwrap();
    if set.contains(e) {
        return true;
    }

    for i in set.iter() {
        let mut p = prev.clone();
        p.push(i);
        if path_exists_aux(p, i, e, data) {
            return true;
        }
    }

    false
}

fn path_exists(s: &str, e: &str, data: &HashMap<String, HashSet<String>>) -> bool {
    path_exists_aux(vec![], s, e, data)
}

fn build_networks(data: &HashMap<String, HashSet<String>>) -> Vec<HashSet<&String>> {
    let mut networks: Vec<HashSet<&String>> = vec![];
    for computer in data.keys() {
        dp!(computer);

        // check for networks to join
        let joins = networks
            .iter()
            .enumerate()
            .filter(|&(_, net)| net.iter().all(|c| path_exists(c, computer, data)))
            .map(|(n, _)| n)
            .collect_vec();

        // add to connections
        let mut to_copy = vec![];
        for j in joins.iter() {
            to_copy.push(networks[*j].clone());
            networks[*j].insert(computer);
        }

        for copy in to_copy {
            networks.push(copy);
        }

        // insert single entry
        let mut hs = HashSet::new();
        hs.insert(computer);
        networks.push(hs);
    }

    networks
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let connections = build_connections(&data.data);

    dp!(connections);

    let networks = build_networks(&connections);

    let longest_net: Option<&HashSet<&String>> = networks.iter().fold(None, |acc, n| {
        if acc.is_none_or(|o| o.len() < n.len()) {
            Some(n)
        } else {
            acc
        }
    });

    let longest_net = longest_net.unwrap();
    let answer = longest_net.iter().sorted().join(",");

    println!("{}", answer);

    Ok(())
}
