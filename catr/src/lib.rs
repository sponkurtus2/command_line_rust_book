// A lib is a way to separate our code, here we can define funcions
// structs, etc, and all the functionality will be implemented on
// the main file(binary).
use clap::{App, Arg, ArgMatches};
use std::error::Error;

// It's a good practice to place definitions near the top
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches: ArgMatches = App::new("catr")
        .version("0.1.0")
        .author("Carlinux")
        .about("Cat made in rust")
        // Define parameters here
        .arg(
            Arg::with_name("files")
                .value_name("files")
                .help("Files to process")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .help("Show number of lines")
                .conflicts_with("number_nonblank_lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .help("Show number of non blanck lines")
                .takes_value(false),
        )
        .get_matches();

    let files: Vec<String> = matches
        .values_of("files")
        .unwrap()
        .map(|f| f.to_string())
        .collect(); // Convertir a Vec<String>

    let number_lines: bool = matches.is_present("number_lines");
    let number_nonblank_lines: bool = matches.is_present("number_nonblank_lines");

    let config: Config = Config {
        files,
        number_lines,
        number_nonblank_lines,
    };

    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    // Print configuration
    dbg!(config);
    Ok(())
}
