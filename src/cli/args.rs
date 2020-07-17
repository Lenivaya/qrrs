use clap::{App, Arg, ArgMatches};

pub struct Arguments<'a> {
    pub matches: ArgMatches<'a>,
}

impl<'a> Arguments<'a> {
    pub fn new() -> Arguments<'a> {
        let matches = App::new("qrrs")
            .about("CLI tool for working with qr-codes")
            .arg(
                Arg::with_name("browser")
                    .short("b")
                    .long("browser")
                    .help("Runs qrrs in browser")
                    .takes_value(false),
            )
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
                    .required_unless("browser")
                    .index(1),
            )
            .arg(
                Arg::with_name("OUTPUT")
                    .help("Output file")
                    .required_unless("browser")
                    .required_unless("read")
                    .index(2),
            )
            .get_matches();

        Arguments { matches }
    }
}
