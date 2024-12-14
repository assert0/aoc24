use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl FromStr for Robot {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"p=(\d+),(\d+)\sv=(\-?\d+),(\-?\d+)").unwrap();
        let caps = re.captures(&s).unwrap();
        Ok(Self {
            position: (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
            velocity: (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
        })
    }
}

impl Robot {
    fn step(&mut self, size: (i32, i32)) {
        self.position.0 = (self.position.0 + self.velocity.0 + size.0) % size.0;
        self.position.1 = (self.position.1 + self.velocity.1 + size.1) % size.1;
    }
}

pub fn day14(args: &[String]) {
    println!("Day 14");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let robots: Vec<Robot> = contents.lines().map(|r| r.parse().unwrap()).collect();

    let is_example = robots.len() == 12;
    let size = if is_example { (11, 7) } else { (101, 103) };

    println!("Part 1: {}", part1(robots.clone(), size));

    // only try part2 with the real data
    if !is_example {
        println!("Part 2: {}", part2(robots.clone(), size).unwrap());
    }
}

fn part1(mut robots: Vec<Robot>, size: (i32, i32)) -> usize {
    for _ in 0..100 {
        for r in &mut robots {
            r.step(size);
        }
    }
    let mut quads = vec![vec![0; 2]; 2];
    let divide = ((size.0 / 2) as i32, (size.1 / 2) as i32);
    for r in &mut robots {
        if r.position.0 != divide.0 && r.position.1 != divide.1 {
            let (qx, qy) = (if r.position.0 < divide.0 { 0 } else { 1 },
                if r.position.1 < divide.1 { 0 } else { 1 });
            quads[qx][qy] += 1
        }
    }
    quads.iter().flatten().product()
}

fn part2(mut robots: Vec<Robot>, size: (i32, i32)) -> Option<usize> {
    for s in 1..10_000 {
        for r in &mut robots {
            r.step(size);
        }
        // NOTE: a pattern appears at this interval
        // if s % 101 == 68 {
        //     println!("Step: {}", s);
        //     print_bathroom(&robots, size);
        // }
        if has_long_line(&robots, size) {
            print_bathroom(&robots, size);
            return Some(s);
        }
    }
    None
}

fn get_counts(robots: &Vec<Robot>) -> HashMap<(i32, i32), i32> {
    let mut counts: HashMap<(i32, i32), i32> = HashMap::new();
    for r in robots.clone() {
        let count = counts.entry(r.position).or_insert(0);
        *count += 1;
    }
    counts
}

fn has_long_line(robots: &Vec<Robot>, size: (i32, i32)) -> bool {
    let counts = get_counts(&robots);
    for pos in counts.keys() {
        let mut run = 0;
        let mut x = pos.0;
        loop {
            if x < size.0 && counts.contains_key(&(x, pos.1)) {
                run += 1;
                if run > 30 {
                    return true;
                }
                x += 1;
            } else {
                break;
            }
        }
    }
    false
}

fn print_bathroom(robots: &Vec<Robot>, size: (i32, i32)) {
    let counts = get_counts(&robots);
    let mut output = Vec::new();
    for y in 0..size.1 {
        for x in 0..size.0 {
            if counts.contains_key(&(x, y)) {
                output.push("X".to_string());
            } else {
                output.push(".".to_string());
            }
        }
        output.push("\n".to_string());
    }
    println!("{}", output.join(""));
}
