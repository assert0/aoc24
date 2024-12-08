use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn day8(args: &[String]) {
    println!("Day 8");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let map: Vec<Vec<_>> = contents.lines().map(|l| l.chars().collect()).collect();
    let mapsize = (map.len(), map[0].len());

    let mut freq_antennas: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    iproduct!(0..mapsize.0, 0..mapsize.1)
        .filter(|(y, x)| map[*y][*x] != '.')
        .for_each(|(y, x)| {
            let freq = freq_antennas.entry(map[y][x]).or_insert(HashSet::new());
            freq.insert((y, x));
        });

    println!("Part 1: {}", solve(&freq_antennas, mapsize, true));
    println!("Part 2: {}", solve(&freq_antennas, mapsize, false));
}

fn in_map(mapsize: (usize, usize), pos: (isize, isize)) -> bool {
    (0..mapsize.0 as isize).contains(&pos.0) && (0..mapsize.1 as isize).contains(&pos.1)
}

fn solve(
    freq_antennas: &HashMap<char, HashSet<(usize, usize)>>,
    mapsize: (usize, usize),
    part1: bool,
) -> usize {
    let mut antinodes = HashSet::new();
    let partrange = if part1 { 1..2 } else { 0..isize::MAX };
    for (_freq, antennas) in freq_antennas {
        for ant in antennas.iter().combinations(2) {
            let (dy, dx) = (
                ant[0].0 as isize - ant[1].0 as isize,
                ant[0].1 as isize - ant[1].1 as isize,
            );
            for i in partrange.clone() {
                let a0 = (ant[0].0 as isize + i * dy, ant[0].1 as isize + i * dx);
                if in_map(mapsize, a0) {
                    antinodes.insert(a0);
                } else {
                    break;
                }
            }
            for i in partrange.clone() {
                let a1 = (ant[1].0 as isize - i * dy, ant[1].1 as isize - i * dx);
                if in_map(mapsize, a1) {
                    antinodes.insert(a1);
                } else {
                    break;
                }
            }
        }
    }
    antinodes.len()
}
