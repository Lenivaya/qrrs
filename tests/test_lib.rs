use std::{fs, path::Path};

mod test_common;
use qrrs::errors::BoxResult;
use test_common::*;

#[test]
fn make_code_with_random_text() -> BoxResult<()> {
    for _ in 0..10 {
        let text: String = random_text();
        let file = "qr_tmp_random.png";

        let config = cli::Arguments {
            input: Some(text.to_string()),
            output: Some(file.to_string()),
            read: false,
            terminal_output: false,
            output_format: cli::OutputFormat::Image,
        };
        let app = App::new(config);
        app.start();

        let path = Path::new(file);
        let text_from_qr = qrcode::read_data_image(path)?.join(" ");
        fs::remove_file(file)?;

        assert_eq!(text, text_from_qr);
    }

    Ok(())
}

#[test]
#[should_panic]
fn save_in_unsuported_extesion() {
    let unsupported_extensions = [
        "txt", "svg", "mp3", "iso", "pdf", "zip", "html", "js", "rs", "py", "docx", "el", "ex",
        "css", "ts", "tar.gz", "go", "tex", "scss",
    ];

    for ext in unsupported_extensions {
        let path = Path::new("file").with_extension(ext);
        let code = qrcode::make_code("QRrs").unwrap();

        qrcode::save(&path, &code, &cli::OutputFormat::Image).unwrap();
    }
}

#[test]
#[should_panic]
fn read_non_existent_file() {
    let file: String = random_text();
    let path = Path::new(&file);

    let _ = qrcode::read_data_image(path).unwrap();
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
        let config = cli::Arguments {
            input: Some(hello.to_string()),
            output: Some(file.to_string()),
            read: false,
            terminal_output: false,
            output_format: cli::OutputFormat::Image,
        };
        let app = App::new(config);
        app.start();

        let path = Path::new(file);
        let hello_from_qr = qrcode::read_data_image(path)?.join(" ");

        assert_eq!(*hello, hello_from_qr);
    }

    fs::remove_file(file)?;

    Ok(())
}
