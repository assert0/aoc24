use itertools::iproduct;
use std::fs;

lazy_static! {
    static ref DIRS: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
}

type AreaPerimeter = (Vec<(isize, isize)>, Vec<(isize, isize, isize, isize)>);

pub fn day12(args: &[String]) {
    println!("Day 12");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let map: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();

    let areas = all_areas(&map);
    println!("Part 1: {}", part1(&areas));
    println!("Part 2: {}", part2(&areas));
}

fn all_areas(map: &Vec<Vec<char>>) -> Vec<AreaPerimeter> {
    let mut test_positions: Vec<_> =
        iproduct!(0..map.len() as isize, 0..map[0].len() as isize).collect();

    let mut areas = vec![];
    while !test_positions.is_empty() {
        let (area, perimeter) = get_area_perimeter(&map, test_positions[0]);
        for p in &area {
            test_positions.remove(test_positions.iter().position(|x| *x == *p).unwrap());
        }
        areas.push((area, perimeter));
    }
    areas
}

fn in_map(map: &Vec<Vec<char>>, pos: (isize, isize)) -> bool {
    (0..map.len() as isize).contains(&pos.0) && (0..map[0].len() as isize).contains(&pos.1)
}

fn get_area_perimeter(map: &Vec<Vec<char>>, pos: (isize, isize)) -> AreaPerimeter {
    let label = map[pos.0 as usize][pos.1 as usize];
    let mut test_pos = vec![pos];
    let mut positions = vec![];
    let mut perimeters = vec![];

    while let Some(p) = test_pos.pop() {
        positions.push(p);
        DIRS.iter()
            .map(|(dy, dx)| (*dy, *dx, p.0 + dy, p.1 + dx))
            .for_each(|(dy, dx, y, x)| {
                if !positions.contains(&(y, x)) && !test_pos.contains(&(y, x)) {
                    if in_map(&map, (y, x)) && map[y as usize][x as usize] == label {
                        test_pos.push((y, x));
                    } else {
                        perimeters.push((dy, dx, p.0, p.1));
                    }
                }
            })
    }
    (positions, perimeters)
}

fn part1(areas: &Vec<AreaPerimeter>) -> usize {
    areas.iter().map(|(a, p)| a.len() * p.len()).sum()
}

fn part2(areas: &Vec<AreaPerimeter>) -> usize {
    areas
        .iter()
        .map(|(area, perimeter)| {
            let mut sides = 0;
            let mut p = perimeter.clone();
            while let Some((dy, dx, y, x)) = p.pop() {
                sides += 1;
                // consume matching edge
                if dy == 0 {
                    // vertical edge
                    for m in vec![-1, 1] {
                        for i in 1.. {
                            let index = p.iter().position(|(fdy, fdx, fy, fx)| {
                                *fdy == dy && *fdx == dx && *fy == y + m * i && *fx == x
                            });
                            if index.is_none() {
                                break;
                            }
                            p.remove(index.unwrap());
                        }
                    }
                } else {
                    // horizontal edge
                    for m in vec![-1, 1] {
                        for i in 1.. {
                            let index = p.iter().position(|(fdy, fdx, fy, fx)| {
                                *fdy == dy && *fdx == dx && *fy == y && *fx == x + m * i
                            });
                            if index.is_none() {
                                break;
                            }
                            p.remove(index.unwrap());
                        }
                    }
                }
            }
            area.len() * sides
        })
        .sum()
}
