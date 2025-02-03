use std::{fs, path::Path};

use qrcode::EcLevel;
use qrrs::{
    cli::args::{Arguments, CliEcLevel, OutputFormat},
    errors::BoxResult,
    qrcode_utils::ImageSaveArguments,
};
use test_common::*;

mod test_common;

#[test]
fn makes_code_with_random_text_for_different_extensions() -> BoxResult<()> {
    let supported_extensions = [
        "png", "jpeg", "bmp", "tiff", "tga", "gif", "webp", "ico", "svg", "qoi", "pbm", "pgm",
        "ppm",
    ];

    for extension in supported_extensions {
        let text: String = random_text();
        let file = Path::new("qr_tmp_random").with_extension(extension);
        let output_format = match extension {
            "svg" => OutputFormat::Svg,
            _ => OutputFormat::Image,
        };

        let config = Arguments {
            input: Some(text.to_string()),
            output: Some(file.to_string_lossy().to_string()),
            read: false,
            terminal_output: false,
            output_format,
            margin: 1,
            invert_colors: false,
            error_correction_level: CliEcLevel::Medium,
            generate_completions: None,
        };
        let app = App::new(config);
        app.start();

        let path = Path::new(&file);
        let text_from_qr = qrcode_utils::read_data_from_image(path)?.join(" ");
        fs::remove_file(file)?;

        assert_eq!(text, text_from_qr);
    }

    Ok(())
}

#[test]
#[should_panic]
fn save_in_unsupported_extension() {
    let unsupported_extensions = [
        "txt", "mp3", "iso", "pdf", "zip", "html", "js", "rs", "py", "docx", "el", "ex", "css",
        "ts", "tar.gz", "go", "tex", "scss",
    ];

    for extension in unsupported_extensions {
        let path = Path::new("file").with_extension(extension);
        let code = qrcode_utils::make_code("QRrs", EcLevel::M).unwrap();

        qrcode_utils::save(
            &path,
            &code,
            ImageSaveArguments {
                output_format: &OutputFormat::Image,
                view_arguments: qrcode_utils::QrCodeViewArguments {
                    margin: 1,
                    invert_colors: false,
                },
            },
        )
        .unwrap();
    }
}

#[test]
#[should_panic]
fn read_non_existent_file() {
    let file: String = random_text();
    let path = Path::new(&file);

    let _ = qrcode_utils::read_data_from_image(path).unwrap();
}

#[test]
fn different_languages_support() -> BoxResult<()> {
    let hellos = [
        "Dobrý den",
        "السلام عليكم",
        "שָׁלוֹם",
        "你好",
        "नमस्ते",
        "こんにちは",
        "안녕하세요",
        "Olá",
        "Hola",
        "Привіт",
        "Bonjour",
        "Здравствуйте",
    ];
    let file = "qr_tmp.png";

    for hello in hellos {
        let config = Arguments {
            input: Some(hello.to_string()),
            output: Some(file.to_string()),
            read: false,
            terminal_output: false,
            output_format: OutputFormat::Image,
            margin: 1,
            invert_colors: false,
            error_correction_level: CliEcLevel::Medium,
            generate_completions: None,
        };
        let app = App::new(config);
        app.start();

        let path = Path::new(file);
        let hello_from_qr = qrcode_utils::read_data_from_image(path)?.join(" ");

        assert_eq!(*hello, hello_from_qr);
    }

    fs::remove_file(file)?;

    Ok(())
}
