use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() -> std::io::Result<()> {
    // Get the command-line arguments iterator directly
    let mut args = env::args();

    // Skip program name
    args.next();

    // Check if a file name is provided as an argument
    let file_path = match args.next() {
        Some(path) => path,
        None => {
            eprintln!("Usage: {} <input_file>", env::args().next().unwrap());
            process::exit(1);
        }
    };

    // Open the input file
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    // Process each line
    for line in reader.lines() {
        let line = line.unwrap();
        let columns: Vec<&str> = line.split_whitespace().collect();

        // Filter lines with column 4 as "m"
        if columns.get(3) == Some(&"m") {
            // Extract required columns
            let chromosome = columns.get(0).unwrap();
            let start = columns.get(1).unwrap();
            let end = columns.get(2).unwrap();
            let total_count = columns.get(9).unwrap();
            let meth_count = columns.get(11).unwrap();

            // Print the extracted columns separated by tabs
            println!("{}\t{}\t{}\t{}\t{}", chromosome, start, end, total_count, meth_count);
        }
    }

    Ok(())
}

