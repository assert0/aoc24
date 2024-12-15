use itertools::iproduct;
use std::collections::HashSet;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Warehouse {
    map: Vec<Vec<char>>,
    robot: (usize, usize),
}

impl FromStr for Warehouse {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        let r = iproduct!(0..m.len(), 0..m[0].len())
            .find(|(y, x)| m[*y][*x] == '@')
            .unwrap();
        m[r.0][r.1] = '.';
        Ok(Self { map: m, robot: r })
    }
}

impl Warehouse {
    fn step(&mut self, direction: char) {
        let d: (isize, isize) = match direction {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => unreachable!("Unknown move"),
        };
        let p = (
            (self.robot.0 as isize + d.0) as usize,
            (self.robot.1 as isize + d.1) as usize,
        );
        self.robot = match self.map[p.0][p.1] {
            '.' => p,
            '#' => self.robot,
            'O' => {
                // find an open space to move this box
                let mut nextp = None;
                let mut p2 = p;
                loop {
                    p2 = (
                        (p2.0 as isize + d.0) as usize,
                        (p2.1 as isize + d.1) as usize,
                    );
                    match self.map[p2.0][p2.1] {
                        '.' => {
                            nextp = Some(p2);
                            break;
                        }
                        '#' => {
                            break;
                        }
                        _ => {}
                    };
                }
                if let Some(np) = nextp {
                    self.map[p.0][p.1] = '.';
                    self.map[np.0][np.1] = 'O';
                    p
                } else {
                    self.robot
                }
            }
            _ => unreachable!("Unknown map char"),
        };
    }

    fn gps_sum(&self) -> usize {
        iproduct!(0..self.map.len(), 0..self.map[0].len())
            .filter(|(y, x)| self.map[*y][*x] == 'O')
            .map(|(y, x)| y * 100 + x)
            .sum()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Warehouse2 {
    map: Vec<Vec<char>>,
    robot: (usize, usize),
}

impl FromStr for Warehouse2 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m: Vec<Vec<char>> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '#' => vec!['#', '#'],
                        'O' => vec!['[', ']'],
                        '.' => vec!['.', '.'],
                        '@' => vec!['@', '.'],
                        _ => unreachable!(),
                    })
                    .flatten()
                    .collect::<Vec<char>>()
            })
            .collect();
        let r = iproduct!(0..m.len(), 0..m[0].len())
            .find(|(y, x)| m[*y][*x] == '@')
            .unwrap();
        m[r.0][r.1] = '.';
        Ok(Self { map: m, robot: r })
    }
}

impl Warehouse2 {
    fn step(&mut self, direction: char) {
        let (dir, d): (Direction, (isize, isize)) = match direction {
            '^' => (Direction::Up, (-1, 0)),
            'v' => (Direction::Down, (1, 0)),
            '<' => (Direction::Left, (0, -1)),
            '>' => (Direction::Right, (0, 1)),
            _ => unreachable!("Unknown move"),
        };
        let p = (
            (self.robot.0 as isize + d.0) as usize,
            (self.robot.1 as isize + d.1) as usize,
        );
        let c = self.map[p.0][p.1];
        self.robot = match c {
            '.' => p,
            '#' => self.robot,
            '[' | ']' => {
                let block = if c == '[' { p } else { (p.0, p.1 - 1) };
                if self.move_blocks(block, dir) {
                    p
                } else {
                    self.robot
                }
            }
            _ => unreachable!("Unknown map char"),
        }
    }

    fn move_blocks(&mut self, block: (usize, usize), dir: Direction) -> bool {
        let testblocks = self.connected_blocks(block, dir);
        // println!("connected: {:?}", testblocks);
        let mut warehouse_temp = self.map.clone();
        let allmoved = testblocks.iter().rev().all(|l| {
            l.iter()
                .all(|b| Self::move_block(&mut warehouse_temp, *b, dir))
        });
        if allmoved {
            self.map = warehouse_temp;
        }
        allmoved
    }

    fn connected_blocks(
        &mut self,
        block: (usize, usize),
        dir: Direction,
    ) -> Vec<HashSet<(usize, usize)>> {
        let mut connected = vec![];
        let mut testblocks = vec![(0, block)];
        while let Some((l, b)) = testblocks.pop() {
            if l + 1 > connected.len() {
                connected.push(HashSet::new());
            }
            connected[l].insert(b.clone());
            match dir {
                Direction::Down | Direction::Up => {
                    let ny = if dir == Direction::Up {
                        b.0 - 1
                    } else {
                        b.0 + 1
                    };
                    for nx in b.1 - 1..=b.1 + 1 {
                        if self.map[ny][nx] == '[' {
                            testblocks.push((l + 1, (ny, nx)));
                        }
                    }
                }
                Direction::Left | Direction::Right => {
                    let nx = if dir == Direction::Left {
                        b.1 - 2
                    } else {
                        b.1 + 2
                    };
                    if self.map[b.0][nx] == '[' {
                        testblocks.push((l + 1, (b.0, nx)));
                    }
                }
            }
        }
        connected
    }

    // block is the left edge position
    fn move_block(map: &mut Vec<Vec<char>>, block: (usize, usize), dir: Direction) -> bool {
        if Self::can_move(map, block, dir) {
            match dir {
                Direction::Up => {
                    map[block.0 - 1][block.1] = '[';
                    map[block.0 - 1][block.1 + 1] = ']';
                    map[block.0][block.1] = '.';
                    map[block.0][block.1 + 1] = '.';
                }
                Direction::Down => {
                    map[block.0 + 1][block.1] = '[';
                    map[block.0 + 1][block.1 + 1] = ']';
                    map[block.0][block.1] = '.';
                    map[block.0][block.1 + 1] = '.';
                }
                Direction::Left => {
                    map[block.0][block.1 - 1] = '[';
                    map[block.0][block.1] = ']';
                    map[block.0][block.1 + 1] = '.';
                }
                Direction::Right => {
                    map[block.0][block.1] = '.';
                    map[block.0][block.1 + 1] = '[';
                    map[block.0][block.1 + 2] = ']';
                }
            }
            return true;
        }
        false
    }

    fn can_move(map: &Vec<Vec<char>>, block: (usize, usize), dir: Direction) -> bool {
        match dir {
            Direction::Down | Direction::Up => {
                let ny = if dir == Direction::Up {
                    block.0 - 1
                } else {
                    block.0 + 1
                };
                map[ny][block.1] == '.' && map[ny][block.1 + 1] == '.'
            }
            Direction::Left | Direction::Right => {
                let nx = if dir == Direction::Left {
                    block.1 - 1
                } else {
                    block.1 + 2
                };
                map[block.0][nx] == '.'
            }
        }
    }

    fn gps_sum(&self) -> usize {
        iproduct!(0..self.map.len(), 0..self.map[0].len())
            .filter(|(y, x)| self.map[*y][*x] == '[')
            .map(|(y, x)| y * 100 + x)
            .sum()
    }
}

pub fn day15(args: &[String]) {
    println!("Day 15");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut sections = contents.split("\n\n");

    let mut warehouse: Warehouse = sections.next().unwrap().parse().unwrap();
    let moves: Vec<char> = sections
        .next()
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .flatten()
        .collect();

    // println!("Start: {:?}", warehouse.robot);
    moves.iter().for_each(|m| warehouse.step(*m));
    println!("Part 1: {}", warehouse.gps_sum());

    sections = contents.split("\n\n");
    let mut warehouse2: Warehouse2 = sections.next().unwrap().parse().unwrap();
    // println!("Start2: {:?}", warehouse2.robot);
    moves.iter().for_each(|m| {
        warehouse2.step(*m);
    });
    // print_warehouse(&warehouse2.map, warehouse2.robot);
    println!("Part 2: {}", warehouse2.gps_sum());
}

fn print_warehouse(map: &Vec<Vec<char>>, robot: (usize, usize)) {
    let mut output = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if (y, x) == robot {
                output.push("@".to_string());
            } else {
                output.push(map[y][x].to_string());
            }
        }
        output.push("\n".to_string());
    }
    println!("{}", output.join(""));
}
