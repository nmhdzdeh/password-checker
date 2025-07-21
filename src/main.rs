use rpassword::read_password;
use std::io::{self, Write};

#[derive(Debug, PartialEq)]
enum PasswordStrength {
    TooShort,
    Weak,
    Medium,
    Strong,
}

fn check_password(password: &str) -> PasswordStrength {
    if password.len() < 8 {
        return PasswordStrength::TooShort;
    }

    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_digit = false;
    let mut has_symbol = false;

    for ch in password.chars() {
        if ch.is_lowercase() {
            has_lower = true;
        } else if ch.is_uppercase() {
            has_upper = true;
        } else if ch.is_ascii_digit() {
            has_digit = true;
        } else {
            has_symbol = true;
        }
    }

    let types_count = has_lower as u8 + has_upper as u8 + has_digit as u8 + has_symbol as u8;

    match types_count {
        0 | 1 => PasswordStrength::Weak,
        2 | 3 => PasswordStrength::Medium,
        4 => PasswordStrength::Strong,
        _ => PasswordStrength::Weak,
    }
}

fn main() {
    print!("Please enter your password: ");
    io::stdout().flush().unwrap();

    let password = read_password().expect("Fail to read password");

    let strength = check_password(&password);

    let message = match strength {
        PasswordStrength::TooShort => "Password is too short",
        PasswordStrength::Weak => "Password is weak",
        PasswordStrength::Medium => "Password is medium",
        PasswordStrength::Strong => "Password is strong",
    };

    println!("{}", message);
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
}
