use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cell")?;

    cmd.arg("test/file/doesnt/exist");
    cmd.assert().failure().stderr(predicate::str::contains(
        "file not found: test/file/doesnt/exist",
    ));

    Ok(())
}

#[test]
fn prints_file_contents_to_stdout() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str(
        "a | sample | table \n\
              with | two | lines",
    )?;

    let mut cmd = Command::cargo_bin("cell")?;
    cmd.arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains(
        "\n\
        a    | sample | table\n\
        with | two    | lines\n\n",
    ));

    Ok(())
}

// @todo make this test pass (implement eval and increment)
// The formula sum(1,2,3) should be incremented to sum(2,3,4)
#[test]
fn lets_test_some_formulas() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str(
        "a | =sum(1,2,3) | table \n\
              with | ^^ | formulas",
    )?;

    let mut cmd = Command::cargo_bin("cell")?;
    cmd.arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains(
        "\n\
        a    | 6 | table\n\
        with | 9 | formulas\n\n",
    ));

    Ok(())
}
