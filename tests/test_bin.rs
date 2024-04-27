use assert_cmd::Command;
use predicates::{prelude::*, str};
use qrrs::{cli::args::OutputFormat, errors::BoxResult, qrcode_utils::ImageSaveArguments};
use std::{fs, path::Path};

mod test_common;
use test_common::*;

#[test]
fn failures_without_arguments() -> BoxResult<()> {
    let mut cmd = Command::cargo_bin("qrrs")?;

    cmd.assert()
        .failure()
        .stderr(str::contains("Usage:"))
        .stderr(str::contains("For more information, try '--help'."));

    Ok(())
}

#[test]
fn wrong_arguments() -> BoxResult<()> {
    let mut cmd = Command::cargo_bin("qrrs")?;

    cmd.arg("-r").arg("-d").arg("-something").arg("unexpected");

    cmd.assert()
        .failure()
        .stderr(str::contains("error: unexpected argument '-d' found"))
        .stderr(str::contains("Usage:"));

    Ok(())
}

#[test]
fn creates_reads_qr_code() -> BoxResult<()> {
    let supported_extensions = ["png", "jpeg", "bmp", "tiff", "tga"];

    for ext in supported_extensions {
        if let Some(path) = Path::new("qr_random_text").with_extension(ext).to_str() {
            create_read_code(path)?;
        };
    }

    Ok(())
}

#[test]
fn file_doesnt_exits() -> BoxResult<()> {
    let mut cmd = Command::cargo_bin("qrrs")?;

    let wrong_path_unix = str::contains("No such file or directory");
    let wrong_path_windows = str::contains("The system cannot find the path specified");

    cmd.arg("-r").arg("/test/file/doesnt/exist/qr.png");

    cmd.assert()
        .failure()
        .stderr(wrong_path_unix.or(wrong_path_windows));

    Ok(())
}

fn create_read_code(file_path: &str) -> BoxResult<()> {
    let path = Path::new(file_path);
    let text: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let code = qrcode_utils::make_code(&text)?;
    qrcode_utils::save(
        path,
        &code,
        ImageSaveArguments {
            output_format: &OutputFormat::Image,
            view_arguments: qrcode_utils::QrCodeViewArguments {
                margin: 1,
                invert_colors: false,
            },
        },
    )?;

    let mut cmd = Command::cargo_bin("qrrs")?;
    cmd.arg("-r").arg(file_path);
    cmd.assert().success().stdout(str::contains(text));

    fs::remove_file(file_path)?;
    Ok(())
}
