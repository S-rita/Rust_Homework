use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn no_arg() -> TestResult {
    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd.arg("").arg("").assert().success().stdout("Player 1: \nPlayer 2: \n");
    Ok(())
}

#[test]
fn one_arg() -> TestResult {
    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd.arg("Mike").arg("").assert().success().stdout("Player 1: Mike\nPlayer 2: \n");
    Ok(())
}

#[test]
fn two_args() -> TestResult {
    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd.arg("Mike").arg("Leo").assert().success().stdout("Player 1: Mike\nPlayer 2: Leo\n");
    Ok(())
}

#[test]
fn three_args() -> TestResult {
    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd.arg("Mike").arg("Leo").arg("Ralph").assert().success().stdout("Player 1: Mike\nPlayer 2: Leo\n");
    Ok(())
}
