use itertools::{iproduct, Itertools};
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Debug)]
struct TopoMap {
    map: Vec<Vec<u32>>,
}

impl FromStr for TopoMap {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            map: s
                .lines()
                .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect(),
        })
    }
}

impl TopoMap {
    fn size(&self) -> (usize, usize) {
        (self.map.len(), self.map[0].len())
    }

    fn trailheads(&self) -> Vec<(isize, isize)> {
        let (my, mx) = self.size();
        iproduct!(0..my, 0..mx)
            .filter(|(y, x)| self.map[*y][*x] == 0)
            .map(|(y, x)| (y as isize, x as isize))
            .collect()
    }

    fn in_map(&self, pos: (isize, isize)) -> bool {
        (0..self.map.len() as isize).contains(&pos.0)
            && (0..self.map[0].len() as isize).contains(&pos.1)
    }

    fn level(&self, pos: (isize, isize)) -> Option<u32> {
        if self.in_map(pos) {
            Some(self.map[pos.0 as usize][pos.1 as usize])
        } else {
            None
        }
    }
}

lazy_static! {
    static ref DIRS: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
}

pub fn day10(args: &[String]) {
    println!("Day 10");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let topomap: TopoMap = contents.parse().unwrap();
    let trailheads = topomap.trailheads();

    let part1: usize = trailheads
        .iter()
        .map(|t| solve_part1(&topomap, *t).len())
        .sum();
    println!("Part 1: {}", part1);

    let part2: usize = trailheads.iter().map(|t| solve_part2(&topomap, *t)).sum();
    println!("Part 2: {}", part2);
}

fn solve_part1(topomap: &TopoMap, pos: (isize, isize)) -> Vec<(isize, isize)> {
    let level = topomap.level(pos).unwrap();
    if level == 9 {
        vec![pos]
    } else {
        DIRS.iter()
            .map(|(dy, dx)| (pos.0 + dy, pos.1 + dx))
            .filter(|(y, x)| topomap.level((*y, *x)) == Some(level + 1))
            .map(|pos2| solve_part1(&topomap, pos2))
            .flatten()
            .unique()
            .collect()
    }
}

fn solve_part2(topomap: &TopoMap, pos: (isize, isize)) -> usize {
    let level = topomap.level(pos).unwrap();
    if level == 9 {
        return 1;
    };
    DIRS.iter()
        .map(|(dy, dx)| (pos.0 + dy, pos.1 + dx))
        .filter(|(y, x)| topomap.level((*y, *x)) == Some(level + 1))
        .map(|pos2| solve_part2(&topomap, pos2))
        .sum()
}
