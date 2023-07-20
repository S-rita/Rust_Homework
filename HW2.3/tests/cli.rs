use assert_cmd::Command;

#[test]
fn runs()  {
    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd.arg("Mike").arg("Leo").assert().success().stdout("Player 1: Mike\nPlayer 2: Leo\n");
}
