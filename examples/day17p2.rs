use anyhow::Error;
use aoc2024::{dp, Args};
use bytes::complete::tag;
use character::complete::{digit1, multispace0};
use clap::Parser;
use combinator::map_res;
use debug_print::debug_println;
use multi::separated_list1;
use nom::*;
use sequence::{preceded, terminated, tuple};
use std::{fs, path::Path};

const TEST_INPUT: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

#[derive(Debug)]
struct Data {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
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
    a: i64,
    b: i64,
    c: i64,
    ip: usize,
}

impl Machine {
    fn combo(&self, oprand: u8) -> i64 {
        match oprand {
            0 | 1 | 2 | 3 => oprand as i64,
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
                let num = self.a;
                let den = 2i64.pow(self.combo(oprand) as u32);
                self.a = num / den;
            }
            1 => {
                self.b = self.b ^ oprand as i64;
            }
            2 => {
                let combo = self.combo(oprand) % 8;
                self.b = combo;
            }
            3 => {
                if self.a != 0 {
                    self.ip = oprand as usize;
                }
            }
            4 => {
                self.b = self.b ^ self.c;
            }
            5 => {
                let combo = self.combo(oprand) % 8;
                out.push(combo as u8);
            }
            6 => {
                let num = self.a;
                let den = 2i64.pow(self.combo(oprand) as u32);
                self.b = num / den;
            }
            7 => {
                let num = self.a;
                let den = 2i64.pow(self.combo(oprand) as u32);
                self.c = num / den;
            }
            _ => unreachable!(),
        }

        // halt when ip >= program
        self.ip >= prog.len()
    }
}

fn run_trial(start: i64, end: i64, data: &Data) -> (i64, bool) {
    let program = &data.program;

    let mut a = start;
    loop {
        let mut machine = Machine {
            a,
            b: data.reg_b,
            c: data.reg_c,
            ip: 0,
        };
        let mut output = vec![];

        let mut halt_state = false;
        while !halt_state {
            halt_state = machine.step(program, &mut output);
        }

        debug_println!("{a}: {:?}", data.program);
        debug_println!("{a}: {output:?}");

        // check if the ends match during this iteration
        if output
            .iter()
            .rev()
            .zip(program.iter().rev())
            .all(|(a, b)| a == b)
        {
            return (a, output.len() == data.program.len());
        }

        //read_line();

        a += 1;
    }
}

fn main() -> Result<(), Error> {
    let data = read_data()?;

    let mut start = 0;
    loop {
        dp!(start);
        let (result, complete) = run_trial(start, 8, &data);
        start = result << 3;

        debug_println!("current result: {result}");
        debug_println!("next start: {start:b}");

        //read_line()?;

        if complete {
            println!("{result}");
            break;
        }
    }

    Ok(())
}
