use clap::{Command, CommandFactory};
use clap_complete::{
    generate_to,
    Shell::{Bash, Fish, PowerShell, Zsh},
};
use clap_mangen::Man;

use roff::{line_break, roman, Roff};

use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};

include!("src/cli.rs");

type Res = Result<(), Box<dyn Error>>;

fn main() -> Res {
    let mut cli = Arguments::command();

    let profile = env::var("PROFILE")?;
    if let "release" = profile.as_str() {
        let outdir_completions = PathBuf::from("./completions");
        let outdir_manpage = PathBuf::from("./man");

        generate_completions(&mut cli, &outdir_completions)?;
        generate_manpage(cli, &outdir_manpage)?;
    }

    Ok(())
}

fn generate_completions(cli: &mut Command, outdir: &Path) -> Res {
    vec![Bash, Zsh, Fish, PowerShell]
        .into_iter()
        .for_each(|sh| {
            let path = generate_to(sh, cli, "qrrs", outdir);

            println!(
                "cargo:warning=completion file for {:?} is generated: {:?}",
                sh, path
            );
        });

    Ok(())
}

fn generate_manpage(cli: Command, outdir: &Path) -> Res {
    let mut buffer: Vec<u8> = Default::default();

    let man_file = outdir.join("qrrs.1");
    let man = Man::new(cli);
    man.render(&mut buffer)?;
    Roff::new()
        .control("SH", ["EXAMPLES OF USAGE"])
        .text([
            roman("qrrs \"Your input here\""),
            line_break(),
            roman("qrrs \"Something\" /tmp/qr.png "),
            line_break(),
            roman("qrrs -r /tmp/qr.png "),
            line_break(),
        ])
        .to_writer(&mut buffer)?;

    std::fs::write(&man_file, buffer)?;
    println!("cargo:warning=manpage file is generated: {:?}", man_file);

    Ok(())
}
