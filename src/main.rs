use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;
use rayon::prelude::*;

fn process_lines(lines: Vec<String>) -> String {
    let mut output = String::new();
    for line in lines {
        let columns: Vec<&str> = line.split_whitespace().collect();

        // Filter lines with column 4 as "m"
        if let Some(&"m") = columns.get(3) {
            // Extract required columns
            let chromosome = columns.get(0).unwrap();
            let start = columns.get(1).unwrap();
            let end = columns.get(2).unwrap();
            let total_count = columns.get(9).unwrap();
            let meth_count = columns.get(11).unwrap();

            // Append the extracted columns separated by tabs
            output.push_str(&format!("{}\t{}\t{}\t{}\t{}", chromosome, start, end, total_count, meth_count));
            output.push('\n'); // Add a newline here
        }
    }
    output
}

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

    // Process each line and filter them
    let outputs: Vec<String> = reader.lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>() // Collect lines into a Vec<String> to be able to parallelize
        .par_chunks(10000) // Parallel iteration over the chunks of lines
        .flat_map(|chunk| {
            let processed_chunk = process_lines(chunk.to_vec()); // Process each chunk
            vec![processed_chunk]
        })
        .collect();

    // Combine the outputs without extra line breaks
    let combined_output = outputs.join("");

    // Print the combined output after trimming
    println!("{}", combined_output.trim());

    Ok(())
}
