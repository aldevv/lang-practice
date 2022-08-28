use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("integration_tests").expect("binary not found");
    cmd.assert().success();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").expect("no binary named true was found");
    cmd.assert().success();
}
