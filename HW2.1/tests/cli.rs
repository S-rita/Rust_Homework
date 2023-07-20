use assert_cmd::Command;

#[test]
fn runs(){
    let mut cmd = Command::cargo_bin("area").unwrap();
    cmd.arg("25").assert().success().stdout("The area of a circle: 1963.5\n");
 
}