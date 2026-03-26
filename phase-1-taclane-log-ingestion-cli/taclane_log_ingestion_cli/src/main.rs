use std::io::{BufRead, BufReader};
use std::fs::File;

// `main` is the program entry point. Returning `Result` lets us use `?`
// and have Rust print any error instead of handling every error manually here.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    // This path points to the sample log file.
    // It is interpreted relative to the directory where the program starts.
    let path = "phase-1-taclane-log-ingestion-cli/sample_logs.log";
    
    // Read the file and print each line.
    // If this fails, `?` returns the error from `main`.
    read_file_contents(path)?;
    Ok(())
}

// This helper opens a file, reads it one line at a time, and prints each line.
fn read_file_contents(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Try to open the file path we were given.
    // If the file cannot be opened, return the error immediately.
    let file = File::open(file_path)?;

    // Wrap the file in a buffered reader so line-by-line reading is efficient.
    let reader = BufReader::new(file);

    // `lines()` returns an iterator where each item is a `Result<String, io::Error>`.
    for line in reader.lines() {
        // Turn the line result into a `String`.
        // If reading one line fails, return that error immediately.
        let line = line?;
        println!("{}", line);
    }

    // If we reached this point, the whole file was processed successfully.
    Ok(())
}
