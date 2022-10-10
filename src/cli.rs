use clap::{ColorChoice, Parser, ValueHint};

const AFTER_TEXT: &str = "
Examples:
  qrrs \"Some text\"
  qrrs \"Some text\" /tmp/qr.png

  qrrs --read /tmp/qr.png
  Some text

  qrrs --read /tmp/qr.png /tmp/qr1.png
  qrrs --read /tmp/qr1.png
  Some text
";

#[derive(Parser, Debug)]
#[clap(
    name = "qrrs",
    author,
    about,
    version,
    after_help = AFTER_TEXT,
    color = ColorChoice::Always
)]
pub struct Arguments {
    /// Input data
    #[clap(
        name = "INPUT",
        value_hint = ValueHint::AnyPath,
        required(true),
        index(1)
    )]
    pub input: Option<String>,

    /// Output file
    #[clap(
        name = "OUTPUT",
        value_hint = ValueHint::AnyPath,
        required_unless_present_any(["INPUT", "read", "terminal"]),
        index(2)
    )]
    pub output: Option<String>,

    /// Read the qr-code instead of generating it
    #[clap(name = "read", short, long)]
    pub read: bool,

    /// Display code in terminal
    #[clap(name = "terminal", short, long)]
    pub terminal_output: bool,
}
