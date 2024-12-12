use std::collections::HashMap;
use std::fs;

pub fn day11(args: &[String]) {
    println!("Day 11");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut stones: Vec<usize> = contents
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    for _ in 1..=25 {
        stones = blink(stones);
    }
    println!("Part 1: {}", stones.len());

    let mut stones_counts: HashMap<usize, usize> = HashMap::new();
    contents
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .for_each(|v| {
            let count = stones_counts.entry(v).or_insert(0);
            *count += 1;
        });
    for _ in 1..=75 {
        stones_counts = blink2(stones_counts);
    }
    println!("Part 2: {}", stones_counts.values().sum::<usize>());
}

fn next_stones(v: usize) -> Vec<usize> {
    if v == 0 {
        vec![1]
    } else {
        let l = ((v + 1) as f64).log10().ceil() as u32;
        if l % 2 == 0 {
            let s = 10_usize.pow(l / 2);
            vec![v / s, v % s]
        } else {
            vec![v * 2024]
        }
    }
}

fn blink(stones: Vec<usize>) -> Vec<usize> {
    stones
        .into_iter()
        .map(|v| next_stones(v))
        .flatten()
        .collect()
}

fn blink2(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut stones2: HashMap<usize, usize> = HashMap::new();
    for (num, count) in stones {
        next_stones(num).iter().for_each(|v| {
            let count2 = stones2.entry(*v).or_insert(0);
            *count2 += count;
        });
    }
    stones2
}
