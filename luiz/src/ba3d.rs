use std::collections::HashMap;
use std::error::Error;
use std::fs;

use bioinformatics_algorithms::{composition, overlap};

pub fn de_bruijn(k: usize, text: &str) -> HashMap<String, Vec<String>> {
    let kmers: Vec<&str> = composition(k - 1, text).collect();
    let edges = overlap(&kmers);

    let mut dbg = HashMap::default();

    edges
        .into_iter()
        .map(|(kmer, neighbor)| {
            dbg.entry(kmer).or_insert(Vec::new()).push(neighbor);
        })
        .count();

    dbg
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("data/rosalind_ba3d.txt")?;
    let mut lines = data.lines();
    let k = lines.next().unwrap().parse()?;
    let text: String = lines.next().unwrap().into();

    for (node, neighbors) in de_bruijn(k, &text) {
        if neighbors.len() == 1 {
            println!("{} -> {}", node, neighbors[0]);
        } else {
            print!("{} -> {}", node, neighbors[0]);

            for neighbor in &neighbors[1..] {
                print!(",{}", neighbor);
            }
            println!();
        }
    }

    Ok(())
}
