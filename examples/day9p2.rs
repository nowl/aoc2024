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
    fn find_last_file_num(&self) -> Option<usize> {
        self.data.iter().rev().find_map(|x| match *x {
            BlockType::File(file_num, _) => Some(file_num),
            BlockType::Empty(..) => None,
        })
    }

    fn find_file_num(&self, fnum: usize) -> Option<usize> {
        self.data.iter().position(|blk| match *blk {
            BlockType::File(file_num, _) => file_num == fnum,
            BlockType::Empty(..) => false,
        })
    }

    fn find_first_largest_gap_for(&self, fidx: usize) -> Option<usize> {
        let file = self.data.get(fidx).unwrap();
        let BlockType::File(_, file_size) = *file else {
            unreachable!()
        };

        self.data.iter().position(|blk| match *blk {
            BlockType::File(..) => false,
            BlockType::Empty(free_size) => free_size >= file_size,
        })
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

            self.data.insert(gidx, BlockType::File(file_num, file_size));

            let old_file = self.data.get_mut(fidx + 1).unwrap();
            *old_file = BlockType::Empty(file_size);
        } else if gap_size == file_size {
            self.data.swap(gidx, fidx);
        }
    }

    //fn compact_empty_blocks(self) -> Self {
    //    let compacted = self
    //        .data
    //        .into_iter()
    //        .fold(Vec::new(), |mut acc, blk| match blk {
    //            BlockType::File(..) => {
    //                acc.push(blk);
    //                acc
    //            }
    //            BlockType::Empty(blk_size) => {
    //                if let Some(last_blk) = acc.last_mut() {
    //                    if last_blk
    //                } else {
    //                    acc.push(blk);
    //                }
    //                acc
    //            }
    //        });

    //    Self { data: compacted }
    //}

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
                BlockType::Empty(size) => (acc, n + size),
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
    assert!(data.0.is_empty());
    Ok(data.1)
}

fn main() -> Result<(), Error> {
    let data = read_data()?;
    dp!(data);

    let mut blockdata: BlockVector = data.into();
    dp!(blockdata);

    let last_file_num = blockdata.find_last_file_num().unwrap();
    dp!(last_file_num);

    for n in (0..=last_file_num).rev() {
        //blockdata = blockdata.compact_empty_blocks();

        dp!(n);
        let fidx = blockdata.find_file_num(n).unwrap();
        let gap_idx = blockdata.find_first_largest_gap_for(fidx);

        let Some(gidx) = gap_idx else { continue };
        if gidx > fidx {
            continue;
        }

        blockdata.fill_gap_from_file(gidx, fidx);
        dp!(blockdata);

        //read_line()?;
    }

    dp!(blockdata);

    println!("{}", blockdata.checksum());

    Ok(())
}
