use std::collections::HashSet;
use std::iter::FromIterator;

/* Mirror the product(repeat=n) iter from Python itertools
    https://github.com/python/cpython/blob/234531b4462b20d668762bd78406fd2ebab129c9/Modules/itertoolsmodule.c#L2095
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
