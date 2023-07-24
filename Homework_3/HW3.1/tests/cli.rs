use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("Rust-hw3_1").unwrap();
    cmd.assert().failure();
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("Rust-hw3_1").unwrap();
    cmd.arg("100").assert().success().stdout("The grade is Excellent with A+\n");
    Ok(())
}