use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

pub fn day5(args: &[String]) {
    println!("Day 5");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut sections = contents.split("\n\n");
    let rules: Vec<(_, _)> = sections
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.split("|")
                .map(|v| v.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    let updates: Vec<Vec<_>> = sections
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split(",").map(|v| v.parse::<i32>().unwrap()).collect())
        .collect();

    let mut part1 = 0;
    updates.iter().for_each(|u| {
        if is_valid(&rules, u) {
            part1 += u[u.len() / 2];
        }
    });
    println!("Part 1: {}", part1);

    let mut part2 = 0;
    updates.iter().for_each(|u| {
        let mut u2 = u.clone();
        u2.sort_by(|a, b| compare(&rules, *a, *b));
        assert!(is_valid(&rules, &u2));
        part2 += u2[u2.len() / 2];
    });
    println!("Part 2: {}", part2 - part1);
}

pub fn is_valid(rules: &Vec<(i32, i32)>, update: &Vec<i32>) -> bool {
    (0..update.len()).all(|i| check_right(&rules, &update, i))
}

pub fn check_right(rules: &Vec<(i32, i32)>, update: &Vec<i32>, pos: usize) -> bool {
    let right_updates: HashSet<i32> = update[pos + 1..].iter().cloned().collect();
    let right_rules = match_left(&rules, update[pos]);
    right_updates.is_subset(&right_rules)
}

pub fn match_left(rules: &Vec<(i32, i32)>, rule: i32) -> HashSet<i32> {
    rules.iter().filter(|r| r.0 == rule).map(|r| r.1).collect()
}

pub fn compare(rules: &Vec<(i32, i32)>, a: i32, b: i32) -> Ordering {
    for r in rules {
        if r.0 == a && r.1 == b {
            return Ordering::Less;
        } else if r.0 == b && r.1 == a {
            return Ordering::Greater;
        }
    }
    unreachable!("No rule?");
}
