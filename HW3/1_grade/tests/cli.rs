use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("area").unwrap();
    cmd.assert().failure();
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("area").unwrap();
    cmd.arg("25").assert().success().stdout("The area of a circle: 1963.5\n");
    Ok(())
}