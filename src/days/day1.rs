use itertools::Itertools;
use std::fs;
use std::iter::zip;

pub fn day1(args: &[String]) {
    println!("Day 1");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let (mut left, mut right): (Vec<_>, Vec<_>) = contents
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();
    left.sort();
    right.sort();

    let part1: i32 = zip(left.clone(), right.clone())
        .map(|(a, b)| (a - b).abs())
        .sum();
    println!("Part 1: {}", part1);

    let right_counts = right.iter().counts_by(|v| v);
    let part2: i32 = left
        .iter()
        .map(|l| *right_counts.get(l).unwrap_or(&0) as i32 * l)
        .sum();
    println!("Part 2: {}", part2);
}
