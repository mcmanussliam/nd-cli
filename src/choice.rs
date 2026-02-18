use std::{error::Error, fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Quit,
    Back,
    Select(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChoiceParsingError;

impl fmt::Display for ChoiceParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid choice")
    }
}

impl Error for ChoiceParsingError {}

impl FromStr for Choice {
    type Err = ChoiceParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "q" | "Q" => Ok(Choice::Quit),
            "b" | "B" => Ok(Choice::Back),
            other => {
                let n: usize = other.parse().map_err(|_| ChoiceParsingError)?;
                if n == 0 {
                    return Err(ChoiceParsingError);
                }
                Ok(Choice::Select(n))
            }
        }
    }
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Choice::Quit => write!(f, "[q] quit"),
            Choice::Back => write!(f, "[b] back"),
            Choice::Select(choices) => write!(f, "[1..{}] select", choices),
        }
    }
}

pub fn commands_hint(total_choices: usize) -> String {
    if total_choices == 0 {
        format!("Commands: {}, {}", Choice::Back, Choice::Quit)
    } else {
        format!(
            "Commands: {}, {}, {}",
            Choice::Select(total_choices),
            Choice::Back,
            Choice::Quit
        )
    }
}

pub fn out_of_range_message(max_choice: usize) -> String {
    if max_choice == 0 {
        "No directories available. Use [b] or [q].".to_string()
    } else {
        format!("Choose a number from 1 to {max_choice}, or use [b]/[q].")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quit_choice() {
        assert_eq!(Choice::from_str("q"), Ok(Choice::Quit));
        assert_eq!(Choice::from_str("Q"), Ok(Choice::Quit));
    }

    #[test]
    fn parses_back_choice() {
        assert_eq!(Choice::from_str("b"), Ok(Choice::Back));
        assert_eq!(Choice::from_str("B"), Ok(Choice::Back));
    }

    #[test]
    fn parses_numeric_choice() {
        assert_eq!(Choice::from_str("12"), Ok(Choice::Select(12)));
    }

    #[test]
    fn rejects_zero_choice() {
        assert!(Choice::from_str("0").is_err());
    }

    #[test]
    fn builds_command_hint_when_choices_exist() {
        assert_eq!(
            commands_hint(3),
            "Commands: [1..3] select, [b] back, [q] quit"
        );
    }

    #[test]
    fn builds_command_hint_without_choices() {
        assert_eq!(commands_hint(0), "Commands: [b] back, [q] quit");
    }
}
