use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::str;

/* Mirror the product(repeat=n) iter from Python itertools
    https://github.com/python/cpython/blob/234531b4462b20d668762bd78406fd2ebab129c9/Modules/itertoolsmodule.c#L2095
    https://dev.to/naufraghi/procedural-macro-in-rust-101-k3f
pub fn product<I, T>(iter: I, repeat: usize) -> Vec<T>
where
    I: Iterator<Item = T>,
{
    Vec::new()
}
*/

pub fn revc(text: &str) -> String {
    text.chars()
        .rev()
        .map(|x| match x {
            'A' => "T",
            'C' => "G",
            'G' => "C",
            'T' => "A",
            _ => panic!(),
        })
        .collect()
}

pub fn hamming(seq1: &str, seq2: &str) -> usize {
    seq1.chars()
        .zip(seq2.chars())
        .filter(|(b1, b2)| b1 != b2)
        .count()
}

pub fn neighbors(pattern: &str, d: usize) -> HashSet<String> {
    if d == 0 {
        return HashSet::from_iter(vec![pattern.into()]);
    }

    if pattern.len() == 1 {
        return HashSet::from_iter(vec!["A".into(), "C".into(), "G".into(), "T".into()]);
    }

    let mut neighborhood = HashSet::default();

    for t in neighbors(&pattern[1..], d) {
        if hamming(&pattern[1..], &t) < d {
            for bp in vec!["A", "C", "G", "T"].into_iter() {
                neighborhood.insert(bp.to_owned() + &t);
            }
        } else {
            neighborhood.insert(pattern[0..1].to_owned() + &t);
        }
    }

    neighborhood
}

pub fn composition<'a>(k: usize, text: &'a str) -> impl Iterator<Item = &'a str> {
    text.as_bytes()
        .windows(k)
        .map(|w| str::from_utf8(w).unwrap())
}

// TODO: avoid creating adj_list, rewrite the loop as iter code and return it
// before collect?
pub fn overlap<'a>(text: &'a [&str]) -> impl Iterator<Item = (String, String)> + 'a {
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

    adj_list.into_iter().flat_map(move |(node, neighbors)| {
        neighbors
            .into_iter()
            .map(move |neighbor| (node.to_string(), neighbor.to_string()))
    })
}

pub type Matrix = Vec<Vec<f64>>;

pub fn kmer_prob(kmer: &[u8], matrix: &Matrix) -> f64 {
    let mut prob = 1.;
    for (i, nt) in kmer.iter().enumerate() {
        let pos = match nt {
            b'A' => 0,
            b'C' => 1,
            b'G' => 2,
            b'T' => 3,
            _ => unimplemented!(),
        };
        prob = prob * matrix[pos][i];
    }

    prob
}

pub fn profile_most_probable(text: &str, k: usize, matrix: &Matrix) -> String {
    let mut most_probable_kmer = text[0..k].as_bytes();
    let mut max_prob = kmer_prob(most_probable_kmer, matrix);

    for kmer in text[1..].as_bytes().windows(k) {
        let prob = kmer_prob(kmer, matrix);
        if prob > max_prob {
            max_prob = prob;
            most_probable_kmer = kmer
        }
    }

    String::from_utf8_lossy(most_probable_kmer).into()
}
