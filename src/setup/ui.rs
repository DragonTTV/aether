#[cfg(unix)]
use std::io::BufRead;
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
    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut reader: Box<dyn BufRead> = match open_tty() {
        Ok(reader) => reader,
        Err(_) => {
            println!("(no terminal detected, defaulting to yes)");
            return Ok(true);
        }
    };

    let mut input = String::new();
    reader.read_line(&mut input).map_err(|e| e.to_string())?;

    let input: String = input.trim().to_lowercase();

    Ok(input.is_empty() || input == "y" || input == "yes")
}

pub fn newline() {
    println!();
}

#[cfg(unix)]
fn open_tty() -> io::Result<Box<dyn BufRead>> {
    use std::fs::OpenOptions;
    let tty = OpenOptions::new().read(true).open("/dev/tty")?;
    Ok(Box::new(io::BufReader::new(tty)))
}

#[cfg(windows)]
fn open_tty() -> io::Result<Box<dyn BufRead>> {
    use std::fs::OpenOptions;
    let con = OpenOptions::new().read(true).open("CONIN$")?;
    Ok(Box::new(io::BufReader::new(con)))
}
