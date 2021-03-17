pub mod cli;

use cli::Config;

use std::panic;
use std::path::Path;

use image::Luma;
use qrcode::QrCode;
use qrcode::render::unicode;
use rqrr::PreparedImage;

pub struct App<'a> {
    config: Config<'a>,
}

impl<'a> App<'a> {
    pub fn new(config: Config<'a>) -> Self {
        App { config }
    }

    pub fn run(self) {
        // Removing output(especially backtrace) when invoking panic
        panic::set_hook(Box::new(|_| {}));

        match self.config {
            // Saves qr code
            Config {
                input: Some(i),
                output: Some(o),
                read: false,
                terminal_output: false,
            } => {
                let code = App::make_code(i);
                let file = Path::new(o);

                App::save(&file, code)
            }

            // Reads code and shows it in terminal
            Config {
                input: Some(i),
                output: None,
                read: true,
                terminal_output: true,
            } => {
                let file = Path::new(i);
                let data = App::read_code(&file).join(" ");

                let code = App::make_code(&data);
                App::print_code_to_term(code);
            }

            // Reads qr code
            Config {
                input: Some(i),
                read: true,
                terminal_output: false,
                ..
            } => {
                let file = Path::new(i);
                let data = App::read_code(&file);

                for something in data {
                    println!("{}", something)
                }
            }

            // Prints code generated from user input to a terminal
            Config {
                input: Some(i),
                read: false,
                terminal_output: true,
                ..
            } => {
                let code = App::make_code(i);

                App::print_code_to_term(code)
            }

            _ => (),
        }
    }

    fn make_code(data: &str) -> QrCode {
        let code = QrCode::new(data.as_bytes()).unwrap_or_else(|err| {
            eprintln!("Problem creating qr code: {}", err);
            panic!();
        });

        code
    }

    fn read_code(file: &Path) -> Vec<String> {
        let img = image::open(file)
            .unwrap_or_else(|err| {
                eprintln!("Problem opening file: {} ", err);
                panic!();
            })
            .to_luma8();
        let mut img = PreparedImage::prepare(img);
        let grids = img.detect_grids();

        grids
            .into_iter()
            .map(|grid| {
                let (_, content) = grid.decode().unwrap_or_else(|err| {
                    eprintln!("Problem reading data from qr code: {}", err);
                    panic!();
                });

                content
            })
            .collect::<Vec<String>>()
    }

    fn print_code_to_term(code: QrCode) {
        let image = code
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();

        // let string = code
        //     .render::<char>()
        //     .quiet_zone(false)
        //     .module_dimensions(2, 1)
        //     .build();

        println!("\n{}", image);
    }

    fn save(file: &Path, code: QrCode) {
        let image = code.render::<Luma<u8>>().build();
        image.save(file).unwrap_or_else(|err| {
            eprintln!("Problem saving code: {}", err);
            std::fs::remove_file(file).unwrap();
            panic!();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::fs;

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
        let text_from_qr = App::read_code(&path).join(" ");
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
            let text_from_qr = App::read_code(&path).join(" ");
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

                App::save(&path, code);
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

        let _ = App::read_code(&path);
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
            let hello_from_qr = App::read_code(&path).join(" ");

            assert_eq!(*hello, hello_from_qr);
        }

        fs::remove_file(file).unwrap();
    }

}
