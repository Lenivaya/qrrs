use qrrs::{cli, App};

fn main() {
    let args = cli::args::Arguments::new();
    let config = cli::config::Config::new(&args.matches);

    let app = App::new(config);
    app.run();
}
