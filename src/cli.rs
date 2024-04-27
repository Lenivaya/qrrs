pub mod args;

use self::args::OutputFormat;
use crate::cli::args::Arguments;
use crate::qrcode_utils::is_svg_path;

use clap::Parser;
use std::io::{self, Read};
use std::path::Path;

impl Arguments {
    pub fn parse_cli_args() -> Arguments {
        let args = Arguments::parse();
        let input = args
            .input
            .and_then(|parsed_input| match parsed_input.as_str() {
                "-" => Arguments::parse_stdin(),
                _ => Some(parsed_input),
            });

        let output_format = match args.output {
            Some(ref output) if is_svg_path(Path::new(&output)) => OutputFormat::Svg,
            _ => args.output_format,
        };

        Arguments {
            input,
            output_format,
            ..args
        }
    }

    fn parse_stdin() -> Option<String> {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf).ok();
        String::from_utf8(buf).ok()
    }
}
