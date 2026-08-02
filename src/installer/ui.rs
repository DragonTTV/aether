use std::io::{self, Write};
pub fn header() {
    println!();
    println!("═══════════════════════════════════════");
    println!("              Aether Setup");
    println!("═══════════════════════════════════════");
    println!();
}

pub fn step(message: &str) {
    println!("➜ {message}");
}

pub fn success(message: &str) {
    println!("✓ {message}");
}

pub fn warning(message: &str) {
    println!("⚠ {message}");
}

pub fn error(message: &str) {
    println!("✗ {message}");
}

pub fn info(message: &str) {
    println!("{message}");
}

pub fn confirm(prompt: &str) -> Result<bool, String> {
    print!("{prompt} [Y/n]: ");
    io::stdout()
        .flush()
        .map_err(|e| e.to_string())?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    let input = input.trim().to_lowercase();

    Ok(input.is_empty() || input == "y" || input == "yes")
}

pub fn newline() {
    println!();
}