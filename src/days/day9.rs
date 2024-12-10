use std::fs;

pub fn day9(args: &[String]) {
    println!("Day 9");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let diskmap: Vec<Option<usize>> = contents.chars().map(|c| c.to_digit(10).unwrap()).enumerate().map(|(i, v)| {
        if i % 2 == 0 { 
            vec![Some(i / 2); v as usize]
        } else {
            vec![None; v as usize]
        }
    }).flatten().collect();
   
    println!("Part 1: {}", part1(diskmap.clone()));  
    println!("Part 2: {}", part2(diskmap.clone()));
}

pub fn part1(mut diskmap: Vec<Option<usize>>) -> usize {
    let (mut start, mut end) = (0, diskmap.len() - 1);
    loop {
        while diskmap[start].is_some() { start += 1; }
        while diskmap[end].is_none(){ end -= 1; }
        if start >= end { break; }
        diskmap[start] = diskmap[end];
        diskmap[end] = None;
    }
    checksum(&diskmap)
}

pub fn part2(mut diskmap: Vec<Option<usize>>) -> usize {
    let mut end = diskmap.len();
    while end > 0 {
        let (laststart, lastsize) = find_last_file(&diskmap, end);
        let spot = find_space(&diskmap, lastsize).unwrap_or(end);
        if spot < laststart {
            let id = diskmap[laststart];
            for i in 0..lastsize {
                diskmap[laststart + i] = None;
            }
            for i in 0..lastsize {
                diskmap[spot + i] = id;
            }
        }
        end = laststart;
    }
    checksum(&diskmap)
}

pub fn checksum(diskmap: &Vec<Option<usize>>) -> usize {
    diskmap.iter().enumerate().map(|(i, v)| {
        if v.is_some() { i * v.unwrap() } else { 0 }
    }).sum()
}

// Return starting position of a block that is at least the size
// of the given blocksize.
pub fn find_space(diskmap: &Vec<Option<usize>>, blocksize: usize) -> Option<usize> {
    let mut size = 0;
    for (i, b) in diskmap.iter().enumerate() {
        if b.is_some() {
            size = 0
        } else {
            size += 1;
            if size == blocksize { 
                return Some(i - size + 1)
            }
        }
    }
    None
}

pub fn find_last_file(diskmap: &Vec<Option<usize>>, end: usize) -> (usize, usize) {
    let mut size = 0;
    let mut id = None;
    for (i, b) in diskmap[0..end].iter().enumerate().rev() {
        if id.is_none() {
            id = Some(*b);
        }
        if id == Some(*b) {
            size += 1;
        } else {
            return (i + 1, size);
        }
    }
    (0, 0)
}
