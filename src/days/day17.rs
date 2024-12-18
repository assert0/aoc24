use itertools::Itertools;
use regex::Regex;
use std::{fs, num::ParseIntError, str::FromStr, usize};

#[derive(Clone, Eq, PartialEq, Debug)]
struct Computer {
    registers: Vec<usize>,
    ip: usize,
    program: Vec<usize>,
    output: Vec<usize>,
}

impl FromStr for Computer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: ([\d\,]+)",
        )
        .unwrap();
        let caps = re.captures(s).unwrap();
        let a = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let b = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let c = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let p = caps
            .get(4)
            .unwrap()
            .as_str()
            .split(",")
            .map(|c| c.parse::<usize>().unwrap())
            .collect();

        Ok(Self {
            registers: vec![a, b, c],
            ip: 0,
            program: p,
            output: vec![],
        })
    }
}

impl Computer {
    fn step(&mut self) -> bool {
        if self.ip >= self.program.len() {
            return false;
        }
        let opcode = self.program[self.ip];
        let operand = self.program[self.ip + 1];
        self.ip += 2;
        // println!("{} {}", opcode, operand);
        // println!("{:?}", self);
        match opcode {
            0 => {
                self.registers[0] /= 2_usize.pow(self.combo_operand(operand) as u32);
            }
            1 => {
                self.registers[1] ^= operand;
            }
            2 => {
                self.registers[1] = self.combo_operand(operand) % 8;
            }
            3 => {
                if self.registers[0] != 0 {
                    self.ip = operand;
                }
            }
            4 => {
                self.registers[1] ^= self.registers[2];
            }
            5 => {
                self.output.push(self.combo_operand(operand) % 8);
            }
            6 => {
                self.registers[1] =
                    self.registers[0] / 2_usize.pow(self.combo_operand(operand) as u32);
            }
            7 => {
                self.registers[2] =
                    self.registers[0] / 2_usize.pow(self.combo_operand(operand) as u32);
            }
            _ => {
                unreachable!("Unknown opcode: {}", opcode);
            }
        };
        true
    }

    fn combo_operand(&self, value: usize) -> usize {
        match value {
            0 | 1 | 2 | 3 => value,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => unreachable!("Bad operand"),
        }
    }
}

pub fn day17(args: &[String]) {
    println!("Day 17");
    if args.len() != 1 {
        println!("Missing input file.");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut cpu: Computer = contents.parse().unwrap();

    loop {
        if !cpu.step() {
            println!("Part 1: {}", cpu.output.iter().join(","));
            break;
        }
    }

    let cpu: Computer = contents.parse().unwrap();
    // run until the bottom 24 bits of register A are known, then
    // use that as a the base to start counting from.
    let mut i = 0;
    let mut best = 0;
    let mut lowerbits = 0;
    let mut lowerbitcount = 0;
    loop {
        let a = i << lowerbitcount | lowerbits;
        let mut cpu2 = cpu.clone();
        cpu2.registers[0] = a;
        loop {
            if !cpu2.step() {
                break;
            }
            if cpu2.output != cpu2.program[0..cpu2.output.len()] {
                break;
            } else {
                if cpu2.output.len() > best {
                    best = cpu2.output.len();
                    // println!("{:#020x} {} -> {:?}", a, a, cpu2.output);
                    if (a >> 24) > 0 {
                        lowerbits = a & 0xFFFFFF;
                        lowerbitcount = 24;
                        i = 0;
                        break;
                    }
                }
            }
        }
        if cpu2.output == cpu2.program {
            println!("Part 2: {}", a);
            break;
        }
        i += 1;
    }
}
