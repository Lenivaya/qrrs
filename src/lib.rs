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
    pub fn new(config: Config) -> App {
        App { config }
    }

    pub fn run(self) {
        match self.config.run_in_browser {
            true => println!("Running in browser..."),
            false => match self.config.read {
                true => {
                    let file = PathBuf::from_str(self.config.input).unwrap();
                    let data = App::read_code(&file);
                    for something in data {
                        println!("{}", something)
                    }
                }
                false => {
                    let code = App::make_code(self.config.input);
                    let image = code.render::<Luma<u8>>().build();
                    image.save(self.config.output).unwrap();
                }
            },
        }
    }

    pub fn make_code(data: &str) -> QrCode {
        let code = QrCode::new(data.as_bytes()).unwrap();
        code
    }

    pub fn read_code(file: &PathBuf) -> Vec<String> {
        let img = image::open(file).unwrap();
        let decoder = bardecoder::default_decoder();

        let results = decoder.decode(&img);
        let unwrapped_results = results
            .into_iter()
            .map(|result| result.unwrap())
            .collect::<Vec<String>>();

        unwrapped_results
    }
}
