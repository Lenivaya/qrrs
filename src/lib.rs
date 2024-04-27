pub mod cli;
pub mod errors;
pub mod qrcode_utils;

use cli::args::Arguments;
use errors::BoxResult;
use qrcode_utils::QrCodeViewArguments;

use std::panic;
use std::path::Path;
use std::str;
use std::sync::Arc;
use std::thread;

#[derive(Debug)]
pub struct App {
    args: Arguments,
}

impl App {
    pub fn new(args: Arguments) -> Self {
        App { args }
    }

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
                ..
            } => self.save_code(i, o),

            // Reads code and shows it in terminal
            Arguments {
                input: Some(i),
                output: None,
                read: true,
                terminal_output: true,
                ..
            } => self.print_code(i),

            /*
            Reads code and shows it in terminal,
            also saves to specified output
            */
            Arguments {
                input: Some(i),
                output: Some(o),
                read: true,
                terminal_output: true,
                ..
            } => self.read_print_save_code(i, o),

            // Reads qr code, also saves it to specified output
            Arguments {
                input: Some(i),
                output: Some(o),
                read: true,
                terminal_output: false,
                ..
            } => self.read_save_code(i, o),

            // Reads qr code
            Arguments {
                input: Some(i),
                read: true,
                terminal_output: false,
                ..
            } => self.read_code(i),

            /*
            Prints code generated from user input to a terminal,
            also saves it to specified output
            */
            Arguments {
                input: Some(i),
                output: Some(o),
                read: false,
                terminal_output: true,
                ..
            } => self.generate_print_save_code(i, o),

            /*
            Prints code generated from user input to a terminal
            default behavior with only an input available
            */
            Arguments {
                input: Some(i),
                output: None,
                read: false,
                ..
            } => self.generate_print_code(i),

            _ => Ok(()),
        }
    }

    fn save_code(&self, input: &str, output: &str) -> BoxResult<()> {
        let code = qrcode_utils::make_code(input)?;
        let file = Path::new(output);
        qrcode_utils::save(file, &code, (&self.args).into())
    }

    fn read_code(&self, input: &str) -> BoxResult<()> {
        let file = Path::new(input);
        let data = qrcode_utils::read_data_from_image(file)?;

        Ok(data
            .into_iter()
            .for_each(|something| println!("{}", something)))
    }

    fn print_code(&self, input: &str) -> BoxResult<()> {
        let file = Path::new(input);
        let data = qrcode_utils::read_data_from_image(file)?.join(" ");

        let code = qrcode_utils::make_code(&data)?;
        Ok(qrcode_utils::print_code_to_term(&code, (&self.args).into()))
    }

    fn generate_print_code(&self, input: &str) -> BoxResult<()> {
        let code = qrcode_utils::make_code(input)?;
        Ok(qrcode_utils::print_code_to_term(&code, (&self.args).into()))
    }

    fn read_print_save_code(&self, input: &str, output: &str) -> BoxResult<()> {
        let file = Path::new(input);
        let output = Path::new(output);

        let data = qrcode_utils::read_data_from_image(file)?.join(" ");
        let code = Arc::new(qrcode_utils::make_code(&data)?);
        let code_view: QrCodeViewArguments = (&self.args).into();

        let print_handle = thread::spawn({
            let code = Arc::clone(&code);
            move || qrcode_utils::print_code_to_term(&code, code_view)
        });

        qrcode_utils::save(output, &code, (&self.args).into())?;
        print_handle.join().unwrap();

        Ok(())
    }

    fn read_save_code(&self, input: &str, output: &str) -> BoxResult<()> {
        let input = Path::new(&input);
        let output = Path::new(&output);
        let data = Arc::new(qrcode_utils::read_data_from_image(input)?);

        let print_handle = thread::spawn({
            let data = Arc::clone(&data);
            move || data.iter().for_each(|something| println!("{}", something))
        });

        let data_to_write = data.join("");
        let code = qrcode_utils::make_code(&data_to_write)?;

        qrcode_utils::save(output, &code, (&self.args).into())?;
        print_handle.join().unwrap();

        Ok(())
    }

    fn generate_print_save_code(&self, input: &str, output: &str) -> BoxResult<()> {
        let output = Path::new(output);
        let code = Arc::new(qrcode_utils::make_code(input)?);
        let code_view: QrCodeViewArguments = (&self.args).into();

        let print_handle = thread::spawn({
            let code = Arc::clone(&code);
            move || qrcode_utils::print_code_to_term(&code, code_view)
        });

        qrcode_utils::save(output, &code, (&self.args).into())?;
        print_handle.join().unwrap();

        Ok(())
    }
}
