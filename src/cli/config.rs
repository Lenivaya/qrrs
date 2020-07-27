use clap::ArgMatches;

#[derive(Debug)]
pub struct Config<'a> {
    pub input: Option<&'a str>,
    pub output: Option<&'a str>,
    pub read: bool,
    pub terminal_output: bool,
}

impl<'a> Config<'a> {
    pub fn new(matches: &'a ArgMatches) -> Self {
        let input = match matches.value_of("INPUT") {
            None => None,
            Some(i) => Some(i),
        };

        let output = match matches.value_of("OUTPUT") {
            None => None,
            Some(o) => Some(o),
        };

        let mut read = false;
        let mut terminal_output = false;

        if matches.is_present("read") {
            read = true
        }

        if matches.is_present("terminal") {
            terminal_output = true
        }

        Config {
            input,
            output,
            read,
            terminal_output,
        }
    }
}
