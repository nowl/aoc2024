use std::{fs, path::Path};

use anyhow::Error;
use aoc2024::{dp, Args};
use clap::Parser;
use debug_print::debug_println;
use nom::{branch::*, character::complete::*, combinator::*, multi::*, sequence::*, *};

const TEST_INPUT: &str = "2333133121414131402";

#[derive(Debug)]
struct Data {
    data: Vec<u32>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_line = map_res(anychar, |c| c.to_digit(10).ok_or("bad parse"));
    let mut parse_problem = terminated(many1(parse_line), alt((eof, multispace1)));
    let (i, data) = parse_problem(i)?;

    let data = Data { data };
    Ok((i, data))
}

#[derive(Debug, PartialEq, Eq)]
enum BlockType {
    File(usize, u32),
    Empty(u32),
}

#[derive(Debug)]
struct BlockVector {
    data: Vec<BlockType>,
}

impl From<Data> for BlockVector {
    fn from(value: Data) -> Self {
        let data = value
            .data
            .chunks(2)
            .enumerate()
            .flat_map(|(n, v)| {
                use BlockType::*;
                let mut vec = vec![];
                vec.push(File(n, v[0]));
                if v.len() == 2 {
                    vec.push(Empty(v[1]));
                }
                vec
            })
            .filter(|v| *v != BlockType::Empty(0))
            .collect();
        Self { data }
    }
}

impl BlockVector {
    fn find_last_file(&self) -> Option<usize> {
        self.data.iter().enumerate().rev().find_map(|(n, x)| {
            if matches!(*x, BlockType::File(..)) {
                Some(n)
            } else {
                None
            }
        })
    }

    fn find_first_gap(&self) -> Option<usize> {
        self.data
            .iter()
            .position(|x| matches!(*x, BlockType::Empty(..)))
    }

    fn fill_gap_from_file(&mut self, gidx: usize, fidx: usize) {
        let gap = self.data.get(gidx).unwrap();
        let BlockType::Empty(gap_size) = *gap else {
            unreachable!()
        };

        let file = self.data.get(fidx).unwrap();
        let BlockType::File(file_num, file_size) = *file else {
            unreachable!()
        };

        if gap_size > file_size {
            let BlockType::Empty(new_gap_size) = self.data.get_mut(gidx).unwrap() else {
                unreachable!()
            };
            *new_gap_size = gap_size - file_size;

            let file = self.data.remove(fidx);
            self.data.insert(gidx, file);
        } else if gap_size == file_size {
            self.data.swap(gidx, fidx);
            self.data.swap_remove(fidx);
        } else {
            // file_size > gap_size
            let BlockType::File(_, new_file_size) = self.data.get_mut(fidx).unwrap() else {
                unreachable!()
            };
            *new_file_size = file_size - gap_size;

            let old_gap = self.data.get_mut(gidx).unwrap();
            *old_gap = BlockType::File(file_num, gap_size);
        }
    }

    fn checksum(&self) -> u64 {
        let fold = self
            .data
            .iter()
            .fold((0, 0), |(acc, n), block| match block {
                &BlockType::File(file_num, size) => {
                    let mut new_acc = acc;
                    for s in n..n + size {
                        new_acc += s as u64 * file_num as u64;
                    }
                    (new_acc, n + size)
                }
                BlockType::Empty(_) => (acc, n),
            });

        fold.0
    }
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

fn main() -> Result<(), Error> {
    let data = read_data()?;
    dp!(data);

    let mut blockdata: BlockVector = data.into();
    dp!(blockdata);

    loop {
        let file_idx = blockdata.find_last_file();
        let gap_idx = blockdata.find_first_gap();

        let Some(fidx) = file_idx else { break };
        let Some(gidx) = gap_idx else { break };

        if gidx > fidx {
            break;
        };

        blockdata.fill_gap_from_file(gidx, fidx);
    }

    dp!(blockdata);

    println!("{}", blockdata.checksum());

    Ok(())
}
