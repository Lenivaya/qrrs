use clap::Parser;
use qrrs::{cli, App};

fn main() {
    let args = cli::Arguments::parse();

    let app = App::new(args);
    app.start();
}
