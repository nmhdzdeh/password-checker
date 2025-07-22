use clap::{Arg, ArgAction, Command};

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
        .arg(
            Arg::new("generate")
                .short('g')
                .long("generate")
                .help("Generate a secure random password")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("length")
                .long("length")
                .help("Length of the generated password (default: 16)")
                .value_name("LENGTH")
                .num_args(1)
                .default_value("16")
                .requires("generate"),
        )
        .after_help("Examples:\n  --password mypass123\n  --generate --length 24")
        .get_matches()
        
}