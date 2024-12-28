use itertools::iproduct;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TumblerType {
    Lock,
    Key,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct LockKey {
    tumbler: TumblerType,
    pins: Vec<usize>,
}

impl FromStr for LockKey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        let t = match m[0][0] {
            '.' => TumblerType::Lock,
            '#' => TumblerType::Key,
            _ => unreachable!(),
        };
        let p: Vec<_> = (0..m[0].len())
            .map(|x| (0..m.len()).filter(|y| m[*y][x] == '#').count() - 1)
            .collect();
        Ok(Self {
            tumbler: t,
            pins: p,
        })
    }
}

pub fn day25(args: &[String]) {
    println!("Day 25");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let sections = contents.split("\n\n");
    let lk: Vec<LockKey> = sections.map(|s| s.parse().unwrap()).collect();

    let locks: Vec<_> = filter_type(lk.clone(), TumblerType::Lock);
    let keys: Vec<_> = filter_type(lk.clone(), TumblerType::Key);

    let part1 = iproduct!(locks, keys)
        .filter(|(l, k)| (0..l.pins.len()).all(|i| l.pins[i] + k.pins[i] <= 5))
        .count();
    println!("Part 1: {}", part1);
}

fn filter_type(lk: Vec<LockKey>, t: TumblerType) -> Vec<LockKey> {
    lk.into_iter()
        .filter(|l| l.tumbler == t)
        .collect()
}
