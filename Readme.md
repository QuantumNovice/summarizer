# Summarizer

Summarizer is a Rust CLI tool that iteratively compresses a given text file by merging adjacent paragraphs using a LLaMA 3-based summarization model. The process repeats until a concise, high-level summary of the entire document is achieved.

## Features
- Accepts a single input file.
- Splits the content into manageable chunks.
- Summarizes and merges adjacent paragraphs using LLaMA 3.
- Repeats the process iteratively until a final summary remains.

## Installation

```sh
cargo build --release
```

## Usage

```sh
target/release/summarizer <file_name>
```

Example:

```sh
target/release/summarizer document.txt
```

## Requirements
- Rust (latest stable version recommended)
- Internet connection (if using an online LLaMA 3 API for summarization)

## How It Works
1. Reads the input file.
2. Divides content into paragraph chunks.
3. Uses LLaMA 3 to summarize and merge adjacent paragraphs.
4. Repeats step 3 until a final concise summary remains.
5. Outputs the final summary.

## License
This project is licensed under the MIT License.
