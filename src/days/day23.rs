use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use tqdm::tqdm;

pub fn day23(args: &[String]) {
    println!("Day 23");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let connections: Vec<(String, String)> = contents
        .lines()
        .map(|l| l.split("-").map(|s| s.to_string()).collect_tuple().unwrap())
        .collect();
    let nodes: HashSet<_> = connections
        .clone()
        .into_iter()
        .map(|(a, b)| vec![a.to_string(), b.to_string()])
        .flatten()
        .collect();
    // println!("{}: {:?}", nodes.len(), nodes);

    let mut nodes_connections = HashMap::new();
    nodes.clone().into_iter().for_each(|n| {
        nodes_connections.insert(n.clone(), node_connections(&connections, n));
    });

    let part1 = tqdm(nodes.clone().into_iter().combinations(3))
        .filter(|n| starts_with_t(n))
        .filter(|n| are_connected(n, &connections))
        .count();
    println!("Part 1: {:?}", part1);

    let best = nodes
        .clone()
        .into_iter()
        .map(|n| node_interestions(&nodes_connections, n.clone()))
        .max()
        .unwrap();
    let mut best_nodes: Vec<_> = nodes
        .clone()
        .into_iter()
        .filter(|n| best == node_interestions(&nodes_connections, n.clone()))
        .collect();
    best_nodes.sort();
    println!("Part 2: {}", best_nodes.join(","));
}

fn are_connected(nodes: &Vec<String>, connections: &Vec<(String, String)>) -> bool {
    nodes.iter().combinations(2).all(|n| {
        connections.contains(&(n[0].clone(), n[1].clone()))
            || connections.contains(&(n[1].clone(), n[0].clone()))
    })
}

fn starts_with_t(nodes: &Vec<String>) -> bool {
    nodes.iter().any(|n| n.starts_with("t"))
}

fn node_connections(connections: &Vec<(String, String)>, node: String) -> HashSet<String> {
    connections
        .clone()
        .into_iter()
        .filter_map(|(a, b)| {
            if a == node {
                Some(b)
            } else if b == node {
                Some(a)
            } else {
                None
            }
        })
        .collect()
}

fn node_interestions(nodes_connections: &HashMap<String, HashSet<String>>, node: String) -> usize {
    let mut root_connections: HashSet<_> = nodes_connections.get(&node).unwrap().clone();
    let nodes: Vec<_> = root_connections.clone().into_iter().collect();
    root_connections.insert(node.clone());
    let mut count = 0;
    for rc in nodes.clone() {
        let c: HashSet<String> = nodes_connections.get(&rc).unwrap().clone();
        let nc: HashSet<String> = root_connections.intersection(&c).cloned().collect();
        if nc.len() + 1 >= nodes.len() {
            count += 1;
        }
    }
    count
}
