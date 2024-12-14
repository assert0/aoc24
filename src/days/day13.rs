use itertools::iproduct;
use lstsq;
use nalgebra::{self as na, OMatrix, OVector, U2};
use regex::Regex;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Button {
    label: char,
    x: usize,
    y: usize,
}

impl FromStr for Button {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Button ([A-Z]): X\+(\d+), Y\+(\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        Ok(Self {
            label: caps[1].chars().next().unwrap(),
            x: caps[2].parse().unwrap(),
            y: caps[3].parse().unwrap(),
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Prize {
    x: usize,
    y: usize,
}

impl FromStr for Prize {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        Ok(Self {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Claw {
    a: Button,
    b: Button,
    prize: Prize,
}

impl FromStr for Claw {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        Ok(Self {
            a: lines.next().unwrap().parse().unwrap(),
            b: lines.next().unwrap().parse().unwrap(),
            prize: lines.next().unwrap().parse().unwrap(),
        })
    }
}

impl Claw {
    fn hit_prize(&self, presses: (usize, usize)) -> bool {
        let (pa, pb) = presses;
        self.a.x * pa + self.b.x * pb == self.prize.x
            && self.a.y * pa + self.b.y * pb == self.prize.y
    }

    fn part1_tokens(&self) -> Option<usize> {
        iproduct!(0..100, 0..100)
            .filter(|p| self.hit_prize(*p))
            .map(|p| tokens(p))
            .min()
    }

    fn part2_tokens(&self) -> Option<usize> {
        let a = OMatrix::<f64, na::Dyn, U2>::from_row_slice(&[
            self.a.x as f64,
            self.b.x as f64,
            self.a.y as f64,
            self.b.y as f64,
        ]);
        let c = 10000000000000_f64;
        let b = OVector::<f64, na::Dyn>::from_row_slice(&[
            self.prize.x as f64 + c,
            self.prize.y as f64 + c,
        ]);

        let epsilon = 1e-10;
        let results = lstsq::lstsq(&a, &b, epsilon).unwrap();
        if results.solution.nrows() == 2 && results.solution.min() > 0.0 {
            let (a, b) = (results.solution[0], results.solution[1]);
            // valid solutions should be basically be integer values, if they are not
            // then it is not a valid solution.
            if (a.fract() < 0.001 || a.fract() > 0.999) && (b.fract() < 0.001 || b.fract() > 0.999)
            {
                return Some(tokens((a.round() as usize, b.round() as usize)));
            }
        }
        None
    }
}

pub fn tokens(presses: (usize, usize)) -> usize {
    presses.0 * 3 + presses.1
}

pub fn day13(args: &[String]) {
    println!("Day 13");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let sections = contents.split("\n\n");
    let claws: Vec<Claw> = sections.map(|s| s.parse().unwrap()).collect();

    let part1: usize = claws.iter().filter_map(|c| c.part1_tokens()).sum();
    println!("Part 1: {}", part1);

    let part2: usize = claws.iter().filter_map(|c| c.part2_tokens()).sum();
    println!("Part 2: {}", part2);
}
