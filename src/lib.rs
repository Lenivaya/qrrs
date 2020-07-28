pub mod cli;

use cli::config::Config;

use std::path::PathBuf;
use std::str::FromStr;

use bardecoder;
use image::Luma;
use qrcode::QrCode;

pub struct App<'a> {
    config: Config<'a>,
}

impl<'a> App<'a> {
    pub fn new(config: Config<'a>) -> Self {
        App { config }
    }

    pub fn run(self) {
        match self.config {
            // Saves qr code
            Config {
                input: Some(i),
                output: Some(o),
                read: false,
                terminal_output: false,
            } => {
                let code = App::make_code(i);
                let file = PathBuf::from_str(o).unwrap();

                App::save(&file, code)
            }

            // Reads code and shows it in terminal
            Config {
                input: Some(i),
                output: None,
                read: true,
                terminal_output: true,
            } => {
                let file = PathBuf::from_str(i).unwrap();
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
                let file = PathBuf::from_str(i).unwrap();
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
        let code = QrCode::new(data.as_bytes())
            .unwrap_or_else(|err| panic!("Problem creating qr code: {}", err));
        code
    }

    fn read_code(file: &PathBuf) -> Vec<String> {
        let img = image::open(file)
            .unwrap_or_else(|err| panic!("Problem opening file: {} ", err));
        let decoder = bardecoder::default_decoder();

        let results = decoder.decode(&img);
        let unwrapped_results = results
            .into_iter()
            .map(|result| {
                result.unwrap_or_else(|err| {
                    panic!("Problem reading data from qr code: {}", err)
                })
            })
            .collect::<Vec<String>>();

        unwrapped_results
    }

    fn print_code_to_term(code: QrCode) {
        // TODO -- Implement this when there are no dependency problems
        // let image = code
        //     .render::<unicode::Dense1x2>()
        //     .dark_color(unicode::Dense1x2::Light)
        //     .light_color(unicode::Dense1x2::Dark)
        //     .build();

        let string = code
            .render::<char>()
            .quiet_zone(false)
            .module_dimensions(2, 1)
            .build();

        println!("\n{}", string);
    }

    fn save(file: &PathBuf, code: QrCode) {
        let image = code.render::<Luma<u8>>().build();
        image
            .save(file)
            .unwrap_or_else(|err| panic!("Problem saving code: {}", err));
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
        let file = "qr_tmp.png";

        let config = cli::config::Config {
            input: Some(text),
            output: Some(file),
            read: false,
            terminal_output: false,
        };
        let app = App::new(config);
        app.run();

        let path = PathBuf::from_str(file).unwrap();
        let text_from_qr = App::read_code(&path).join(" ");
        fs::remove_file(file).unwrap();

        assert_eq!(text, text_from_qr);
    }

    #[test]
    fn make_code_with_random_text() {
        let text: String =
            thread_rng().sample_iter(&Alphanumeric).take(30).collect();
        let file = "qr_tmp_random.png";

        let config = cli::config::Config {
            input: Some(&text),
            output: Some(file),
            read: false,
            terminal_output: false,
        };
        let app = App::new(config);
        app.run();

        let path = PathBuf::from_str(file).unwrap();
        let text_from_qr = App::read_code(&path).join(" ");
        fs::remove_file(file).unwrap();

        assert_eq!(text, text_from_qr);
    }
}
