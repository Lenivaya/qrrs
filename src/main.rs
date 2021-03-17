use qrrs::{cli, App};

fn main() {
    let args = cli::Arguments::new();
    let config = args.get_config();

    let app = App::new(config);
    app.run();
}
