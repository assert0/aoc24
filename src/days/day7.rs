use itertools::Itertools;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Calibration {
    total: usize,
    values: Vec<usize>,
}

impl FromStr for Calibration {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split(": ").collect_tuple().unwrap();
        Ok(Self {
            total: l.parse().unwrap(),
            values: r
                .split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .collect(),
        })
    }
}

impl Calibration {
    fn eval(&self, ops: &Vec<char>) -> usize {
        ops.iter()
            .zip(self.values[1..].iter())
            .fold(self.values[0], |acc, (o, v)| match o {
                '+' => acc + v,
                '*' => acc * v,
                '|' => format!("{}{}", acc, v).parse().unwrap(),
                _ => unreachable!(),
            })
    }
}

pub fn day7(args: &[String]) {
    println!("Day 7");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let calibrations: Vec<Calibration> = contents.lines().map(|l| l.parse().unwrap()).collect();

    println!("Part 1: {}", solve(&calibrations, vec!['+', '*']));
    println!("Part 2: {}", solve(&calibrations, vec!['+', '*', '|']));
}

fn solve(calibrations: &Vec<Calibration>, ops: Vec<char>) -> usize {
    let mut solution = 0;
    for c in calibrations {
        let c_ops = vec![ops.clone(); c.values.len() - 1];
        for o in c_ops.into_iter().multi_cartesian_product() {
            let total = c.eval(&o);
            if total == c.total {
                solution += total;
                break;
            }
        }
    }
    solution
}
