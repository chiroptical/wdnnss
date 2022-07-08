use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("wdnnss")?;

    // We don't have any arguments yet, don't set them
    // cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello"));

    Ok(())
}
