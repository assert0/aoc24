use itertools::Itertools;
use std::collections::BinaryHeap;
use std::fs;

pub fn day18(args: &[String]) {
    println!("Day 18");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let positions: Vec<(usize, usize)> = contents
        .lines()
        .map(|l| {
            l.split(",")
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut map: Vec<Vec<char>> = if positions.len() == 25 {
        vec![vec!['.'; 7]; 7]
    } else {
        vec![vec!['.'; 71]; 71]
    };
    let bytes = if positions.len() == 25 { 12 } else { 1024 };
    for p in &positions[0..bytes] {
        map[p.1][p.0] = '#';
    }
    // print_map(&map);
    let part1 = shortest_path(&map);
    println!("Part 1: {}", part1.unwrap());

    for p in &positions[bytes..] {
        map[p.1][p.0] = '#';
        if shortest_path(&map).is_none() {
            println!("Part 2: {},{}", p.0, p.1);
            break;
        }
    }
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
    '#'
}

fn shortest_path(map: &Vec<Vec<char>>) -> Option<i32> {
    let mut lowest = vec![vec![i32::MAX; map[0].len()]; map.len()];
    let mut positions = BinaryHeap::new();
    positions.push((0, (0, 0)));
    let dest = (map.len() as isize - 1, map[0].len() as isize - 1);

    while let Some((cur_score, cur_pos)) = positions.pop() {
        if cur_pos == dest {
            return Some(-cur_score);  // at end
        }
        for next_pos in adjacent(cur_pos) {
            if map_value(&map, next_pos) == '#' {
                continue;
            }
            let next_score = -cur_score + 1;
            if next_score < lowest[next_pos.0 as usize][next_pos.1 as usize] {
                positions.push((-next_score, next_pos));
                lowest[next_pos.0 as usize][next_pos.1 as usize] = next_score;
            }
        }
    }
    None
}
