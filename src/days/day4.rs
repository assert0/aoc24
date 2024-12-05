use itertools::{iproduct, Itertools};
use std::fs;

lazy_static! {
    static ref DIRS: Vec<(isize, isize)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
}

pub fn day4(args: &[String]) {
    println!("Day 4");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let search: Vec<Vec<_>> = contents.lines().map(|l| l.chars().collect()).collect();

    let part1 = iproduct!(0..search.len(), 0..search[0].len(), DIRS.clone())
        .filter(|(y, x, d)| is_xmas(&search, (*y, *x), *d))
        .count();

    println!("Part 1: {}", part1);

    let part2 = iproduct!(0..search.len(), 0..search[0].len())
        .filter(|(y, x)| is_x_mas(&search, (*y, *x)))
        .count();

    println!("Part 2: {}", part2);
}

pub fn is_xmas(search: &Vec<Vec<char>>, pos: (usize, usize), direction: (isize, isize)) -> bool {
    "XMAS".chars().enumerate().all(|(i, c)| {
        let (y, x) = (
            pos.0 as isize + i as isize * direction.0,
            pos.1 as isize + i as isize * direction.1,
        );
        c == get_value(search, (y, x)).unwrap_or('.')
    })
}

pub fn is_x_mas(search: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    let (tl, tr, c, bl, br) = vec![(-1, -1), (-1, 1), (0, 0), (1, -1), (1, 1)]
        .iter()
        .map(|(y, x)| get_value(search, (pos.0 as isize + y, pos.1 as isize + x)).unwrap_or('.'))
        .collect_tuple()
        .unwrap();

    let mas_orders = vec![('M', 'A', 'S'), ('S', 'A', 'M')];
    mas_orders.contains(&(tl, c, br)) && mas_orders.contains(&(tr, c, bl))
}

pub fn get_value(search: &Vec<Vec<char>>, pos: (isize, isize)) -> Option<char> {
    if (0..search.len() as isize).contains(&pos.0)
        && (0..search[0].len() as isize).contains(&pos.1)
    {
        return Some(search[pos.0 as usize][pos.1 as usize]);
    }
    None
}
