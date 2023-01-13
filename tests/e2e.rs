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

#[test]
fn lets_test_some_formulas() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str(
        "\
        a     | =sum(1,2,incfrom(3)) | table \n\
        with  | =^^                  | formulas\
        ",
    )?;

    let mut cmd = Command::cargo_bin("cell")?;
    cmd.arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains(
        "a    | 6 | table\n\
        with | 7 | formulas\n\n",
    ));

    Ok(())
}

#[test]
fn the_full_challenge_e2e_true() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str(
        "!date|!transaction_id|!tokens|!token_prices|!total_cost
        2022-02-20|=concat(\"t_\", text(incFrom(1)))|btc,eth,dai|38341.88,2643.77,1.0003|=sum(split(D2, \",\"))
        2022-02-21|=^^|bch,eth,dai|304.38,2621.15,1.0001|=E^+sum(split(D3, \",\"))
        2022-02-22|=^^|sol,eth,dai|85,2604.17,0.9997|=^^
        !fee|!cost_threshold|||
        0.09|10000|||
        !adjusted_cost||||
        =E^v+(E^v*A6)||||
        !cost_too_high||||
        =text(gte(@adjusted_cost<1>, @cost_threshold<1>))||||",
    )?;

    let mut cmd = Command::cargo_bin("cell")?;
    cmd.arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains(
        "\n\
        !date              | !transaction_id | !tokens     | !token_prices           | !total_cost\n\
        2022-02-20         | t_1             | btc,eth,dai | 38341.88,2643.77,1.0003 | 40986.65\n\
        2022-02-21         | t_2             | bch,eth,dai | 304.38,2621.15,1.0001   | 43913.180100000005\n\
        2022-02-22         | t_3             | sol,eth,dai | 85,2604.17,0.9997       | 46603.3498\n\
        !fee               | !cost_threshold |             |                         | \n\
        0.09               | 10000           |             |                         | \n\
        !adjusted_cost     |                 |             |                         | \n\
        50797.651282000006 |                 |             |                         | \n\
        !cost_too_high     |                 |             |                         | \n\
        true               |                 |             |                         | \n\
        \n",
    ));

    Ok(())
}

#[test]
fn the_full_challenge_e2e_false() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str(
        "!date|!transaction_id|!tokens|!token_prices|!total_cost
        2022-02-20|=concat(\"t_\", text(incFrom(1)))|btc,eth,dai|38341.88,2643.77,1.0003|=sum(split(D2, \",\"))
        2022-02-21|=^^|bch,eth,dai|304.38,2621.15,1.0001|=E^+sum(split(D3, \",\"))
        2022-02-22|=^^|sol,eth,dai|85,2604.17,0.9997|=^^
        !fee|!cost_threshold|||
        0.09|51000|||
        !adjusted_cost||||
        =E^v+(E^v*A6)||||
        !cost_too_high||||
        =text(gte(@adjusted_cost<1>, @cost_threshold<1>))||||",
    )?;

    let mut cmd = Command::cargo_bin("cell")?;
    cmd.arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains(
        "\n\
        !date              | !transaction_id | !tokens     | !token_prices           | !total_cost\n\
        2022-02-20         | t_1             | btc,eth,dai | 38341.88,2643.77,1.0003 | 40986.65\n\
        2022-02-21         | t_2             | bch,eth,dai | 304.38,2621.15,1.0001   | 43913.180100000005\n\
        2022-02-22         | t_3             | sol,eth,dai | 85,2604.17,0.9997       | 46603.3498\n\
        !fee               | !cost_threshold |             |                         | \n\
        0.09               | 51000           |             |                         | \n\
        !adjusted_cost     |                 |             |                         | \n\
        50797.651282000006 |                 |             |                         | \n\
        !cost_too_high     |                 |             |                         | \n\
        false              |                 |             |                         | \n\
        \n",
    ));

    Ok(())
}
