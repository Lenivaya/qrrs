#[cfg(test)]
use qrrs::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::{fs, panic, path::Path};

#[test]
fn make_code() {
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    let file = "qr_tmp_lorem.png";

    let config = cli::Config {
        input: Some(&text),
        output: Some(file),
        read: false,
        terminal_output: false,
    };
    let app = App::new(config);
    app.run();

    let path = Path::new(file);
    let text_from_qr = App::read(&path).join(" ");
    fs::remove_file(file).unwrap();

    assert_eq!(text, text_from_qr);
}

#[test]
fn make_code_with_random_text() {
    for _ in 0..10 {
        let text: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        let file = "qr_tmp_random.png";

        let config = cli::Config {
            input: Some(&text),
            output: Some(file),
            read: false,
            terminal_output: false,
        };
        let app = App::new(config);
        app.run();

        let path = Path::new(file);
        let text_from_qr = App::read(&path).join(" ");
        fs::remove_file(file).unwrap();

        assert_eq!(text, text_from_qr);
    }
}

#[test]
fn save_in_unsuported_extesion() {
    let unsuported_extensions =
        ["txt", "svg", "mp3", "iso", "pdf", "zip", "html", "js"];

    for ext in &unsuported_extensions {
        let res = panic::catch_unwind(|| {
            let path = Path::new("file").with_extension(ext);
            let code = App::make_code("QRrs");

            App::save(&path, &code);
        });
        assert!(res.is_err());
    }
}

#[test]
#[should_panic]
fn read_non_existent_file() {
    let file: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(40)
        .map(char::from)
        .collect();
    let path = Path::new(&file);

    let _ = App::read(&path);
}

#[test]
fn different_languages_support() {
    let hellos = [
        "Dobrý den",
        "नमस्ते",
        "こんにちは",
        "안녕하세요",
        "Здравствуйте",
    ];
    let file = "qr_tmp.png";

    for hello in hellos.iter() {
        let config = cli::Config {
            input: Some(hello),
            output: Some(file),
            read: false,
            terminal_output: false,
        };
        let app = App::new(config);
        app.run();

        let path = Path::new(file);
        let hello_from_qr = App::read(&path).join(" ");

        assert_eq!(*hello, hello_from_qr);
    }

    fs::remove_file(file).unwrap();
}
