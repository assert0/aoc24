use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

use itertools::iproduct;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Moving {
    Right,
    Left,
    Up,
    Down,
}

fn moving_index(moving: Moving) -> usize {
    match moving {
        Moving::Right => 0,
        Moving::Left => 1,
        Moving::Up => 2,
        Moving::Down => 3,
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    score: usize,
    path: Vec<(isize, isize)>,
    moving: Moving,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.score.cmp(&self.score)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn day16(args: &[String]) {
    println!("Day 16");
    if args.len() != 1 {
        println!("Missing input file.");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let map: Vec<Vec<_>> = contents.lines().map(|l| l.chars().collect()).collect();

    let (part1, part2) = shortest_path(&map);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

lazy_static! {
    static ref ADJ: Vec<(isize, isize, Moving)> = vec![
        (0, 1, Moving::Right),
        (0, -1, Moving::Left),
        (1, 0, Moving::Down),
        (-1, 0, Moving::Up)
    ];
}

pub fn adjacent(pos: (isize, isize)) -> Vec<(isize, isize, Moving)> {
    ADJ.iter()
        .map(|(dy, dx, moving)| (pos.0 + dy, pos.1 + dx, *moving))
        .collect()
}

pub fn char_position(map: &Vec<Vec<char>>, c: char) -> (isize, isize) {
    let v = iproduct!(0..map.len(), 0..map[0].len())
        .find(|(y, x)| map[*y][*x] == c)
        .unwrap();
    (v.0 as isize, v.1 as isize)
}

fn shortest_path(map: &Vec<Vec<char>>) -> (usize, usize) {
    let mut lowest = vec![vec![vec![usize::MAX; 4]; map[0].len()]; map.len()];
    let mut seats = HashSet::new();
    let mut paths = BinaryHeap::new();
    paths.push(State {
        score: 0,
        path: vec![char_position(&map, 'S')],
        moving: Moving::Right,
    });

    let mut best = usize::MAX;
    while let Some(current) = paths.pop() {
        let (score, path, moving) = (current.score, current.path, current.moving);

        let cur_p = *path.last().unwrap();
        let cur_c = map[cur_p.0 as usize][cur_p.1 as usize];
        // at end
        if cur_c == 'E' {
            if score <= best {
                best = score;
                for p in path.clone() {
                    seats.insert(p);
                }
            }
        }

        for (y, x, m) in adjacent(cur_p) {
            if map[y as usize][x as usize] == '#' {
                continue;
            }
            let next_score = score + if moving == m { 1 } else { 1001 };
            let next_moving_index = moving_index(m);

            if next_score <= lowest[y as usize][x as usize][next_moving_index] {
                let mut next_path = path.clone();
                next_path.push((y, x));
                paths.push(State {
                    score: next_score,
                    path: next_path,
                    moving: m,
                });
                lowest[y as usize][x as usize][next_moving_index] = next_score;
            }
        }
    }
    (best, seats.len())
}
