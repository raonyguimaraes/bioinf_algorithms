use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs;

pub fn overlap(text: &[&str]) -> HashSet<(String, String)> {
    let mut adj_list = HashMap::with_capacity(text.len());

    for kmer in text {
        let mut neighbors = Vec::with_capacity(text.len());

        for other in text {
            if other == kmer {
                continue;
            } else if kmer[1..] == other[..kmer.len() - 1] {
                neighbors.push(other);
            }
        }

        adj_list.insert(kmer, neighbors);
    }

    adj_list
        .into_iter()
        .flat_map(move |(node, neighbors)| {
            neighbors
                .into_iter()
                .map(move |neighbor| (node.to_string(), neighbor.to_string()))
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        overlap(&["ATGCG", "GCATG", "CATGC", "AGGCA", "GGCAT"]),
        [
            ("AGGCA".into(), "GGCAT".into()),
            ("CATGC".into(), "ATGCG".into()),
            ("GCATG".into(), "CATGC".into()),
            ("GGCAT".into(), "GCATG".into())
        ]
        .iter()
        .cloned()
        .collect()
    );

    let input: String = env::args().nth(1).expect("Input data file missing");
    let data = fs::read_to_string(input)?;
    let lines: Vec<&str> = data.lines().collect();

    for (node, neighbor) in overlap(&lines) {
        println!("{} -> {}", node, neighbor);
    }

    Ok(())
}
