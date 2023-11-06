mod minigrep;
use crate::minigrep::{Config, grep_text};

fn main() {
    let handle_args = Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    if let Err(e) = grep_text(&handle_args) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
