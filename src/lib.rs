pub mod cli;

use cli::Config;

use std::panic;
use std::path::Path;
use std::sync::Arc;
use std::thread;

use image::Luma;
use qrcode::render::unicode;
use qrcode::QrCode;
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

                App::save(&file, &code)
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
                App::print_code_to_term(&code);
            }

            // Reads code and shows it in terminal,
            // also saves to specified output
            Config {
                input: Some(i),
                output: Some(o),
                read: true,
                terminal_output: true,
            } => {
                let file = Path::new(i);
                let output = Path::new(o);
                let data = App::read_code(&file).join(" ");

                let code = Arc::new(App::make_code(&data));
                let codepointer = code.clone();

                let print_handle = thread::spawn(move || {
                    App::print_code_to_term(&code);
                });

                App::save(&output, &codepointer);

                print_handle.join().unwrap();
            }

            // Reads qr code, also saves it to specified output
            Config {
                input: Some(i),
                output: Some(o),
                read: true,
                terminal_output: false,
            } => {
                let input = Path::new(i);
                let output = Path::new(o);

                let data = Arc::new(App::read_code(&input));
                let datapointer = data.clone();

                let print_handle = thread::spawn(move || {
                    for something in data.iter() {
                        println!("{}", something)
                    }
                });

                let data_to_write = datapointer.join("");
                let code = App::make_code(&data_to_write);
                App::save(&output, &code);

                print_handle.join().unwrap();
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

            // Prints code generated from user input to a terminal,
            // also saves it to specified output
            Config {
                input: Some(i),
                output: Some(o),
                read: false,
                terminal_output: true,
            } => {
                let output = Path::new(o);

                let code = Arc::new(App::make_code(i));
                let codepointer = code.clone();

                let print_handle = thread::spawn(move || {
                    App::print_code_to_term(&code);
                });

                App::save(&output, &codepointer);

                print_handle.join().unwrap();
            }

            // Prints code generated from user input to a terminal
            Config {
                input: Some(i),
                read: false,
                terminal_output: true,
                ..
            } => {
                let code = App::make_code(i);

                App::print_code_to_term(&code)
            }

            _ => unreachable!(),
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
        if !file.exists() {
            eprintln!(
                "Error opening file: {:?} \nNo such file or directory",
                file
            );
            panic!();
        }

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

    fn print_code_to_term(code: &QrCode) {
        let image = code
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();

        println!("\n{}", image);
    }

    fn save(file: &Path, code: &QrCode) {
        let image = code.render::<Luma<u8>>().build();
        image.save(file).unwrap_or_else(|err| {
            eprintln!("Problem saving code: {}", err);
            std::fs::remove_file(file).unwrap();
            panic!();
        });
    }
}

