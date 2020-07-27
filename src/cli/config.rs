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
