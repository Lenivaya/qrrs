pub mod cli;

use cli::Config;

use std::error::Error;
use std::panic;
use std::path::Path;
use std::sync::Arc;
use std::thread;

use image::Luma;
use qrcode::render::unicode;
use qrcode::QrCode;
use rqrr::PreparedImage;

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

pub struct App<'a> {
    config: Config<'a>,
}

// Methods
impl<'a> App<'a> {
    pub fn start(self) {
        // Removing output(especially backtrace) when invoking panic
        panic::set_hook(Box::new(|_| {}));

        if let Err(e) = self.run() {
            eprintln!("\nERROR: {}", e);
            panic!();
        }
    }

    fn run(&self) -> BoxResult<()> {
        match self.config {
            // Saves qr code
            Config {
                input: Some(i),
                output: Some(o),
                read: false,
                terminal_output: false,
            } => self.save_code(i, o)?,

            // Reads code and shows it in terminal
            Config {
                input: Some(i),
                output: None,
                read: true,
                terminal_output: true,
            } => self.print_code(i)?,

            // Reads code and shows it in terminal,
            // also saves to specified output
            Config {
                input: Some(i),
                output: Some(o),
                read: true,
                terminal_output: true,
            } => self.save_print_code(i, o)?,

            // Reads qr code, also saves it to specified output
            Config {
                input: Some(i),
                output: Some(o),
                read: true,
                terminal_output: false,
            } => self.save_read_code(i, o)?,

            // Reads qr code
            Config {
                input: Some(i),
                read: true,
                terminal_output: false,
                ..
            } => self.read_code(i)?,

            // Prints code generated from user input to a terminal,
            // also saves it to specified output
            Config {
                input: Some(i),
                output: Some(o),
                read: false,
                terminal_output: true,
            } => self.save_gen_print_code(i, o)?,

            // Prints code generated from user input to a terminal
            Config {
                input: Some(i),
                read: false,
                terminal_output: true,
                ..
            } => self.gen_print_code(i)?,

            _ => unreachable!(),
        }

        Ok(())
    }

    fn save_code(&self, input: &str, output: &str) -> BoxResult<()> {
        let code = App::make_code(input)?;
        let file = Path::new(output);
        App::save(&file, &code)?;

        Ok(())
    }

    fn read_code(&self, input: &str) -> BoxResult<()> {
        let file = Path::new(input);
        let data = App::read(&file)?;

        for something in data {
            println!("{}", something)
        }

        Ok(())
    }

    fn print_code(&self, input: &str) -> BoxResult<()> {
        let file = Path::new(input);
        let data = App::read(&file)?.join(" ");

        let code = App::make_code(&data)?;
        App::print_code_to_term(&code);

        Ok(())
    }

    fn gen_print_code(&self, input: &str) -> BoxResult<()> {
        let code = App::make_code(input)?;
        App::print_code_to_term(&code);

        Ok(())
    }

    fn save_print_code(&self, input: &str, output: &str) -> BoxResult<()> {
        let file = Path::new(input);
        let output = Path::new(output);
        let data = App::read(&file)?.join(" ");

        let code = Arc::new(App::make_code(&data)?);
        let codepointer = code.clone();

        let print_handle = thread::spawn(move || {
            App::print_code_to_term(&code);
        });

        App::save(&output, &codepointer)?;
        print_handle.join().unwrap();

        Ok(())
    }

    fn save_read_code(&self, input: &str, output: &str) -> BoxResult<()> {
        let input = Path::new(input);
        let output = Path::new(output);

        let data = Arc::new(App::read(&input)?);
        let datapointer = data.clone();

        let print_handle = thread::spawn(move || {
            for something in data.iter() {
                println!("{}", something)
            }
        });

        let data_to_write = datapointer.join("");
        let code = App::make_code(&data_to_write)?;

        App::save(&output, &code)?;
        print_handle.join().unwrap();

        Ok(())
    }

    fn save_gen_print_code(&self, input: &str, output: &str) -> BoxResult<()> {
        let output = Path::new(output);

        let code = Arc::new(App::make_code(input)?);
        let codepointer = code.clone();

        let print_handle = thread::spawn(move || {
            App::print_code_to_term(&code);
        });

        App::save(&output, &codepointer)?;
        print_handle.join().unwrap();

        Ok(())
    }
}

// Associated functions
impl<'a> App<'a> {
    pub fn new(config: Config<'a>) -> Self {
        App { config }
    }

    pub fn make_code(data: &str) -> BoxResult<QrCode> {
        let code = QrCode::new(data.as_bytes())?;

        Ok(code)
    }

    pub fn read(file: &Path) -> BoxResult<Vec<String>> {
        let img = image::open(file)?.to_luma8();
        let mut prepared_img = PreparedImage::prepare(img);

        let grids = prepared_img.detect_grids();
        let contents = grids
            .into_iter()
            .map(|grid| {
                let (_, content) = grid.decode().unwrap_or_else(|err| {
                    eprintln!("\nERROR reading data from qr code: {}", err);
                    panic!();
                });

                content
            })
            .collect::<Vec<String>>();

        Ok(contents)
    }

    pub fn print_code_to_term(code: &QrCode) {
        let image = code
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();

        println!("\n{}", image);
    }

    pub fn save(file: &Path, code: &QrCode) -> BoxResult<()> {
        let image = code.render::<Luma<u8>>().build();
        image.save(file)?;

        Ok(())
    }
}
