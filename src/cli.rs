use clap::{Arg, Command};

pub fn cli() -> clap::ArgMatches {
    Command::new("Password Checker")
        .version("1.0")
        .about("Checks the strength of a password")
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .value_name("PASSWORD")
                .help("Provide a password directly")
                .num_args(1),
        )
        .get_matches()
}