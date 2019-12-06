// Day 6

use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let input_filename = "input.txt";
    if let Err(e) = run(input_filename) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    // Read the input file
    let contents = fs::read_to_string(filename)?;
    // Create the set of nodes and directed edges, build a graph
    // let mut edge_list: Vec<Link> = Vec::new();
    // Iternally, a graph is just a list of nodes, and a set of edges.edge_list

    let contents = "COM)B
    B)C
    C)D
    D)E
    E)F
    B)G
    G)H
    D)I
    E)J
    J)K
    K)L";
    let mut edge_list: Vec<Link> = Vec::new();
    for line in contents.lines() {
        let edge: Vec<&str> = line.trim().split(")").collect();
        let link: Link = Link {
            source: edge[0].to_string(),
            target: edge[1].to_string(),
        };
        edge_list.push(link);
    }
    let edge_list = edge_list; // make immutable
    let graph = Graph::new(&edge_list);

    println!("{:?}", graph.node_set);

    for node in &graph.node_set {
        if node == "COM" {
            continue;
        }

        let distance: u32 = graph.distance_from_root(&node).clone();

        println!("{} distance from root: {}", node, distance);
    }

    Ok(())
}

#[derive(Debug)]
struct Graph {
    edge_list: Vec<Link>,
    node_set: std::collections::HashSet<String>,
}

impl Graph {
    fn new(edge_list: &Vec<Link>) -> Graph {
        // initialise the Graph from an edge_list
        let mut node_set = std::collections::HashSet::new();
        for edge in edge_list.clone() {
            let (node_1, node_2) = (edge.source, edge.target);
            // println!("{}, {}", node_1, node_2);
            if !node_set.contains(&node_1) {
                node_set.insert(node_1);
            }
            if !node_set.contains(&node_2) {
                node_set.insert(node_2);
            }
        }
        Graph {
            edge_list: edge_list.clone(),
            node_set: node_set,
        }
    }

    //  Get the distance from a node to the root node, "COM"
    fn distance_from_root(&self, node: &String) -> u32 {
        if !self.node_set.contains(node) {
            panic!("node not found!");
        }

        // perform a search from the root node to the target node

        0
    }
}

#[derive(Debug, Clone)]
struct Link {
    source: String,
    target: String,
}
