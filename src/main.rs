use ansi_term::Colour::Red;
use clap::{App, Arg};
use log::debug;
use tidy::run;

fn main() {
    let matches = App::new("dately")
        .version("1.0")
        .author("stefan hengl")
        .about("Same date format for all filenames")
        .arg(
            Arg::new("source")
                .about("Describes the format you want to change")
                .value_name("SOURCE")
                .takes_value(true)
                .short('s')
                .long("source")
                .required(true),
        )
        .arg(
            Arg::new("target")
                .about("Describes the target format")
                .value_name("TARGET")
                .takes_value(true)
                .short('t')
                .long("target")
                .env("ORDERLY_FORMAT")
                .required(true),
        )
        .arg(
            Arg::new("dir")
                .about("The root of the directory that should be processed")
                .value_name("DIR")
                .short('d')
                .long("dir")
                .takes_value(true),
        )
        .arg(Arg::new("sim")
             .about("Simulates the changes. Prints the changes to Stdout without acutally renaming files")
             .long("sim")
             .takes_value(false)
        )
        .arg(
            Arg::new("ignore")
                .about("Ignore errors. Skips files which cannot be processed")
                .takes_value(false)
                .short('i')
                .long("ignore")
        )
        .arg(
            Arg::new("review")
                .about("Review changes one by one")
                .takes_value(false)
                .short('r')
                .long("review")
        )
        .get_matches();

    debug!(
        "source: {}; target: {}; dir {}; sim {}; ignore {};",
        &matches.value_of("source").unwrap(),
        &matches.value_of("target").unwrap(),
        &matches.value_of("dir").unwrap_or("."),
        &matches.is_present("sim"),
        &matches.is_present("ignore")
    );

    let opt = tidy::RunOpt {
        simulate: matches.is_present("sim"),
        force: matches.is_present("ignore"),
        review: matches.is_present("review"),
    };

    match run(
        &matches.value_of("source").unwrap(),
        &matches.value_of("target").unwrap(),
        &matches.value_of("dir").unwrap_or("."),
        opt,
    ) {
        Ok(_) => println!("OK"),
        Err(e) => println!("{}\n", Red.paint(format!("ERR: {}", e.to_string()))),
    }
}
