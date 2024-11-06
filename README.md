# pngit - PNG Message Hiding CLI Tool

**pngit** is a command-line interface (CLI) tool that allows you to hide, decode, remove, and print hidden messages within PNG files by manipulating the PNG file's chunks.

## Features

- **Print**: List all chunk types in the PNG file.
- **Encode**: Hide a message inside a PNG file as a chunk.
- **Decode**: Extract a hidden message from the PNG file.
- **Remove**: Remove a hidden message chunk from the PNG file.

## Installation

To build and run the project, you'll need to have Rust installed. If you don't have Rust yet, install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

### Clone the repository

```bash
git clone <repository-url>
cd <project-directory>
```
## Build the project
```bash
cargo build --release
```

## Run the tool
```bash
cargo run -- <subcommand> <arguments>
```
## Example Commands
Print PNG file chunk types:
```bash
cargo run -- print --filepath example.png
```
Encode a hidden message inside a PNG:
```bash
cargo run -- encode --filepath example.png --chunk-type text --message "Hidden secret!"
```
Decode and extract a hidden message from a PNG:
```bash
cargo run -- decode --filepath example.png --chunk-type text
```
Remove a hidden message chunk from a PNG:
```bash
cargo run -- remove --filepath example.png --chunk-type text
```
