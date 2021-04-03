use qrrs::*;

use assert_cmd::Command;
use predicates::{prelude::*, str};

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

    let wrong_path_unix = str::contains("No such file or directory");
    let wrong_path_windows =
        str::contains("The system cannot find the path specified");

    cmd.arg("-r").arg("/test/file/doesnt/exist/qr.png");

    cmd.assert()
        .failure()
        .stderr(wrong_path_unix.or(wrong_path_windows));

    Ok(())
}
