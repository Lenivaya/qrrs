use assert_cmd::Command;
use predicates::{prelude::*, str};
use std::{fs, path::Path};

mod test_common;
use test_common::*;

#[test]
fn failures_wiithout_argumnents() -> BoxResult<()> {
    let mut cmd = Command::cargo_bin("qrrs")?;

    cmd.assert()
        .failure()
        .stderr(str::contains("Usage:"))
        .stderr(str::contains("For more information try '--help'"));

    Ok(())
}

#[test]
fn wrong_arguments() -> BoxResult<()> {
    let mut cmd = Command::cargo_bin("qrrs")?;

    cmd.arg("-r").arg("-d").arg("-something").arg("unexpected");

    cmd.assert()
        .failure()
        .stderr(str::contains("For more information try '--help'"))
        .stderr(str::contains("wasn't expected"))
        .stderr(str::contains("Usage:"));

    Ok(())
}

#[test]
fn reads_qr_code() -> BoxResult<()> {
    let file = "qr_random_text.png";
    let path = Path::new(file);
    let text: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let code = App::make_code(&text)?;
    App::save(path, &code)?;

    let mut cmd = Command::cargo_bin("qrrs")?;

    cmd.arg("-r").arg(file);

    cmd.assert().success().stdout(str::contains(text));

    fs::remove_file(file)?;
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
