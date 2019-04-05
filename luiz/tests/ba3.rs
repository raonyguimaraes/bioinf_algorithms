use std::io::Write;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use predicates::str::contains;
use tempfile::NamedTempFile;

#[test]
fn ba3d_test() -> Result<(), Box<std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "4\nAAGATTCTCTAC")?;

    let mut cmd = Command::cargo_bin("ba3d")?;
    cmd.arg(file.path());
    cmd.assert().success();

    let edges = [
        "AAG -> AGA",
        "AGA -> GAT",
        "ATT -> TTC",
        "CTA -> TAC",
        "CTC -> TCT",
        "GAT -> ATT",
        "TTC -> TCT",
    ];

    for edge in &edges {
        cmd.assert().stdout(contains(*edge));
    }

    cmd.assert()
        .stdout(contains("TCT -> CTA,CTC").or(contains("TCT -> CTC,CTA")));

    Ok(())
}
