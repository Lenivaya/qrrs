use clap::App as ClapApp;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use qrrs::cli::*;
use qrrs::BoxResult;

#[test]
fn generates_cli() -> BoxResult<()> {
    let _cli: ClapApp = Arguments::gen_cli();

    Ok(())
}

#[test]
fn generates_right_config() {
    let inp = "/tmp/qr.png";
    let out = "/tmp/qr1.png";
    let args = vec!["qrrs", "-r", "-t", inp, out];

    let cli = Arguments::gen_cli();
    let matches = cli.get_matches_from(args);
    let arguments = Arguments { matches };

    let config = arguments.get_config();
    assert_eq!(
        config,
        Config {
            input: Some(inp),
            output: Some(out),
            read: true,
            terminal_output: true
        }
    )
}

#[test]
fn genertes_with_random_arguments() {
    let inp = random_text();
    let out = random_text();
    let args = vec!["qrrs", "-r", "-t", &inp, &out];

    let cli = Arguments::gen_cli();
    let matches = cli.get_matches_from(args);
    let arguments = Arguments { matches };

    let config = arguments.get_config();
    assert_eq!(
        config,
        Config {
            input: Some(&inp),
            output: Some(&out),
            read: true,
            terminal_output: true
        }
    )
}

fn random_text() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect()
}
