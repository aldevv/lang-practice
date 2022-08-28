use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    // should die when no argument given
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn runs_basic() -> TestResult {
    // should run with an argument
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let file = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello2() -> TestResult {
    let file = "tests/expected/hello2.txt";
    let expected = fs::read_to_string(file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello", "there"])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello1_using_run() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2_using_run() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline_using_run() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline_using_run() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
