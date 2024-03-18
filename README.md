# bedmeth2bedgraph

This Rust program processes lines from a given bedmethyl input file in parallel and filters them based on certain criteria. It utilizes the Rayon crate for parallel processing.

## Features

- Reads lines from a text file.
- Filters lines based on specific criteria.
- Processes lines in parallel using Rayon.
- Outputs the processed lines.

## Installation

1. Make sure you have Rust and Cargo installed. If not, you can install them from [here](https://www.rust-lang.org/tools/install).

2. Clone this repository:

```sh
git clone https://github.com/sirselim/bedmeth2bedgraph
```

3. Navigate to the project directory:

```sh
cd bedmeth2bedgraph
```

4. Build the project:

```sh
cargo build --release
```

## Usage

Run the compiled binary with the path to the input file as a command-line argument. For example:

```sh
./target/release/bedmethyl2bedgraph input.txt
```

Replace input.txt with the path to your input file.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
