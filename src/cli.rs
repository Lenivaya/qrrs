pub mod args;

use self::args::OutputFormat;
use crate::cli::args::Arguments;
use crate::qrcode_utils::is_svg_path;

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Generator};
use std::io::{self, Read};
use std::path::Path;

impl Arguments {
    pub fn parse_cli_args() -> Self {
        let mut args = Self::parse();

        args.input = args
            .input
            .and_then(|parsed_input| match parsed_input.as_str() {
                "-" => Arguments::parse_stdin(),
                _ => Some(parsed_input),
            });

        args.output_format = match args.output {
            Some(ref output) if is_svg_path(Path::new(output)) => OutputFormat::Svg,
            _ => args.output_format,
        };

        args
    }

    fn parse_stdin() -> Option<String> {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).ok().map(|_| buffer)
    }

    /// Generates completion for the shell
    pub fn generate_completions(gen: impl Generator) {
        generate(
            gen,
            &mut Self::command(),
            Self::command().get_name(),
            &mut io::stdout(),
        );
    }
}
