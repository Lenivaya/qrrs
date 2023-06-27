use qrrs::{cli, App};

fn main() {
    let args = cli::Arguments::parse_cli_args();
    let app = App::new(args);

    app.start();
}
