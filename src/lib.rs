pub mod cli;
pub mod errors;
pub mod qrcode_utils;

use cli::args::{Arguments, CliEcLevel};
use errors::BoxResult;
use qrcode::EcLevel;
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
            std::process::exit(1);
        }
    }

    fn run(self) -> BoxResult<()> {
        if let Some(shell) = self.args.generate_completions {
            Arguments::generate_completions(shell);
            return Ok(());
        }

        match &self.args {
            // Saves qr code
            Arguments {
                input: Some(i),
                output: Some(o),
                read: false,
                terminal_output: false,
                error_correction_level: ec,
                ..
            } => self.save_code(i, ec.into(), o),

            // Reads code and shows it in terminal
            Arguments {
                input: Some(i),
                output: None,
                read: true,
                terminal_output: true,
                error_correction_level: ec,
                ..
            } => self.print_code(i, ec.into()),

            /*
            Reads code and shows it in terminal,
            also saves to specified output
            */
            Arguments {
                input: Some(i),
                output: Some(o),
                read: true,
                terminal_output: true,
                error_correction_level: ec,
                ..
            } => self.read_print_save_code(i, ec.into(), o),

            // Reads qr code, also saves it to specified output
            Arguments {
                input: Some(i),
                output: Some(o),
                read: true,
                terminal_output: false,
                error_correction_level: ec,
                ..
            } => self.read_save_code(i, ec.into(), o),

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
                error_correction_level: ec,
                ..
            } => self.generate_print_save_code(i, ec.into(), o),

            /*
            Prints code generated from user input to a terminal
            default behavior with only an input available
            */
            Arguments {
                input: Some(i),
                output: None,
                read: false,
                error_correction_level: ec,
                ..
            } => self.generate_print_code(i, ec.into()),

            _ => Ok(()),
        }
    }

    fn save_code(&self, input: &str, ec_level: EcLevel, output: &str) -> BoxResult<()> {
        let code = qrcode_utils::make_code(input, ec_level)?;
        let file = Path::new(output);
        qrcode_utils::save(file, &code, (&self.args).into())
    }

    fn read_code(&self, input: &str) -> BoxResult<()> {
        let file = Path::new(input);
        let data = qrcode_utils::read_data_from_image(file)?;

        data.into_iter()
            .for_each(|something| println!("{}", something));
        Ok(())
    }

    fn print_code(&self, input: &str, ec_level: EcLevel) -> BoxResult<()> {
        let file = Path::new(input);
        let data = qrcode_utils::read_data_from_image(file)?.join(" ");

        let code = qrcode_utils::make_code(&data, ec_level)?;
        qrcode_utils::print_code_to_term(&code, (&self.args).into());
        Ok(())
    }

    fn generate_print_code(&self, input: &str, ec_level: EcLevel) -> BoxResult<()> {
        let code = qrcode_utils::make_code(input, ec_level)?;
        qrcode_utils::print_code_to_term(&code, (&self.args).into());
        Ok(())
    }

    fn read_print_save_code(&self, input: &str, ec_level: EcLevel, output: &str) -> BoxResult<()> {
        let file = Path::new(input);
        let output = Path::new(output);

        let data = qrcode_utils::read_data_from_image(file)?.join(" ");
        let code = Arc::new(qrcode_utils::make_code(&data, ec_level)?);
        let code_view: QrCodeViewArguments = (&self.args).into();

        let print_handle = thread::spawn({
            let code = Arc::clone(&code);
            move || qrcode_utils::print_code_to_term(&code, code_view)
        });

        qrcode_utils::save(output, &code, (&self.args).into())?;
        print_handle.join().expect("Failed to join threads");

        Ok(())
    }

    fn read_save_code(&self, input: &str, ec_level: EcLevel, output: &str) -> BoxResult<()> {
        let input = Path::new(&input);
        let output = Path::new(&output);
        let data = Arc::new(qrcode_utils::read_data_from_image(input)?);

        let print_handle = thread::spawn({
            let data = Arc::clone(&data);
            move || data.iter().for_each(|something| println!("{}", something))
        });

        let data_to_write = data.join("");
        let code = qrcode_utils::make_code(&data_to_write, ec_level)?;

        qrcode_utils::save(output, &code, (&self.args).into())?;
        print_handle.join().expect("Failed to join threads");

        Ok(())
    }

    fn generate_print_save_code(
        &self,
        input: &str,
        ec_level: EcLevel,
        output: &str,
    ) -> BoxResult<()> {
        let output = Path::new(output);
        let code = Arc::new(qrcode_utils::make_code(input, ec_level)?);
        let code_view: QrCodeViewArguments = (&self.args).into();

        let print_handle = thread::spawn({
            let code = Arc::clone(&code);
            move || qrcode_utils::print_code_to_term(&code, code_view)
        });

        qrcode_utils::save(output, &code, (&self.args).into())?;
        print_handle.join().expect("Failed to join threads");

        Ok(())
    }
}

impl From<&CliEcLevel> for EcLevel {
    fn from(level: &CliEcLevel) -> EcLevel {
        match level {
            CliEcLevel::Low | CliEcLevel::L => EcLevel::L,
            CliEcLevel::Medium | CliEcLevel::M => EcLevel::M,
            CliEcLevel::Quartile | CliEcLevel::Q => EcLevel::Q,
            CliEcLevel::High | CliEcLevel::H => EcLevel::H,
        }
    }
}
