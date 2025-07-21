enum PasswordStrength {
    TooShort,
    Weak,
    Medium,
    Strong,
}

fn check_password(password: &str) -> PasswordStrength {
    // TODO: implement real logic later
    PasswordStrength::TooShort
}

fn main() {
    println!("Hello, world!");
}
