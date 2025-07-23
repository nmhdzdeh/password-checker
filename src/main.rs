use clap::Parser;
use rpassword::read_password;
use std::io::{self, Write};

mod cli;
mod password;
mod password_generator;
mod ui;
mod clipboard_utils;

use password::{PasswordStrength, check_password};
use ui::colored_message_for;
use clipboard_utils::copy_to_clipboard;

fn handle_password_input(password: &str) {
    let strength = check_password(password);
    println!("Strength: {:?}", strength);
}

fn run_interactive_mode() {
    loop {
        print!("Please enter your password: ");
        io::stdout().flush().unwrap();

        let password = read_password().expect("Failed to read password");
        let strength = check_password(&password);

        println!("{}", colored_message_for(&strength));

        match strength {
            PasswordStrength::TooShort | PasswordStrength::Weak => {
                println!("Please try again.\n");
            }
            _ => break,
        }
    }
}

fn main() {
    let args = cli::Cli::parse();

     if args.generate {
        let password = password_generator::generate_password(args.length);
        println!("🔐 Generated password: {}", password);

        if args.copy {
            if let Err(e) = copy_to_clipboard(&password) {
                eprintln!("❌ Failed to copy to clipboard: {}", e);
            } else {
                println!("📋 Password copied to clipboard!");
            }
        }

        return;
    }

    if let Some(pass) = args.password {
        handle_password_input(&pass);

        if args.copy {
            if let Err(e) = copy_to_clipboard(&pass) {
                eprintln!("❌ Failed to copy to clipboard: {}", e);
            } else {
                println!("📋 Password copied to clipboard!");
            }
        }
    } else {
        run_interactive_mode();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_too_short() {
        assert_eq!(check_password("Ab1!"), PasswordStrength::TooShort);
    }

    #[test]
    fn test_weak() {
        assert_eq!(check_password("abcdefgh"), PasswordStrength::Weak);
    }

    #[test]
    fn test_medium_two_types() {
        assert_eq!(check_password("abcd1234"), PasswordStrength::Medium);
    }

    #[test]
    fn test_medium_three_types() {
        assert_eq!(check_password("Abcd1234"), PasswordStrength::Medium);
    }

    #[test]
    fn test_strong() {
        assert_eq!(check_password("Abcd123!"), PasswordStrength::Strong);
    }

    #[test]
    fn test_only_symbols() {
        assert_eq!(check_password("!@#$%^&*"), PasswordStrength::Weak);
    }

    #[test]
    fn test_only_uppercase() {
        assert_eq!(check_password("ABCDEFGH"), PasswordStrength::Weak);
    }

    #[test]
    fn test_only_digits() {
        assert_eq!(check_password("12345678"), PasswordStrength::Weak);
    }
}
