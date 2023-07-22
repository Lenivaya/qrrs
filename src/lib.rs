pub mod cli;
pub mod errors;
pub mod qrcode;

use cli::{Arguments, OutputFormat};
use errors::BoxResult;

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
                output_format: of,
            } => self.save_code(i, o, of),

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
                output_format: of,
            } => self.read_print_save_code(i, o, of),

            // Reads qr code, also saves it to specified output
            Arguments {
                input: Some(i),
                output: Some(o),
                read: true,
                terminal_output: false,
                output_format: of,
                ..
            } => self.read_save_code(i, o, of),

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
                output_format: of,
                ..
            } => self.generate_print_save_code(i, o, of),

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

    fn save_code(&self, input: &str, output: &str, output_format: &OutputFormat) -> BoxResult<()> {
        let code = qrcode::make_code(input)?;
        let file = Path::new(output);
        qrcode::save(file, &code, output_format)
    }

    fn read_code(&self, input: &str) -> BoxResult<()> {
        let file = Path::new(input);
        let data = qrcode::read_data_image(file)?;

        Ok(data
            .into_iter()
            .for_each(|something| println!("{}", something)))
    }

    fn print_code(&self, input: &str) -> BoxResult<()> {
        let file = Path::new(input);
        let data = qrcode::read_data_image(file)?.join(" ");

        let code = qrcode::make_code(&data)?;
        Ok(qrcode::print_code_to_term(&code))
    }

    fn generate_print_code(&self, input: &str) -> BoxResult<()> {
        let code = qrcode::make_code(input)?;
        Ok(qrcode::print_code_to_term(&code))
    }

    fn read_print_save_code(
        &self,
        input: &str,
        output: &str,
        output_format: &OutputFormat,
    ) -> BoxResult<()> {
        let file = Path::new(input);
        let output = Path::new(output);

        let data = qrcode::read_data_image(file)?.join(" ");
        let code = Arc::new(qrcode::make_code(&data)?);

        let print_handle = thread::spawn({
            let code = Arc::clone(&code);
            move || qrcode::print_code_to_term(&code)
        });

        qrcode::save(output, &code, output_format)?;
        print_handle.join().unwrap();

        Ok(())
    }

    fn read_save_code(
        &self,
        input: &str,
        output: &str,
        output_format: &OutputFormat,
    ) -> BoxResult<()> {
        let input = Path::new(&input);
        let output = Path::new(&output);
        let data = Arc::new(qrcode::read_data_image(input)?);

        let print_handle = thread::spawn({
            let data = Arc::clone(&data);
            move || data.iter().for_each(|something| println!("{}", something))
        });

        let data_to_write = data.join("");
        let code = qrcode::make_code(&data_to_write)?;

        qrcode::save(output, &code, output_format)?;
        print_handle.join().unwrap();

        Ok(())
    }

    fn generate_print_save_code(
        &self,
        input: &str,
        output: &str,
        output_format: &OutputFormat,
    ) -> BoxResult<()> {
        let output = Path::new(output);
        let code = Arc::new(qrcode::make_code(input)?);

        let print_handle = thread::spawn({
            let code = Arc::clone(&code);
            move || qrcode::print_code_to_term(&code)
        });

        qrcode::save(output, &code, output_format)?;
        print_handle.join().unwrap();

        Ok(())
    }
}
