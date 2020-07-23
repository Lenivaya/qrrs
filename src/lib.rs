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
        let code = App::make_code(self.config.input);

        if self.config.read == true {
            let file = PathBuf::from_str(self.config.input).unwrap();
            let data = App::read_code(&file);

            for something in data {
                println!("{}", something)
            }
        } else if self.config.terminal_output == true {
            App::print_code_to_term(code)
        } else {
            let file = PathBuf::from_str(self.config.output).unwrap();
            App::save(&file, code)
        }
    }

    fn make_code(data: &str) -> QrCode {
        let code = QrCode::new(data.as_bytes()).unwrap();
        code
    }

    fn read_code(file: &PathBuf) -> Vec<String> {
        let img = image::open(file).unwrap();
        let decoder = bardecoder::default_decoder();

        let results = decoder.decode(&img);
        let unwrapped_results = results
            .into_iter()
            .map(|result| result.unwrap())
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
        image.save(file).unwrap();
    }
}
