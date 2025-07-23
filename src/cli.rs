use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "Password Strength Checker",
    version = "1.0",
    author = "Nahal",
    about = "Checks password strength or generates one"
)]
pub struct Cli {
    /// Password to check
    #[arg(short, long)]
    pub password: Option<String>,

    /// Generate a password
    #[arg(short, long)]
    pub generate: bool,

    /// Copy generated password to clipboard
    #[arg(long)]
    pub copy: bool,

    /// Length of the generated password
    #[arg(short, long, default_value_t = 16)]
    pub length: usize,
}
