use assert_cmd::Command;

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").expect("no binary named true was found");
    cmd.assert().success();
}
