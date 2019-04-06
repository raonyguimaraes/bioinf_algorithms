use std::io::Write;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::str::contains;
use tempfile::NamedTempFile;

#[test]
fn ba2a_test() -> Result<(), Box<std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "3 1
ATTTGGC
TGCCTTA
CGGTATC
GAAAATT"
    )?;

    let mut cmd = Command::cargo_bin("ba2a")?;
    cmd.arg(file.path());
    cmd.assert().success();

    for kmer in &["ATA", "ATT", "GTT", "TTT"] {
        cmd.assert().stdout(contains(*kmer));
    }

    Ok(())
}

#[test]
fn ba2c_test() -> Result<(), Box<std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "ACCTGTTTATTGCCTAAGTTCCGAACAAACCCAATATAGCCCGAGGGCCT
5
0.2 0.2 0.3 0.2 0.3
0.4 0.3 0.1 0.5 0.1
0.3 0.3 0.5 0.2 0.4
0.1 0.2 0.1 0.1 0.2"
    )?;

    let mut cmd = Command::cargo_bin("ba2c")?;
    cmd.arg(file.path());
    cmd.assert().success().stdout("CCGAG\n");

    Ok(())
}
