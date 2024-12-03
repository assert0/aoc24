use regex::Regex;
use std::fs;

pub fn day3(args: &[String]) {
    println!("Day 3");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let re = Regex::new(r"mul\(([0-9]{1,3})\,([0-9]{1,3})\)").unwrap();
    let part1: i32 = re
        .captures_iter(&contents)
        .map(|caps| {
            let l = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let r = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            l * r
        })
        .sum();
    println!("Part 1: {}", part1);

    let re = Regex::new(r"(mul|do|don't)\(([0-9]{1,3}\,[0-9]{1,3})?\)").unwrap();
    let parts: Vec<_> = re
        .captures_iter(&contents)
        .map(|caps| {
            let op = caps.get(1).unwrap().as_str();
            let result = match caps.get(2) {
                None => 0,
                Some(m) => m
                    .as_str()
                    .split(',')
                    .map(|v| v.parse::<i32>().unwrap())
                    .product(),
            };
            (op, result)
        })
        .collect();
    let mut domult = true;
    let mut part2 = 0;
    parts.into_iter().for_each(|(op, result)| match op {
        "do" => domult = true,
        "don't" => domult = false,
        "mul" => {
            if domult {
                part2 += result
            }
        }
        _ => unreachable!(),
    });
    println!("Part 2: {:?}", part2);
}
