use clap::ArgMatches;

#[derive(Debug)]
pub struct Config<'a> {
    pub input: &'a str,
    pub output: &'a str,
    pub run_in_browser: bool,
    pub read: bool,
}

impl<'a> Config<'a> {
    pub fn new(matches: &'a ArgMatches) -> Config<'a> {
        let input = matches.value_of("INPUT").unwrap_or("");
        let output = matches.value_of("OUTPUT").unwrap_or("");
        let mut run_in_browser = false;
        let mut read = false;

        match matches.occurrences_of("browser") {
            1 => run_in_browser = true,
            _ => (),
        }

        match matches.occurrences_of("read") {
            1 => read = true,
            _ => (),
        };

        Config {
            input,
            output,
            run_in_browser,
            read,
        }
    }
}
