use clap::{App, Arg, ArgMatches};

pub struct Arguments<'a> {
    pub matches: ArgMatches<'a>,
}

impl<'a> Arguments<'a> {
    pub fn new() -> Arguments<'a> {
        let matches = App::new("qrrs")
            .about("CLI tool for working with qr-codes")
            .version("0.1.0")
            .arg(
                Arg::with_name("read")
                    .short("r")
                    .help("Reads the qr-code instead of generating it")
                    .long("read")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("INPUT")
                    .help("Input data")
                    .index(1)
                    .required(true),
            )
            .arg(
                Arg::with_name("OUTPUT")
                    .help("Output file")
                    .index(2)
                    .required_unless("read")
                    .required_unless("terminal"),
            )
            .arg(
                Arg::with_name("terminal")
                    .short("t")
                    .help("Displays code in terminal")
                    .long("terminal")
                    .takes_value(false),
            )
            .get_matches();

        Arguments { matches }
    }
}
