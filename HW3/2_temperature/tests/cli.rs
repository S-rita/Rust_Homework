use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("Rust-hw3_2").unwrap();
    cmd.assert().failure();
    Ok(())
}

#[test]
fn one_args() -> TestResult {
    let mut cmd = Command::cargo_bin("Rust-hw3_2").unwrap();
    cmd.arg("0").assert().failure();
    Ok(())
}

#[test]
fn two_args() -> TestResult {
    let mut cmd = Command::cargo_bin("Rust-hw3_2").unwrap();
    cmd.arg("0").arg("200").assert().failure();
    Ok(())
}

#[test]
fn runsy_x() -> TestResult {
    let expected = "Fahr\tCelcius\n0\t-17.8\n20\t-6.7\n40\t4.4\n.....\n160\t71.1\n180\t82.2\n200\t93.3\n";
    let mut cmd = Command::cargo_bin("Rust-hw3_2").unwrap();
    cmd.arg("0").arg("200").arg("20").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn runsx_y() -> TestResult {
    let expected = "Fahr\tCelcius\n300\t148.9\n280\t137.8\n260\t126.7\n.....\n40\t4.4\n20\t-6.7\n0\t-17.8\n";
    let mut cmd = Command::cargo_bin("Rust-hw3_2").unwrap();
    cmd.arg("300").arg("0").arg("20").assert().success().stdout(expected);
    Ok(())
}
