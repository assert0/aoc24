use itertools::iproduct;
use itertools::Itertools;
use rusttype::{point, Point};
use std::collections::{BinaryHeap, HashMap};
use std::{fs, usize, vec};

pub fn day20(args: &[String]) {
    println!("Day 20");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let map: Vec<Vec<_>> = contents.lines().map(|l| l.chars().collect()).collect();
    let path = shortest_path(&map, char_position(&map, 'S'), char_position(&map, 'E')).unwrap();

    let cheats = find_cheats(&path, 2);
    let saves = if map.len() == 15 { 1 } else { 100 };
    let part1: usize = cheats.iter().filter(|s| **s >= saves).count();
    println!("Part 1: {}", part1);

    let cheats = find_cheats(&path, 20);
    let saves = if map.len() == 15 { 50 } else { 100 };

    let part2: usize = cheats.iter().filter(|s| **s >= saves).count();
    println!("Part 2: {}", part2);
}

lazy_static! {
    static ref ADJ: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
}

pub fn adjacent(pos: (isize, isize)) -> Vec<(isize, isize)> {
    ADJ.iter()
        .map(|(dy, dx)| (pos.0 + dy, pos.1 + dx))
        .collect()
}

pub fn map_value(map: &Vec<Vec<char>>, p: (isize, isize)) -> char {
    if (0..map.len() as isize).contains(&p.0) && (0..map[0].len() as isize).contains(&p.1) {
        return map[p.0 as usize][p.1 as usize];
    }
    '#' // out of bounds
}

pub fn char_position(map: &Vec<Vec<char>>, c: char) -> (isize, isize) {
    let v = iproduct!(0..map.len(), 0..map[0].len())
        .find(|(y, x)| map[*y][*x] == c)
        .unwrap();
    (v.0 as isize, v.1 as isize)
}

pub fn manhatten(a: Point<isize>, b: Point<isize>) -> isize {
    let d = b - a;
    d.x.abs() + d.y.abs()
}

fn shortest_path(
    map: &Vec<Vec<char>>,
    start_pos: (isize, isize),
    end_pos: (isize, isize),
) -> Option<Vec<(isize, isize)>> {
    let mut positions = BinaryHeap::new();
    positions.push((-1, vec![start_pos]));

    while let Some((_, path)) = positions.pop() {
        let p = path.last().unwrap();
        if *p == end_pos {
            return Some(path);
        }
        let c = map_value(&map, *p);
        if c == '#' {
            continue;
        }
        for next_pos in adjacent(*p) {
            if path.contains(&next_pos) {
                continue;
            }
            let mut next_path = path.clone();
            next_path.push(next_pos);
            positions.push((-(next_path.len() as i32), next_path.clone()));
        }
    }
    None
}

fn find_cheats(path: &Vec<(isize, isize)>, cheat_distance: i32) -> Vec<i32> {
    let known_psecs: HashMap<Point<isize>, i32> = path
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, (y, x))| (point(x, y), i as i32))
        .collect();

    known_psecs
        .keys()
        .combinations(2)
        .map(|p| {
            (
                manhatten(*p[0], *p[1]) as i32,
                (known_psecs.get(p[0]).unwrap() - known_psecs.get(p[1]).unwrap()).abs(),
            )
        })
        .filter(|(m, _s)| *m <= cheat_distance)
        .map(|(m, s)| s - m)
        .collect()
}
