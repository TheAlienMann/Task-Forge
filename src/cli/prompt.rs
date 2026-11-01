use std::io::{self, Write};

/// Terminal prompter trait for user input and confirmations.
pub trait Prompter {
    fn confirm(&self, question: &str) -> bool;
}

/// Standard console implementation reading from stdin and writing to stdout.
pub struct ConsolePrompter;

impl Prompter for ConsolePrompter {
    fn confirm(&self, question: &str) -> bool {
        print!("{} (y/N): ", question);
        let _ = io::stdout().flush();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            input.trim().eq_ignore_ascii_case("y")
        } else {
            false
        }
    }
}
