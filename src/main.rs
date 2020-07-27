use qrrs::cli::{args, config};
use qrrs::App;

fn main() {
    let args = args::Arguments::new();
    let config = config::Config::new(&args.matches);

    let app = App::new(config);
    app.run();
}
