use std::io::Write;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::str::contains;
use tempfile::NamedTempFile;

#[test]
fn ba2a_test() -> Result<(), Box<std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "3 1\nATTTGGC\nTGCCTTA\nCGGTATC\nGAAAATT")?;

    let mut cmd = Command::cargo_bin("ba2a")?;
    cmd.arg(file.path());
    cmd.assert().success();

    for kmer in &["ATA", "ATT", "GTT", "TTT"] {
        cmd.assert().stdout(contains(*kmer));
    }

    Ok(())
}
