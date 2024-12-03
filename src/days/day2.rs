use std::fs;

pub fn day2(args: &[String]) {
    println!("Day 2");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let reports: Vec<_> = contents
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let part1 = reports.iter().map(|r| is_safe(r)).filter(|&v| v).count();
    println!("Part 1: {}", part1);

    let part2 = reports
        .iter()
        .map(|r| is_safe_without_one(r))
        .filter(|&v| v)
        .count();
    println!("Part 2: {}", part2);
}

pub fn deltas(report: &Vec<i32>) -> Vec<i32> {
    report.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>()
}

pub fn is_safe(report: &Vec<i32>) -> bool {
    let deltas = deltas(report);
    deltas.iter().all(|&v| (v > 0 && v < 4)) || deltas.iter().all(|&v| (v > -4 && v < 0))
}

pub fn is_safe_without_one(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut report2 = report.clone();
        report2.remove(i);
        if is_safe(&report2) {
            return true;
        }
    }
    false
}
