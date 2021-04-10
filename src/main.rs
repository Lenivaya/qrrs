use qrrs::{cli, App};

#[cfg(not(tarpaulin_include))]
fn main() {
    let args = cli::Arguments::new();
    let config = args.get_config();

    let app = App::new(config);
    app.start();
}
