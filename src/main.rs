use std::fs::{self, OpenOptions};
use std::env;
use std::io::Write;
mod reader;
mod parser;

/// Main function, returns ok if everything goes right, or an error if something fails
fn main() -> Result<(), Box<dyn std::error::Error>>{
    // The file should be the second command line argument
    let file_name = env::args().nth(1).ok_or("No input file provided.")?;
    let input_path = format!("{}.lmcl", file_name);
    let output_path = format!("{}.html", file_name);

    // Reading and processing the input file .lmcl
    let code = fs::read_to_string(&input_path)?;
    let code_lines: Vec<String> = reader::store_valid_lines(&code)?;
    let html_code = parser::parse_lines(code_lines)?;

    // Creating and writing the output file .html
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&output_path)?;
    write!(file, "{}", html_code)?;
    Ok(())
}