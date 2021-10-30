use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn no_args() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("rust-echo")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE:"));
    Ok(())
}

/// Compare contents of test file with `stdout`
fn run(args: &[&str], file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let expected = fs::read_to_string(file_name)?;
    Command::cargo_bin("rust-echo")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn test1() -> Result<(), Box<dyn std::error::Error>> {
    run(&["hello   world"], "tests/expected/test1.txt")
}

#[test]
fn test2() -> Result<(), Box<dyn std::error::Error>> {
    run(&["hello", "world"], "tests/expected/test2.txt")
}

#[test]
fn test3() -> Result<(), Box<dyn std::error::Error>> {
    run(&["-n", "hello", "world"], "tests/expected/test3.txt")
}
