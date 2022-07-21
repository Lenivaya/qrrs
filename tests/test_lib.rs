use std::{fs, path::Path};

mod test_common;
use test_common::*;

#[test]
fn make_code() -> BoxResult<()> {
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia desestartt mollit anim id est laborum.";
    let file = "qr_tmp_lorem.png";

    let config = cli::Arguments {
        input: Some(text.to_string()),
        output: Some(file.to_string()),
        read: false,
        terminal_output: false,
    };
    let app = App::new(config);
    app.start();

    let path = Path::new(file);
    let text_from_qr = App::read(&path)?.join(" ");
    fs::remove_file(file)?;

    assert_eq!(text, text_from_qr);

    Ok(())
}

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
        };
        let app = App::new(config);
        app.start();

        let path = Path::new(file);
        let text_from_qr = App::read(&path)?.join(" ");
        fs::remove_file(file)?;

        assert_eq!(text, text_from_qr);
    }

    Ok(())
}

#[test]
#[should_panic]
fn save_in_unsuported_extesion() {
    let unsupported_extensions = [
        "txt", "svg", "mp3", "iso", "pdf", "zip", "html", "js", "rs", "py",
        "docx", "el", "ex", "css", "ts", "tar.gz", "go", "tex", "scss",
    ];

    for ext in &unsupported_extensions {
        let path = Path::new("file").with_extension(ext);
        let code = App::make_code("QRrs").unwrap();

        App::save(&path, &code).unwrap();
    }
}

#[test]
#[should_panic]
fn read_non_existent_file() {
    let file: String = random_text();
    let path = Path::new(&file);

    let _ = App::read(&path).unwrap();
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

    for hello in hellos.iter() {
        let config = cli::Arguments {
            input: Some(hello.to_string()),
            output: Some(file.to_string()),
            read: false,
            terminal_output: false,
        };
        let app = App::new(config);
        app.start();

        let path = Path::new(file);
        let hello_from_qr = App::read(&path)?.join(" ");

        assert_eq!(*hello, hello_from_qr);
    }

    fs::remove_file(file)?;

    Ok(())
}
