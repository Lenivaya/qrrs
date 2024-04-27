use clap::{command, Parser, ValueEnum, ValueHint};

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
#[command(
    name = "qrrs",
    author,
    about,
    version,
    after_help = AFTER_TEXT,
)]
pub struct Arguments {
    /// Input data
    #[arg(
        name = "INPUT",
        value_hint = ValueHint::AnyPath,
        required(true),
        index(1)
    )]
    pub input: Option<String>,

    /// Output file
    #[arg(
        name = "OUTPUT",
        value_hint = ValueHint::AnyPath,
        required_unless_present_any(["INPUT", "read", "terminal"]),
        index(2)
    )]
    pub output: Option<String>,

    /// Read the qrcode instead of generating it
    #[arg(name = "read", short, long)]
    pub read: bool,

    /// Display code in terminal
    #[arg(name = "terminal", short, long)]
    pub terminal_output: bool,

    /// Format in which the qrcode will be saved
    #[arg(
        short('o'),
        long,
        value_enum,
        default_value_t,
        value_name("FORMAT"),
        ignore_case(true)
    )]
    pub output_format: OutputFormat,

    /// Margin applied to qrcode
    #[arg(name = "margin", long, short = 'm', default_value_t = 5)]
    pub margin: u32,

    /// Invert qrcode colors
    #[arg(name = "invert_colors", long, short = 'i')]
    pub invert_colors: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Image,
    Svg,
    Unicode,
}
