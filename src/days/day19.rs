use std::collections::{BinaryHeap, HashMap};
use std::fs;

pub fn day19(args: &[String]) {
    println!("Day 19");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut sections = contents.split("\n\n");

    let patterns: Vec<_> = sections
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    let designs: Vec<_> = sections
        .next()
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    // println!("{:?}", patterns);
    // println!("{:?}", designs);

    let part2: usize = designs
        .iter()
        .filter(|d| towel_patterns1(&patterns, d.to_string()))
        .count();
    println!("Part 1: {:?}", part2);

    let part1: usize = designs
        .iter()
        .map(|d| towel_patterns2(&mut HashMap::new(), &patterns, d.to_string()))
        .sum();
    println!("Part 2: {:?}", part1);
}

fn towel_patterns1(patterns: &Vec<String>, design: String) -> bool {
    let mut designs = BinaryHeap::new();
    designs.push((-(design.len() as i32), design.clone()));

    let mut solutions: HashMap<String, bool> = HashMap::new();
    while let Some((l, d)) = designs.pop() {
        if solutions.contains_key(&d) {
            continue;
        }
        // println!("{} {}", l, d);
        if l == 0 {
            return true;
        }
        let mut found = false;
        for p in patterns {
            if d.starts_with(p) {
                let next_d = d[p.len()..].to_string();
                designs.push((-(next_d.len() as i32), next_d.clone()));
                found = true;
            }
        }
        solutions.insert(d, found);
    }
    false
}

fn towel_patterns2(
    solutions: &mut HashMap<String, usize>,
    patterns: &Vec<String>,
    design: String,
) -> usize {
    // println!("{}", design);
    if solutions.contains_key(&design) {
        return *solutions.get(&design).unwrap();
    }
    if design.len() == 0 {
        return 1;
    }
    let mut sum = 0;
    for p in patterns {
        if design.starts_with(p) {
            let next_d = design[p.len()..].to_string();
            sum += towel_patterns2(solutions, &patterns, next_d);
        }
    }
    solutions.insert(design, sum);
    sum
}
