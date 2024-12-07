use itertools::iproduct;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Moving {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum State {
    Moving,
    Leaving,
    Looping,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Guard {
    position: (isize, isize),
    moving: Moving,
    visited: HashSet<(isize, isize)>,
    moving_visited: HashSet<(Moving, isize, isize)>,
}

impl Guard {
    fn new(position: (isize, isize), moving: Moving) -> Self {
        Self {
            position,
            moving,
            moving_visited: HashSet::new(),
            visited: HashSet::new(),
        }
    }

    fn step(&mut self, map: &Vec<Vec<char>>) -> State {
        if (0..map.len() as isize).contains(&self.position.0)
            && (0..map[0].len() as isize).contains(&self.position.1)
        {
            let (y, x) = self.position;
            let (m, dy, dx) = match map[y as usize][x as usize] {
                '.' => {
                    self.visited.insert((y, x));
                    if !self.moving_visited.insert((self.moving, y, x)) {
                        return State::Looping;
                    }
                    match self.moving {
                        Moving::Right => (Moving::Right, 0, 1),
                        Moving::Left => (Moving::Left, 0, -1),
                        Moving::Up => (Moving::Up, -1, 0),
                        Moving::Down => (Moving::Down, 1, 0),
                    }
                }
                // turn right
                '#' => match self.moving {
                    Moving::Right => (Moving::Down, 1, -1),
                    Moving::Left => (Moving::Up, -1, 1),
                    Moving::Up => (Moving::Right, 1, 1),
                    Moving::Down => (Moving::Left, -1, -1),
                },
                _ => unreachable!(),
            };
            self.position = (y + dy, x + dx);
            self.moving = m;
            return State::Moving;
        }
        State::Leaving
    }
}

pub fn day6(args: &[String]) {
    println!("Day 6");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut map: Vec<Vec<_>> = contents.lines().map(|l| l.chars().collect()).collect();

    let start = iproduct!(0..map.len(), 0..map[0].len())
        .find(|(y, x)| map[*y][*x] == '^')
        .unwrap();
    map[start.0][start.1] = '.';

    let start = (start.0 as isize, start.1 as isize);
    let mut guard = Guard::new(start, Moving::Up);
    loop {
        if guard.step(&map) == State::Leaving {
            break;
        };
    }
    println!("Part 1: {}", guard.visited.len());

    let mut visited_pos = guard.visited.clone();
    visited_pos.remove(&start);
    let part2 = visited_pos
        .iter()
        .filter(|p| {
            let mut map2 = map.clone();
            map2[p.0 as usize][p.1 as usize] = '#';
            let mut guard2 = Guard::new(start, Moving::Up);
            loop {
                match guard2.step(&map2) {
                    State::Leaving => return false,
                    State::Looping => return true,
                    _ => {}
                };
            }
        })
        .count();
    println!("Part 2: {}", part2);
}
