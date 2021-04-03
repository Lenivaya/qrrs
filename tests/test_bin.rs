use qrrs::*;

use assert_cmd::Command;
use predicates::str;

#[test]
fn failures_wiithout_argumnents() -> BoxResult<()> {
    let mut cmd = Command::cargo_bin("qrrs")?;

    cmd.assert()
        .failure()
        .stderr(str::contains("For more information try --help"));

    Ok(())
}

#[test]
fn wrong_arguments() -> BoxResult<()> {
    let mut cmd = Command::cargo_bin("qrrs")?;

    cmd.arg("-r").arg("-d").arg("-something").arg("unexpected");

    cmd.assert()
        .failure()
        .stderr(str::contains("For more information try --help"))
        .stderr(str::contains("wasn't expected"));

    Ok(())
}

#[test]
fn file_doesnt_exits() -> BoxResult<()> {
    let mut cmd = Command::cargo_bin("qrrs")?;

    cmd.arg("-r").arg("/test/file/doesnt/exist/qr.png");

    cmd.assert()
        .failure()
        .stderr(str::contains("No such file or directory"));

    Ok(())
}
