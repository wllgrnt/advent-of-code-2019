// Day 5

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
    let mut edge_list: Vec<Link> = Vec::new();
    for line in contents.lines() {
        let edge: Vec<&str> = line.split(")").collect();
        let link: Link = Link {
            source: edge[0].to_string(),
            target: edge[1].to_string(),
        };
        edge_list.push(link);
    }

    Ok(())
}

#[derive(Debug)]
struct Link {
    source: String,
    target: String,
}
