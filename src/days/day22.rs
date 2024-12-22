use std::{fs, num::ParseIntError, str::FromStr};

use itertools::{iproduct, Itertools};
use tqdm::tqdm;

#[derive(Clone, Eq, PartialEq, Debug, Copy)]
struct Secret {
    number: usize,
    limit: usize,
}

impl FromStr for Secret {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            number: s.parse().unwrap(),
            limit: 2000,
        })
    }
}

impl Iterator for Secret {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.limit == 0 {
            return None;
        }
        self.number = ((self.number * 0x40) ^ self.number) % 0x1000000;
        self.number = ((self.number / 32) ^ self.number) % 0x1000000;
        self.number = ((self.number * 0x800) ^ self.number) % 0x1000000;
        self.limit -= 1;
        Some(self.number)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Copy)]
struct SecretPriceChanges {
    secret: Secret,
}

impl Iterator for SecretPriceChanges {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let last = self.secret.number;
        let n = self.secret.next();
        if n.is_none() {
            return None;
        }
        Some((n.unwrap() % 10) as isize - (last % 10) as isize)
    }
}

pub fn day22(args: &[String]) {
    println!("Day 22");
    if args.len() != 1 {
        println!("Missing input file.");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let part1: usize = contents
        .lines()
        .map(|l| l.parse::<Secret>().unwrap())
        .map(|s| s.skip(2000 - 1).next().unwrap())
        .sum();
    println!("Part 1: {}", part1);

    // TODO: Rework to find a faster approarch because the following brute-force is
    // very slow (but does get the answer in around 30 minutes).
    let secrets: Vec<Secret> = contents.lines().map(|l| l.parse().unwrap()).collect();
    let mut best = 0;
    for c in tqdm(iproduct!(-9..=9, -9..=9, -9..=9, -9..=9)) {
        let b: usize = secrets.iter().filter_map(|s| get_price(*s, c)).sum();
        if b > best {
            best = b;
        }
    }
    println!("Part 2: {}", best);
}

fn get_price(secret: Secret, changes: (isize, isize, isize, isize)) -> Option<usize> {
    let s = SecretPriceChanges { secret: secret };
    let p = s
        .tuple_windows::<(_, _, _, _)>()
        .find_position(|t| *t == changes);
    if p.is_some() {
        Some(s.secret.skip(p.unwrap().0 + 3).next().unwrap() % 10)
    } else {
        None
    }
}
