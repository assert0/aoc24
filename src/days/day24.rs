use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

lazy_static! {
// These swaps were manually found
static ref SWAPS: HashMap<String, String> = HashMap::from([
    ("z09".to_string(), "nnf".to_string()),
    ("nnf".to_string(), "z09".to_string()),
    ("z20".to_string(), "nhs".to_string()),
    ("nhs".to_string(), "z20".to_string()),
    ("ddn".to_string(), "kqh".to_string()),
    ("kqh".to_string(), "ddn".to_string()),
    ("z34".to_string(), "wrc".to_string()),
    ("wrc".to_string(), "z34".to_string()),
    ]);
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Gate {
    ina: String,
    inb: String,
    op: String,
    out: String,
}

impl FromStr for Gate {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();
        let caps = re.captures(s).unwrap();
        let ina = caps[1].to_string();
        let op = caps[2].to_string();
        let inb = caps[3].to_string();
        let out = caps[4].to_string();

        Ok(Self {
            ina: ina,
            inb: inb,
            op: op,
            out: out,
        })
    }
}

pub fn day24(args: &[String]) {
    println!("Day 24");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut sections = contents.split("\n\n");
    let inputs: HashMap<String, usize> = sections
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let re = Regex::new(r"(\w+): (0|1)").unwrap();
            let caps = re.captures(l).unwrap();
            (
                caps[1].to_string(),
                if caps[2].to_string() == "1" { 1 } else { 0 },
            )
        })
        .collect();
    let gates: Vec<Gate> = sections
        .next()
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    // println!("{:?}", starting);
    // println!("{:?}", inputs);
    // println!("{:?}", gates);

    let part1 = solve(&inputs, &gates);
    println!("Part 1: {}", part1);

    // Apply wires fixes
    let gates = gates
        .clone()
        .into_iter()
        .map(|g| Gate {
            ina: g.ina,
            inb: g.inb,
            op: g.op,
            out: SWAPS.get(&g.out).unwrap_or(&g.out).to_string(),
        })
        .collect();

    // build full adders
    let mut carry = None;
    for b in 0..=44 {
        println!("");
        println!("carry: {:?}", carry);
        let x = format!("x{:02}", b);
        let y = format!("y{:02}", b);
        // let z = format!("z{:02}", b);
        let ab_xor = find_gate(&gates, &x, &y, "XOR").unwrap();
        println!("{:?}", ab_xor);
        let ab_and = find_gate(&gates, &x, &y, "AND").unwrap();
        println!("{:?}", ab_and);
        if carry.is_none() {
            // half adder
            carry = Some(ab_and.out.clone());
        } else {
            // full adder
            let oc_xor = find_gate(&gates, &ab_xor.clone().out, &carry.clone().unwrap(), "XOR");
            println!("oc_xor: {:?}", oc_xor);

            let oc_and = find_gate(&gates, &ab_xor.out, &carry.clone().unwrap(), "AND");
            println!("oc_and: {:?}", oc_and);
            if oc_and.is_some() {
                let or = find_gate(&gates, &oc_and.clone().unwrap().out, &ab_and.out, "OR");
                println!("or: {:?}", or);
                if or.is_some() {
                    carry = Some(or.unwrap().out.clone());
                } else {
                    carry = None;
                }
            }
        }
    }

    let mut wires: Vec<_> = SWAPS.keys().collect();
    wires.sort();
    println!("Part 2: {}", wires.iter().map(|s| s.as_str()).join(","));
}

fn solve(inputs: &HashMap<String, usize>, gates: &Vec<Gate>) -> usize {
    let mut i = inputs.clone();
    let mut allz: Vec<String> = gates
        .iter()
        .map(|g| g.out.clone())
        .filter(|o| o.starts_with("z"))
        .collect();
    allz.sort();

    loop {
        let mut inserted = vec![];
        for g in gates {
            if i.contains_key(&g.ina) && i.contains_key(&g.inb) {
                let outv = match g.op.as_str() {
                    "AND" => i.get(&g.ina).unwrap() & i.get(&g.inb).unwrap(),
                    "OR" => i.get(&g.ina).unwrap() | i.get(&g.inb).unwrap(),
                    "XOR" => i.get(&g.ina).unwrap() ^ i.get(&g.inb).unwrap(),
                    _ => unreachable!(),
                };
                inserted.push(g.out.clone());
                let e = i.entry(g.out.clone()).or_insert(0);
                *e = outv;
            }
        }
        if allz.iter().all(|z| inserted.contains(z)) {
            break;
        }
    }
    allz.iter().rev().fold(0, |v, z| v << 1 | i.get(z).unwrap())
}

fn find_gate(gates: &Vec<Gate>, ina: &String, inb: &String, op: &str) -> Option<Gate> {
    // gate order doesn't matter
    gates
        .iter()
        .find(|g| {
            ((g.ina == *ina && g.inb == *inb) || (g.ina == *inb && g.inb == *ina)) && g.op == *op
        })
        .cloned()
}

// fn find_gate_or(gates: &Vec<Gate>, op: &String, out: &String) -> Option<Gate> {
//     // gate order doesn't matter
//     gates.iter().find(|g| g.out == *out && g.op == *op).cloned()
// }
