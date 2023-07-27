use assert_cmd::Command;
use std::error::Error;
use assert_cmd::assert::OutputAssertExt;
type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn no_args() -> TestResult {
    Command::cargo_bin("arrow")
        .unwrap()
        .assert()
        .failure()
        .stderr("Please provide a number as an argument\n");
    Ok(())
}

#[test]
fn negative_number() -> TestResult {
    Command::cargo_bin("arrow")
        .unwrap()
        .args(&["-1"])
        .assert()
        .failure()
        .stderr("Please provide a positive number as an argument\n");
    Ok(())
}

#[test]
fn arrow_1() -> TestResult {
    Command::cargo_bin("arrow")
    .unwrap()
    .args(&["1"])
    .unwrap()
    .assert()
    .success()
    .stdout("*\n");
    Ok(())
}

#[test]
fn arrow_2() -> TestResult {
    Command::cargo_bin("arrow")
        .unwrap()
        .args(&["2"])
        .unwrap()
        .assert()
        .success()
        .stdout("*\n**\n*\n");
    Ok(())
}

#[test]
fn rno_args() -> TestResult {
    Command::cargo_bin("reverse_arrow")
        .unwrap()
        .assert()
        .failure()
        .stderr("Please provide a number as an argument\n");
    Ok(())
}

#[test]
fn rarrow_3() -> TestResult {
    Command::cargo_bin("reverse_arrow")
        .unwrap()
        .args(&["3"])
        .unwrap()
        .assert()
        .success()
        .stdout("  *\n **\n***\n **\n  *\n");
    Ok(())
}
