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
    println!("Hello, world!");
}
