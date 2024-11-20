use clap::{App, Arg};

fn main() {
    // When using an underscore, we tell Rust that we'll use the variable
    // but not right now.
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Carlinux <myEmail@gmail.com>")
        .about("Echo command made in rust :D")
        .arg(
            Arg::with_name("Text")
                .value_name("Text")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let text: Vec<String> = matches.values_of_lossy("Text").unwrap();
    let omit_newline: bool = matches.is_present("omit_newline");

    // This is not a Rustic way to write this
    //let mut ending: &str = "\n";
    //if omit_newline {
    //    ending = "";
    //}

    // Rustic way, since IF is an expression not a statement
    //let ending: &str = if omit_newline { "" } else { "\n" };

    // But since we only use 'ending' in one way, we can do this instead
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
