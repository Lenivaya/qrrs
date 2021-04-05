use qrrs::cli::*;

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
