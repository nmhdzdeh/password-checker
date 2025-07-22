use colored::*;
use crate::password::PasswordStrength;

pub fn colored_message_for(strength: &PasswordStrength) -> ColoredString {
    match strength {
        PasswordStrength::TooShort => "Password is too short".red(),
        PasswordStrength::Weak => "Password is weak".red(),
        PasswordStrength::Medium => "Password is medium".yellow(),
        PasswordStrength::Strong => "Password is strong".green(),
    }
}