use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

///////////// QUESTION 1 /////////////

#[test]
fn test_q1_1_0() -> TestResult {
    let mut cmd = Command::cargo_bin("q1_1").unwrap();
    cmd.assert().failure();
    Ok(())
}

#[test]
fn test_q1_1_1() -> TestResult {
    let mut cmd = Command::cargo_bin("q1_1").unwrap();
    cmd.arg("1.0").assert().failure();
    Ok(())
}

#[test]
fn test_q1_1() -> TestResult {
    let mut cmd = Command::cargo_bin("q1_1").unwrap();
    cmd.arg("5.0").arg("2.0").arg("7.5").assert().success().stdout("Ascending order: [2.0, 5.0, 7.5]\nDescending order: [7.5, 5.0, 2.0]\n");
    Ok(())
}

#[test]
fn test_q1_2_0() -> TestResult {
    let mut cmd = Command::cargo_bin("q1_2").unwrap();
    cmd.assert().failure();
    Ok(())
}

#[test]
fn test_q1_2_1() -> TestResult {
    let mut cmd = Command::cargo_bin("q1_2").unwrap();
    cmd.arg("1.0").assert().failure();
    Ok(())
}

#[test]
fn test_q1_2() -> TestResult {
    let mut cmd = Command::cargo_bin("q1_2").unwrap();
    cmd.arg("5.0").arg("2.0").arg("7.5").assert().success().stdout("Ascending order: [2.0, 5.0, 7.5]\nDescending order: [7.5, 5.0, 2.0]\n");
    Ok(())
}

///////////// QUESTION 2 /////////////

#[test]
fn test_q2_1_0() -> TestResult {
    let mut cmd = Command::cargo_bin("q2_1").unwrap();
    cmd.assert().failure();
    Ok(())
}

#[test]
fn test_q2_1_1() -> TestResult {
    let mut cmd = Command::cargo_bin("q2_1").unwrap();
    cmd.arg("1.0").assert().failure();
    Ok(())
}

#[test]
fn test_q2_1() -> TestResult {
    let mut cmd = Command::cargo_bin("q2_1").unwrap();
    cmd.args(&["1.0", "2.0", "4.0", "5.0"]).assert().success().stdout("Ascending order: [(1.0, 2.0), (4.0, 5.0)]\nDescending order: [(4.0, 5.0), (1.0, 2.0)]\n");
    Ok(())
}

#[test]
fn test_q2_2_0() -> TestResult {
    let mut cmd = Command::cargo_bin("q2_2").unwrap();
    cmd.assert().failure();
    Ok(())
}

#[test]
fn test_q2_2_1() -> TestResult {
    let mut cmd = Command::cargo_bin("q2_2").unwrap();
    cmd.arg("1.0").assert().failure();
    Ok(())
}

#[test]
fn test_q2_2() -> TestResult {
    let mut cmd = Command::cargo_bin("q2_2").unwrap();
    cmd.args(&["1.0", "2.0", "4.0", "5.0"]).assert().success().stdout("Ascending order: [(1.0, 2.0), (4.0, 5.0)]\nDescending order: [(4.0, 5.0), (1.0, 2.0)]\n");
    Ok(())
}