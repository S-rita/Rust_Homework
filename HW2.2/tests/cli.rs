use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_no_args_cf() -> TestResult {
    let mut cmd = Command::cargo_bin("cf").unwrap();
    cmd.assert().failure();
    Ok(())
}

#[test]
fn test_cf() -> TestResult {
    let mut cmd = Command::cargo_bin("cf").unwrap();
    cmd.arg("5").assert().success().stdout("5 degree Celcius converts to 41 degree Fahrenheit\n");
    Ok(())
}

#[test]
fn test_no_args_fc() -> TestResult {
    let mut cmd = Command::cargo_bin("fc").unwrap();
    cmd.assert().failure();
    Ok(())
}

#[test]
fn test_fc() -> TestResult{
    let mut cmd = Command::cargo_bin("fc").unwrap();
    cmd.arg("32").assert().success().stdout("32 degree Fahrenheit converts to 0 degree Celcius\n");
    Ok(())
}

