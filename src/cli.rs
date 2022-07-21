use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]
#[clap(name = "qrrs", author, about, version)]
pub struct Arguments {
    /// Input data
    #[clap(
        name = "INPUT",
        // parse(from_os_str),
        value_hint = ValueHint::AnyPath,
        required(true),
        index(1)
    )]
    // pub input: Option<PathBuf>,
    pub input: Option<String>,

    #[clap(
        name = "OUTPUT",
        // parse(from_os_str),
        value_hint = ValueHint::AnyPath,
        required_unless_present_any(["INPUT", "read", "terminal"]),
        index(2)
    )]
    // pub output: Option<PathBuf>,
    pub output: Option<String>,

    /// Reads the qr-code instead of generating it
    #[clap(name = "read", short = 'r', long = "read")]
    pub read: bool,

    /// Displays code in terminal
    #[clap(name = "terminal", short = 't', long = "terminal")]
    pub terminal_output: bool,
    // pub matches: ArgMatches,
}

