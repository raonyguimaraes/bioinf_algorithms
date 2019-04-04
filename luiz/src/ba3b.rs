use std::env;
use std::error::Error;
use std::fs;

pub fn genome_from_path(text: Vec<&str>) -> String {
    let mut seq: String = text[0].into();
    for kmer in &text[1..] {
        seq.push(kmer.chars().last().unwrap());
    }
    seq
}

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        genome_from_path(vec!["ACCGA", "CCGAA", "CGAAG", "GAAGC", "AAGCT",]),
        "ACCGAAGCT"
    );

    let input: String = env::args().nth(1).expect("Input data file missing");
    let data = fs::read_to_string(input)?;
    let lines = data.lines().collect();

    println!("{}", genome_from_path(lines));

    Ok(())
}
