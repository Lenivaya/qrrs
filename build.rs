use clap::{App, IntoApp, ValueEnum};
use clap_complete::{generate_to, Shell};
use clap_mangen::Man;

use std::{env, io::Error, path::PathBuf};

include!("src/cli.rs");

type Res = Result<(), Error>;

fn main() -> Res {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => PathBuf::from(outdir),
    };
    let mut cli = Arguments::command();

    generate_completions(&mut cli, &outdir)?;
    generate_manpage(cli, &outdir)?;

    Ok(())
}

fn generate_completions(cli: &mut App, outdir: &PathBuf) -> Res {
    ["bash", "zsh", "fish", "powershell"]
        .iter()
        .map(|sh| Shell::from_str(sh, true))
        .filter_map(|sh| sh.ok())
        .for_each(|sh| {
            let path = generate_to(sh, cli, "qrrs", &outdir);

            println!(
                "cargo:warning=completion file for {:?} is generated: {:?}",
                sh, path
            );
        });

    Ok(())
}

fn generate_manpage(cli: App, outdir: &PathBuf) -> Res {
    let mut buffer: Vec<u8> = Default::default();

    let man = Man::new(cli);
    let man_file = outdir.join("qrrs.1");
    man.render(&mut buffer)?;

    std::fs::write(&man_file, buffer)?;
    println!("cargo:warning=manpage file is generated: {:?}", man_file);

    Ok(())
}
