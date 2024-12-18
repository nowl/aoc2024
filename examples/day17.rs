use anyhow::Error;
use aoc2024::{dp, read_line, Args};
use bytes::complete::tag;
use character::complete::{digit1, multispace0};
use clap::Parser;
use combinator::map_res;
use debug_print::debug_println;
use itertools::Itertools;
use multi::separated_list1;
use nom::*;
use sequence::{preceded, terminated, tuple};
use std::{fs, path::Path};

const TEST_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

#[derive(Debug)]
struct Data {
    reg_a: i32,
    reg_b: i32,
    reg_c: i32,
    program: Vec<u8>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_i32 = || map_res(preceded(multispace0, digit1), str::parse);
    let parse_u8 = map_res(preceded(multispace0, digit1), str::parse);
    let parse_a = preceded(tag("Register A:"), parse_i32());
    let parse_b = preceded(tag("Register B:"), parse_i32());
    let parse_c = preceded(tag("Register C:"), parse_i32());
    let parse_program = preceded(
        terminated(tag("Program:"), multispace0),
        separated_list1(tag(","), parse_u8),
    );
    let mst = |p| terminated(p, multispace0);
    let mstp = |p| terminated(p, multispace0);
    let mut parse_problem = tuple((
        mst(parse_a),
        mst(parse_b),
        mst(parse_c),
        mstp(parse_program),
    ));
    let (i, problems) = parse_problem(i)?;

    dp!(problems);

    let (reg_a, reg_b, reg_c, program) = problems;

    let data = Data {
        reg_a,
        reg_b,
        reg_c,
        program,
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

#[derive(Debug)]
struct Machine {
    a: i32,
    b: i32,
    c: i32,
    ip: usize,
}

impl Machine {
    fn combo(&self, oprand: u8) -> i32 {
        match oprand {
            0 | 1 | 2 | 3 => oprand as i32,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn step(&mut self, prog: &Vec<u8>, out: &mut Vec<u8>) -> bool {
        let opcode = prog[self.ip];
        let oprand = prog[self.ip + 1];
        self.ip += 2;

        match opcode {
            0 => {
                debug_println!("adv");
                let num = self.a;
                let den = 2i32.pow(self.combo(oprand) as u32);
                self.a = num / den;
            }
            1 => {
                debug_println!("bxl");
                self.b = self.b ^ oprand as i32;
            }
            2 => {
                debug_println!("bst");

                let combo = self.combo(oprand) % 8;
                self.b = combo;
            }
            3 => {
                debug_println!("jnz");

                if self.a != 0 {
                    self.ip = oprand as usize;
                }
            }
            4 => {
                debug_println!("--bxc");
                self.b = self.b ^ self.c;
            }
            5 => {
                debug_println!("out");
                let combo = self.combo(oprand) % 8;
                out.push(combo as u8);
            }
            6 => {
                debug_println!("--bdv");
                let num = self.a;
                let den = 2i32.pow(self.combo(oprand) as u32);
                self.b = num / den;
            }
            7 => {
                debug_println!("--cdv");
                let num = self.a;
                let den = 2i32.pow(self.combo(oprand) as u32);
                self.c = num / den;
            }
            _ => unreachable!(),
        }

        // halt when ip >= program
        self.ip >= prog.len()
    }
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    dp!(data);

    let program = data.program;
    let mut output = vec![];

    let mut machine = Machine {
        a: data.reg_a,
        b: data.reg_b,
        c: data.reg_c,
        ip: 0,
    };

    dp!(machine);

    let mut halt_state = false;
    while !halt_state {
        halt_state = machine.step(&program, &mut output);

        dp!(machine);
        dp!(output);

        //read_line()?;
    }

    let result = output.into_iter().join(",");

    println!("{result}");

    Ok(())
}
