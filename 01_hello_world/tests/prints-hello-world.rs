use assert_cmd::Command;

#[test]
fn prints_hello_world() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("hello_world").unwrap();
    cmd.assert().stdout("Hello World!\n");
    Ok(())
}
