use clap::{crate_authors, crate_version};
use clap::{App, AppSettings, Arg, ArgMatches};

#[derive(Debug)]
pub struct Config<'a> {
    pub input: Option<&'a str>,
    pub output: Option<&'a str>,
    pub read: bool,
    pub terminal_output: bool,
}

#[derive(Debug, Default)]
pub struct Arguments {
    pub matches: ArgMatches,
}

impl<'a> Arguments {
    #[cfg(not(tarpaulin_include))]
    pub fn new() -> Self {
        let cli = Arguments::gen_cli();
        let matches = cli.get_matches();

        Arguments { matches }
    }

    pub fn gen_cli() -> App<'a> {
        App::new("qrrs")
            .about("CLI tool for working with qr-codes")
            .version(crate_version!())
            .author(crate_authors!())
            .setting(AppSettings::ColoredHelp)
            .arg(
                Arg::new("INPUT")
                    .about("Input data")
                    .index(1)
                    .required(true),
            )
            .arg(
                Arg::new("OUTPUT")
                    .about("Output file")
                    .index(2)
                    .required_unless_present("read")
                    .required_unless_present("terminal"),
            )
            .arg(
                Arg::new("read")
                    .short('r')
                    .about("Reads the qr-code instead of generating it")
                    .long("read")
                    .takes_value(false),
            )
            .arg(
                Arg::new("terminal")
                    .short('t')
                    .about("Displays code in terminal")
                    .long("terminal")
                    .takes_value(false),
            )
    }

    pub fn get_config(&'a self) -> Config<'a> {
        Config {
            input: self.matches.value_of("INPUT"),
            output: self.matches.value_of("OUTPUT"),
            read: self.matches.is_present("read"),
            terminal_output: self.matches.is_present("terminal"),
        }
    }
}
