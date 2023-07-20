use assert_cmd::Command;

#[test]
fn test_calc(){
    let mut cmd = Command::cargo_bin("fc").unwrap();
    cmd.arg("32").assert().success().stdout("32 degree Fahrenheit converts to 0 degree Celcius\n");
 
}

#[test]
fn test_calc2(){
    let mut cmd = Command::cargo_bin("cf").unwrap();
    cmd.arg("5").assert().success().stdout("5 degree Celcius converts to 41 degree Fahrenheit\n");
}
