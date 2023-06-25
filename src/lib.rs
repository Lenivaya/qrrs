pub mod cli;

use cli::Arguments;

use std::error::Error;
use std::panic;
use std::path::Path;
use std::sync::Arc;
use std::thread;

use image::Luma;
use qrencode::render::unicode;
use qrencode::QrCode;
use rqrr::PreparedImage;

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct App {
    args: Arguments,
}

// Methods
impl App {
    pub fn start(self) {
        // Removing output(especially backtrace) when invoking panic
        panic::set_hook(Box::new(|_| {}));

        if let Err(e) = self.run() {
            eprintln!("\nERROR: {}", e);
            panic!();
        }
    }

    fn run(self) -> BoxResult<()> {
        match &self.args {
            // Saves qr code
            Arguments {
                input: Some(i),
                output: Some(o),
                read: false,
                terminal_output: false,
            } => self.save_code(i, o)?,

            // Reads code and shows it in terminal
            Arguments {
                input: Some(i),
                output: None,
                read: true,
                terminal_output: true,
            } => self.print_code(i)?,

            // Reads code and shows it in terminal,
            // also saves to specified output
            Arguments {
                input: Some(i),
                output: Some(o),
                read: true,
                terminal_output: true,
            } => self.save_print_code(i, o)?,

            // Reads qr code, also saves it to specified output
            Arguments {
                input: Some(i),
                output: Some(o),
                read: true,
                terminal_output: false,
            } => self.save_read_code(i, o)?,

            // Reads qr code
            Arguments {
                input: Some(i),
                read: true,
                terminal_output: false,
                ..
            } => self.read_code(i)?,

            // Prints code generated from user input to a terminal,
            // also saves it to specified output
            Arguments {
                input: Some(i),
                output: Some(o),
                read: false,
                terminal_output: true,
            } => self.save_gen_print_code(i, o)?,

            /*
            Prints code generated from user input to a terminal
            default behaviour with only an input available
            */
            Arguments {
                input: Some(i),
                output: None,
                read: false,
                ..
            } => self.gen_print_code(i)?,

            _ => unreachable!(),
        }

        Ok(())
    }

    fn save_code(&self, input: &str, output: &str) -> BoxResult<()> {
        let code = App::make_code(input)?;
        let file = Path::new(output);
        App::save(file, &code)?;

        Ok(())
    }

    fn read_code(&self, input: &str) -> BoxResult<()> {
        let file = Path::new(input);
        let data = App::read(file)?;

        data.into_iter()
            .for_each(|something| println!("{}", something));

        Ok(())
    }

    fn print_code(&self, input: &str) -> BoxResult<()> {
        let file = Path::new(input);
        let data = App::read(file)?.join(" ");

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
        let data = App::read(file)?.join(" ");

        let code = Arc::new(App::make_code(&data)?);
        let code_pointer = code.clone();

        let print_handle = thread::spawn(move || {
            App::print_code_to_term(&code);
        });

        App::save(output, &code_pointer)?;
        print_handle.join().unwrap();

        Ok(())
    }

    fn save_read_code(&self, input: &str, output: &str) -> BoxResult<()> {
        let input = Path::new(&input);
        let output = Path::new(&output);

        let data = Arc::new(App::read(input)?);
        let data_pointer = data.clone();

        let print_handle = thread::spawn(move || {
            data.iter().for_each(|something| println!("{}", something));
        });

        let data_to_write = data_pointer.join("");
        let code = App::make_code(&data_to_write)?;

        App::save(output, &code)?;
        print_handle.join().unwrap();

        Ok(())
    }

    fn save_gen_print_code(&self, input: &str, output: &str) -> BoxResult<()> {
        let output = Path::new(&output);

        let code = Arc::new(App::make_code(input)?);
        let code_pointer = code.clone();

        let print_handle = thread::spawn(move || {
            App::print_code_to_term(&code);
        });

        App::save(output, &code_pointer)?;
        print_handle.join().unwrap();

        Ok(())
    }
}

// Associated functions
impl App {
    pub fn new(args: Arguments) -> Self {
        App { args }
    }

    pub fn make_code(data: &str) -> BoxResult<QrCode> {
        let code = QrCode::new(data.as_bytes())?;

        Ok(code)
    }

    pub fn read(file: &Path) -> BoxResult<Vec<String>> {
        let img = image::open(file)?.to_luma8();
        let mut prepared_img = PreparedImage::prepare(img);

        let grids = prepared_img.detect_grids();
        let contents: Vec<String> = grids
            .into_iter()
            .map(|grid| {
                grid.decode()
                    .map(|(_, content)| content)
                    .unwrap_or_else(|err| {
                        eprintln!("\nERROR reading data from qr code: {}", err);
                        panic!();
                    })
            })
            .collect();

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
        image.save(file).unwrap_or_else(|err| {
            eprintln!("\nERROR: {}", err);
            std::fs::remove_file(file).unwrap();
            panic!();
        });

        Ok(())
    }
}
