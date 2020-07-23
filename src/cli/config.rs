use clap::ArgMatches;

#[derive(Debug)]
pub struct Config<'a> {
    pub input: &'a str,
    pub output: &'a str,
    pub read: bool,
    pub terminal_output: bool,
}

impl<'a> Config<'a> {
    pub fn new(matches: &'a ArgMatches) -> Config<'a> {
        let input = matches.value_of("INPUT").unwrap_or("");
        let output = matches.value_of("OUTPUT").unwrap_or("");
        let mut read = false;
        let mut terminal_output = false;

        match matches.occurrences_of("read") {
            1 => read = true,
            _ => (),
        };

        match matches.occurrences_of("terminal") {
            1 => terminal_output = true,
            _ => (),
        }

        Config {
            input,
            output,
            read,
            terminal_output,
        }
    }
}
